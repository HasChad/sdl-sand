use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};

mod cell_updates;
pub mod cells;

use cell_updates::*;
use cells::{Cell, CellState};

const GRID_X_SIZE: usize = 300;
const GRID_Y_SIZE: usize = 160;
const DOT_SIZE_IN_PXS: usize = 4;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macro-Sand".into(),
        window_width: (GRID_X_SIZE * DOT_SIZE_IN_PXS) as i32,
        window_height: (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() -> Result<(), String> {
    // Rendering things
    let mut image = Image::gen_image_color(
        GRID_X_SIZE as u16,
        GRID_Y_SIZE as u16,
        Color::from_rgba(10, 10, 10, 255),
    );
    let texture = Texture2D::from_image(&image);
    let texture_param = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: screen_width(),
            y: screen_height(),
        }),
        source: None,
        rotation: 0.,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };

    // Game things
    let mut cells = vec![Cell::spawn_empty(); GRID_X_SIZE * GRID_Y_SIZE];
    let mut brush = Cell::spawn_sand();

    // Game Loop
    'running: loop {
        if is_key_pressed(KeyCode::Escape) {
            break 'running;
        }

        update_dropper(&mut cells, &mut brush).await;
        update_world(&mut cells).await;
        draw_world(&mut cells, &mut image, &texture, &texture_param).await;

        next_frame().await
    }

    Ok(())
}

async fn update_dropper(cells: &mut [Cell], brush: &mut Cell) {
    let ui_windows_size = Vec2::new(150., 200.);
    let ui_windows_pos = Vec2::new(25., 25.);

    widgets::Window::new(hash!(), ui_windows_pos, ui_windows_size)
        .label("User Window")
        .ui(&mut root_ui(), |ui| {
            ui.tree_node(hash!(), "Brushes", |ui| {
                if ui.button(Vec2::new(10., 25.), "Sand") {
                    *brush = Cell::spawn_sand()
                }
                if ui.button(Vec2::new(10., 50.), "Water") {
                    *brush = Cell::spawn_water()
                }
                if ui.button(Vec2::new(10., 75.), "Stone") {
                    *brush = Cell::spawn_stone()
                }
            });
        });

    //Change Brush
    if let Some(input) = get_last_key_pressed() {
        match input {
            KeyCode::Key1 => *brush = Cell::spawn_sand(),
            KeyCode::Key2 => *brush = Cell::spawn_water(),
            KeyCode::Key3 => *brush = Cell::spawn_stone(),
            KeyCode::C => {
                for cell in cells.iter_mut() {
                    *cell = Cell::spawn_empty();
                }
            }
            //TODO: add button to clear canvas
            _ => (),
        }
    }

    //Mouse Click Spawn
    let (mouse_xpos, mouse_ypos) = mouse_position();
    let pixel_posx = mouse_xpos as usize / DOT_SIZE_IN_PXS;
    let pixel_posy = mouse_ypos as usize / DOT_SIZE_IN_PXS;
    let pixel_pos = pixel_posx + (pixel_posy * GRID_X_SIZE);

    if mouse_xpos >= 0.
        && mouse_xpos < screen_width()
        && mouse_ypos >= 0.
        && mouse_ypos < screen_height()
        && !((mouse_xpos > ui_windows_pos.x)
            && (mouse_xpos < ui_windows_pos.x + ui_windows_size.x)
            && (mouse_ypos > ui_windows_pos.y)
            && (mouse_ypos < ui_windows_pos.y + ui_windows_size.y))
    {
        if is_mouse_button_down(MouseButton::Left) && cells[pixel_pos] == Cell::spawn_empty() {
            cells[pixel_pos] = *brush;

            //top
            /* cells[pixel_pos - 2 * GRID_X_SIZE - 2] = *brush;
            cells[pixel_pos - 2 * GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos - 2 * GRID_X_SIZE] = *brush;
            cells[pixel_pos - 2 * GRID_X_SIZE + 1] = *brush;
            cells[pixel_pos - 2 * GRID_X_SIZE + 2] = *brush; */

            //cells[pixel_pos - GRID_X_SIZE - 2] = *brush;
            cells[pixel_pos - GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos - GRID_X_SIZE] = *brush;
            cells[pixel_pos - GRID_X_SIZE + 1] = *brush;
            //cells[pixel_pos - GRID_X_SIZE + 2] = *brush;

            //middle
            cells[pixel_pos - 2] = *brush;
            cells[pixel_pos - 1] = *brush;
            cells[pixel_pos + 1] = *brush;
            cells[pixel_pos + 2] = *brush;

            //bottom
            //cells[pixel_pos + GRID_X_SIZE - 2] = *brush;
            cells[pixel_pos + GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos + GRID_X_SIZE] = *brush;
            cells[pixel_pos + GRID_X_SIZE + 1] = *brush;
            //cells[pixel_pos + GRID_X_SIZE + 2] = *brush;

            /* cells[pixel_pos + 2 * GRID_X_SIZE - 2] = *brush;
            cells[pixel_pos + 2 * GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos + 2 * GRID_X_SIZE] = *brush;
            cells[pixel_pos + 2 * GRID_X_SIZE + 1] = *brush;
            cells[pixel_pos + 2 * GRID_X_SIZE + 2] = *brush; */
        }

        if is_mouse_button_down(MouseButton::Right) {
            cells[pixel_pos] = Cell::spawn_empty();

            //top
            cells[pixel_pos - GRID_X_SIZE - 1] = Cell::spawn_empty();
            cells[pixel_pos - GRID_X_SIZE] = Cell::spawn_empty();
            cells[pixel_pos - GRID_X_SIZE + 1] = Cell::spawn_empty();

            //middle
            cells[pixel_pos - 2] = Cell::spawn_empty();
            cells[pixel_pos - 1] = Cell::spawn_empty();
            cells[pixel_pos + 1] = Cell::spawn_empty();
            cells[pixel_pos + 2] = Cell::spawn_empty();

            //bottom
            cells[pixel_pos + GRID_X_SIZE - 1] = Cell::spawn_empty();
            cells[pixel_pos + GRID_X_SIZE] = Cell::spawn_empty();
            cells[pixel_pos + GRID_X_SIZE + 1] = Cell::spawn_empty();
        }
    }
}

