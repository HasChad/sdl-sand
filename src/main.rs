use color_eyre::eyre::Result;
use macroquad::prelude::*;

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;

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

    let width = SCREEN_WIDTH;
    let height = SCREEN_HEIGHT;

    let mut buffer = vec![CellState::Dead; width * height];

    let mut _current_item = CellState::Sand;

    let mut image = Image::gen_image_color(SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16, WHITE);

    let texture = Texture2D::from_image(&image);

    //Simulation Loop
    loop {
        clear_background(BLACK);

        //Mouse item changer
        if is_key_pressed(KeyCode::Key2) {
            _current_item = CellState::Water;
        } else if is_key_pressed(KeyCode::Key1) {
            _current_item = CellState::Sand;
        }

        //Mouse input
        if is_mouse_button_down(MouseButton::Left) {
            let (mut _mouse_posx, mut _mouse_posy) = mouse_position();
            let mut _mousepos: usize = (_mouse_posy * width as f32 + _mouse_posx) as usize;

            buffer[_mousepos - width] = _current_item;
            buffer[_mousepos - width - 1] = _current_item;
            buffer[_mousepos - width + 1] = _current_item;
            buffer[_mousepos + width] = _current_item;
            buffer[_mousepos + width - 1] = _current_item;
            buffer[_mousepos + width + 1] = _current_item;
            buffer[_mousepos] = _current_item;
            buffer[_mousepos - 1] = _current_item;
            buffer[_mousepos + 1] = _current_item;
        }

        //Pixel iterate
        for y in (0..height).rev() {
            for x in 0..width {
                //info!("x = {}, y = {}", x, y);

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
                    CellState::Dead => continue,
                    CellState::Sand => {
                        if down < width * height {
                            //Down
                            if buffer[down] == CellState::Dead {
                                buffer[down] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;

                            //Down left
                            } else if buffer[down_left] == CellState::Dead && x > 0 {
                                buffer[down_left] = CellState::Sand;
                                buffer[pixel_pos] = CellState::Dead;

                            //Down right
                            } else if buffer[down_right] == CellState::Dead && x < width - 1 {
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
                        //Down
                        if buffer[down] == CellState::Dead && down < width * height {
                            buffer[down] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;

                        //Down left
                        } else if buffer[down_left] == CellState::Dead && down < width * height {
                            buffer[down_left] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;

                        //Down right
                        } else if buffer[down_right] == CellState::Dead && down < width * height {
                            buffer[down_right] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;

                        //Left
                        } else if buffer[left] == CellState::Dead {
                            info!("succes left");
                            buffer[left] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;

                        //Right
                        } else if buffer[right] == CellState::Dead {
                            info!("success right");
                            buffer[right] = CellState::Water;
                            buffer[pixel_pos] = CellState::Dead;
                        }
                    }
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
