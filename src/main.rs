mod cells;

use cells::{Cell, CellState, Direction};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};
use std::{thread::sleep, time::Duration};

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

        update_dropper(&mut cells, &mut brush, &event_pump);
        update_world(&mut cells);
        draw_world(&mut cells, &mut canvas);

        canvas.present();
    }

    Ok(())
}

fn update_dropper(cells: &mut [Cell], brush: &mut Cell, event_pump: &EventPump) {
    let mouse_xpos = event_pump.mouse_state().x() / DOT_SIZE_IN_PXS as i32;
    let mouse_ypos = event_pump.mouse_state().y() / DOT_SIZE_IN_PXS as i32;
    let pixel_pos = (mouse_xpos + (mouse_ypos * GRID_X_SIZE as i32)) as usize;

    //Mouse Click Spawn
    if mouse_xpos >= 0
        && mouse_xpos < (GRID_X_SIZE as i32)
        && mouse_ypos >= 0
        && mouse_ypos < (GRID_Y_SIZE as i32)
    {
        if event_pump.mouse_state().right() {
            *brush = Cell::spawn_water();
        }

        if event_pump.mouse_state().left() || event_pump.mouse_state().right() {
            cells[pixel_pos] = *brush;
            cells[pixel_pos + 1] = *brush;
            cells[pixel_pos - 1] = *brush;
            cells[pixel_pos + GRID_X_SIZE] = *brush;
            cells[pixel_pos - GRID_X_SIZE] = *brush;
        }
    }
}

fn update_world(cells: &mut [Cell]) {
    //Pixel iterate
    for y in (0..GRID_Y_SIZE).rev() {
        for x in 0..GRID_X_SIZE {
            let pixel_pos: usize = (y * GRID_X_SIZE) + x;
            let down: usize = pixel_pos + GRID_X_SIZE;
            let down_left: usize = down - 1;
            let down_right: usize = down + 1;

            match cells[pixel_pos].state {
                CellState::Dead => continue,
                CellState::Sand => {
                    //Down-Side checker
                    let downleft_is_empty = cells[down_left] == Cell::spawn_empty();
                    let downright_is_empty = cells[down_right] == Cell::spawn_empty();

                    if y != GRID_Y_SIZE - 1 {
                        //Down
                        if cells[down] == Cell::spawn_empty() {
                            cells[down] = Cell::spawn_sand();
                            cells[pixel_pos] = Cell::spawn_empty();
                        //Down water
                        } else if cells[down].state == CellState::Water {
                            cells[down] = Cell::spawn_sand();
                            cells[pixel_pos] = Cell::spawn_water();
                        //Down left
                        } else if x != 0 && downleft_is_empty {
                            cells[down_left] = Cell::spawn_sand();
                            cells[pixel_pos] = Cell::spawn_empty();
                        //Down right
                        } else if x != GRID_X_SIZE - 1 && downright_is_empty {
                            cells[down_right] = Cell::spawn_sand();
                            cells[pixel_pos] = Cell::spawn_empty();
                        }
                    }
                }
                CellState::Water => {
                    //Down-Side checker
                    let downleft_is_empty = cells[down_left] == Cell::spawn_empty();
                    let downright_is_empty = cells[down_right] == Cell::spawn_empty();
                    //Side checker
                    let left_is_empty = cells[pixel_pos - 1] == Cell::spawn_empty();
                    let right_is_empty = cells[pixel_pos + 1] == Cell::spawn_empty();

                    if y != GRID_Y_SIZE - 1 {
                        //Down
                        if cells[down] == Cell::spawn_empty() {
                            cells[down] = Cell::spawn_water();
                            cells[pixel_pos] = Cell::spawn_empty();

                        //Down left
                        } else if x != 0 && downleft_is_empty {
                            cells[down_left] = Cell::spawn_water();
                            cells[pixel_pos] = Cell::spawn_empty();

                        //Down right
                        } else if x != GRID_X_SIZE - 1 && downright_is_empty {
                            cells[down_right] = Cell::spawn_water();
                            cells[pixel_pos] = Cell::spawn_empty();
                        //Left
                        } else if x != 0
                            && left_is_empty
                            && cells[pixel_pos].move_direction == Direction::Left
                            && !cells[pixel_pos].is_moved
                        {
                            cells[pixel_pos - 1] = Cell::spawn_water();
                            cells[pixel_pos - 1].move_direction = Direction::Left;
                            cells[pixel_pos] = Cell::spawn_empty();
                        //Right
                        } else if x != GRID_X_SIZE - 1
                            && right_is_empty
                            && cells[pixel_pos].move_direction == Direction::Right
                            && !cells[pixel_pos].is_moved
                        {
                            cells[pixel_pos + 1] = Cell::spawn_water();
                            cells[pixel_pos + 1].move_direction = Direction::Right;
                            cells[pixel_pos] = Cell::spawn_empty();
                        } else {
                            match cells[pixel_pos].move_direction {
                                Direction::Left => {
                                    cells[pixel_pos].move_direction = Direction::Right
                                }
                                Direction::Right => {
                                    cells[pixel_pos].move_direction = Direction::Left
                                }
                                Direction::None => (),
                            }
                        }
                    }
                }
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
