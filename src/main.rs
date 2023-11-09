use color_eyre::eyre::Result;
use macroquad::prelude::*;

const SCREEN_WIDTH: u16 = 1280;
const SCREEN_HEIGHT: u16 = 720;

#[derive(Copy, Clone, Debug, PartialEq)]
enum CellState {
    Dead,
    Sand,
    Water,
}

#[macroquad::main("Sandbox")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    request_new_screen_size(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let width: usize = (SCREEN_WIDTH) as usize;
    let height: usize = (SCREEN_HEIGHT) as usize;

    let mut buffer = vec![CellState::Dead; width * height];

    //let mut xpos = vec![CellState::Dead; width];
    //let mut ypos = vec![xpos; height];

    let mut image = Image::gen_image_color(SCREEN_WIDTH, SCREEN_HEIGHT, WHITE);

    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);

        //Mouse input
        if is_mouse_button_down(MouseButton::Left) {
            let (mut _mouse_posx, mut _mouse_posy) = mouse_position();
            let mut _mousepos: usize = (_mouse_posy * width as f32 + _mouse_posx) as usize;

            buffer[_mousepos - width] = CellState::Sand;
            buffer[_mousepos - width - 1] = CellState::Sand;
            buffer[_mousepos - width + 1] = CellState::Sand;
            buffer[_mousepos + width] = CellState::Sand;
            buffer[_mousepos + width - 1] = CellState::Sand;
            buffer[_mousepos + width + 1] = CellState::Sand;
            buffer[_mousepos] = CellState::Sand;
            buffer[_mousepos - 1] = CellState::Sand;
            buffer[_mousepos + 1] = CellState::Sand;
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (mut _mouse_posx, mut _mouse_posy) = mouse_position();
            let mut _mousepos: usize = (_mouse_posy * width as f32 + _mouse_posx) as usize;

            buffer[_mousepos - width] = CellState::Water;
            buffer[_mousepos - width - 1] = CellState::Water;
            buffer[_mousepos - width + 1] = CellState::Water;
            buffer[_mousepos + width] = CellState::Water;
            buffer[_mousepos + width - 1] = CellState::Water;
            buffer[_mousepos + width + 1] = CellState::Water;
            buffer[_mousepos] = CellState::Water;
            buffer[_mousepos - 1] = CellState::Water;
            buffer[_mousepos + 1] = CellState::Water;
        }

        //Pixel iterate
        for y in (0..height).rev() {
            for x in 0..width {
                if x == width - 1 || x == 0 {
                    continue;
                }
                let pixel_pos: usize = (y * width) + x;
                let left: usize = pixel_pos - 1;
                let right: usize = pixel_pos + 1;
                let down: usize = pixel_pos + width;
                let down_left: usize = down - 1;
                let down_right: usize = down + 1;

                match buffer[pixel_pos] {
                    CellState::Sand => {
                        if y != height - 1 {
                            //Down
                            if buffer[down] == CellState::Dead {
                                buffer[down] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;
                            //Down left
                            } else if buffer[down_left] == CellState::Dead {
                                buffer[down_left] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;
                            //Down right
                            } else if buffer[down_right] == CellState::Dead {
                                buffer[down_right] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;
                                //Water collision
                            } else if buffer[down] == CellState::Water {
                                buffer[down] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Water;
                            }
                        }
                    }

                    CellState::Water => {
                        let control = y != height - 2;

                        //Down
                        if buffer[down] == CellState::Dead && control {
                            buffer[down] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;
                        //Down left
                        } else if buffer[down_left] == CellState::Dead && control {
                            buffer[down_left] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;
                        //Down right
                        } else if buffer[down_right] == CellState::Dead && control {
                            buffer[down_right] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;
                        //Left
                        } else if buffer[left] == CellState::Dead {
                            buffer[left] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;
                        //Right
                        } else if buffer[right] == CellState::Dead {
                            buffer[right] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;
                        }
                    }
                    _ => (),
                }
            }
        }

        //Per-pixel coloring
        for (i, _) in buffer.iter().enumerate() {
            image.set_pixel(
                (i % width) as u32,
                (i / width) as u32,
                match buffer[i] {
                    CellState::Dead => BLACK,
                    CellState::Water => BLUE,
                    CellState::Sand => ORANGE,
                },
            );
        }

        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await
    }
}
