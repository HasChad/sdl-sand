use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump,
};
use std::{thread::sleep, time::Duration};

mod cell_updates;
pub mod cells;

use cell_updates::*;
use cells::{Cell, CellState};

const GRID_X_SIZE: usize = 300;
const GRID_Y_SIZE: usize = 160;
const DOT_SIZE_IN_PXS: usize = 4;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?; //SDL2 init
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem //Creating window
        .window(
            "SDL-Sand",
            (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32,
            (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;

    //Game things
    let mut cells = vec![Cell::spawn_empty(); GRID_X_SIZE * GRID_Y_SIZE + GRID_X_SIZE + 1];
    let mut brush = Cell::spawn_sand();

    //Game Loop
    'running: loop {
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
        canvas.set_draw_color(Color::RGB(10, 10, 10));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } //Quit app when pressed close button
                | Event::KeyDown { //Quit app when pressed escape button 
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        update_dropper(&mut cells, &mut brush, &mut event_pump);
        update_world(&mut cells);
        draw_world(&mut cells, &mut canvas);

        canvas.present();
    }

    Ok(())
}

fn update_dropper(cells: &mut [Cell], brush: &mut Cell, event_pump: &mut EventPump) {
    //Change Brush
    for input in event_pump.keyboard_state().pressed_scancodes() {
        match input {
            Scancode::Num1 => *brush = Cell::spawn_sand(),
            Scancode::Num2 => *brush = Cell::spawn_water(),
            _ => (),
        }
    }

    //Mouse Click Spawn
    let mouse_xpos = event_pump.mouse_state().x() / DOT_SIZE_IN_PXS as i32;
    let mouse_ypos = event_pump.mouse_state().y() / DOT_SIZE_IN_PXS as i32;
    let pixel_pos = (mouse_xpos + (mouse_ypos * GRID_X_SIZE as i32)) as usize;

    if mouse_xpos >= 0
        && mouse_xpos < (GRID_X_SIZE as i32)
        && mouse_ypos >= 0
        && mouse_ypos < (GRID_Y_SIZE as i32)
        && event_pump.mouse_state().left()
        && cells[pixel_pos] == Cell::spawn_empty()
    {
        cells[pixel_pos] = *brush;

        //top
        cells[pixel_pos - 2 * GRID_X_SIZE] = *brush;
        cells[pixel_pos - GRID_X_SIZE] = *brush;
        cells[pixel_pos - GRID_X_SIZE - 1] = *brush;
        cells[pixel_pos - GRID_X_SIZE + 1] = *brush;

        //middle
        cells[pixel_pos - 2] = *brush;
        cells[pixel_pos - 1] = *brush;
        cells[pixel_pos + 1] = *brush;
        cells[pixel_pos + 2] = *brush;

        //bottom
        cells[pixel_pos + 2 * GRID_X_SIZE] = *brush;
        cells[pixel_pos + GRID_X_SIZE] = *brush;
        cells[pixel_pos + GRID_X_SIZE - 1] = *brush;
        cells[pixel_pos + GRID_X_SIZE + 1] = *brush;
    }
}

fn update_world(cells: &mut [Cell]) {
    //Pixel iterate
    for y in (0..GRID_Y_SIZE).rev() {
        for x in 0..GRID_X_SIZE {
            let pixel_pos: usize = (y * GRID_X_SIZE) + x;

            match cells[pixel_pos].state {
                CellState::Sand => update_sand(x, y, cells),
                CellState::Water => update_water(x, y, cells),
                _ => (),
            }
        }
    }
}

fn draw_world(cells: &mut [Cell], canvas: &mut Canvas<Window>) {
    //Per-pixel coloring
    for (i, cell) in cells.iter_mut().enumerate() {
        if cell.state != CellState::Dead {
            canvas.set_draw_color(cell.color);

            cell.is_moved = false;

            canvas
                .fill_rect(Rect::new(
                    ((i % GRID_X_SIZE) * DOT_SIZE_IN_PXS) as i32,
                    ((i / GRID_X_SIZE) * DOT_SIZE_IN_PXS) as i32,
                    DOT_SIZE_IN_PXS as u32,
                    DOT_SIZE_IN_PXS as u32,
                ))
                .ok()
                .unwrap_or_default();
        }
    }
}
