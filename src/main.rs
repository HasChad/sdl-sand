extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;

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
        .window("sdl-sand", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0)); //black background
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    //Game things
    let mut buffer = vec![CellState::Dead; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut cells = vec![CellState::Dead; SCREEN_WIDTH * SCREEN_HEIGHT];

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

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::YELLOW);
        canvas
            .draw_point(Point::new(10, 10))
            .ok()
            .unwrap_or_default();

        for y in (0..SCREEN_HEIGHT).rev() {
            for x in 0..SCREEN_WIDTH {
                canvas
                    .draw_point(Point::new(x as i32, y as i32))
                    .ok()
                    .unwrap_or_default();
            }
        }

        canvas.present();
    }

    Ok(())
}