async fn update_world(cells: &mut [Cell]) {
    // Pixel iterate
    for y in (0..GRID_Y_SIZE).rev() {
        for x in 0..GRID_X_SIZE {
            let pixel_pos: usize = (y * GRID_X_SIZE) + x;

            match cells[pixel_pos].state {
                CellState::Sand => update_sand(x, y, cells).await,
                CellState::Water => update_water(x, y, cells).await,
                _ => (),
            }
        }
    }
}

async fn draw_world(
    cells: &mut [Cell],
    image: &mut Image,
    texture: &Texture2D,
    texture_param: &DrawTextureParams,
) {
    // Per-pixel coloring
    for (i, cell) in cells.iter_mut().enumerate() {
        image.set_pixel(
            (i % GRID_X_SIZE) as u32,
            (i / GRID_X_SIZE) as u32,
            cell.color,
        );
    }

    // Cursor
    let (mouse_xpos, mouse_ypos) = mouse_position();
    let pixel_posx = mouse_xpos / DOT_SIZE_IN_PXS as f32;
    let pixel_posy = mouse_ypos / DOT_SIZE_IN_PXS as f32;

    if mouse_xpos >= 0.
        && mouse_xpos < screen_width()
        && mouse_ypos >= 0.
        && mouse_ypos < screen_height()
    {
        // left
        if mouse_xpos > DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx as u32 - 1, pixel_posy as u32, WHITE);
        }
        // right
        if mouse_xpos < screen_width() - DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx as u32 + 1, pixel_posy as u32, WHITE);
        }
        // top
        if mouse_ypos > DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx as u32, pixel_posy as u32 - 1, WHITE);
        }
        // bottom
        if mouse_ypos < screen_height() - DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx as u32, pixel_posy as u32 + 1, WHITE);
        }
    }

    // Rendering
    texture.update(image);
    texture.set_filter(FilterMode::Nearest);
    draw_texture_ex(texture, 0., 0., WHITE, texture_param.clone());
}
