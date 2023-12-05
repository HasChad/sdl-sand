use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const GRID_X_SIZE: usize = 160;
const GRID_Y_SIZE: usize = 120;
const DOT_SIZE_IN_PXS: usize = 5;

#[derive(Copy, Clone, Debug, PartialEq)]
enum CellState {
    Dead,
    Sand,
    //Water,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?; //SDL2 init
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem //Creating window
        .window(
            "sdl-sand",
            (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32,
            (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0)); //black background
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;

    //Game things
    let mut buffer = vec![CellState::Dead; GRID_X_SIZE * GRID_Y_SIZE];
    let mut cells = vec![CellState::Dead; GRID_X_SIZE * GRID_Y_SIZE];

    'running: loop {
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
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        //Main game loop
        if event_pump.mouse_state().left() {
            let mouse_xpos = event_pump.mouse_state().x() / 5;
            let mouse_ypos = event_pump.mouse_state().y() / 5;

            let buffer_pos = (mouse_xpos + (mouse_ypos * GRID_X_SIZE as i32)) as usize;

            buffer[buffer_pos] = CellState::Sand;
        }

        println!(
            "x: {}, y: {}",
            event_pump.mouse_state().x() / 5,
            event_pump.mouse_state().y() / 5
        );

        //Pixel iterate
        for y in (0..GRID_Y_SIZE).rev() {
            for x in 0..GRID_X_SIZE {
                let pixel_pos: usize = (y * GRID_X_SIZE) + x;
                let down: usize = pixel_pos + GRID_X_SIZE;
                let down_left: usize = down - 1;
                let down_right: usize = down + 1;

                match buffer[pixel_pos] {
                    CellState::Dead => continue,
                    CellState::Sand => {
                        if down < GRID_X_SIZE * GRID_Y_SIZE {
                            //Down
                            if buffer[down] == CellState::Dead {
                                buffer[down] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;

                            //Down left
                            } else if x != 0 && buffer[down_left] == CellState::Dead {
                                buffer[down_left] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;

                            //Down right
                            } else if x != 160 && buffer[down_right] == CellState::Dead {
                                buffer[down_right] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;
                            }
                        }
                    }
                }
            }
        }

        //Per-pixel coloring
        for i in 0..buffer.len() {
            //cells[i] = buffer[i];

            match buffer[i] {
                CellState::Dead => canvas.set_draw_color(Color::BLACK),
                CellState::Sand => canvas.set_draw_color(Color::YELLOW),
                //CellState::Water => BLUE,
            }

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

        canvas.present();
    }

    Ok(())
}
