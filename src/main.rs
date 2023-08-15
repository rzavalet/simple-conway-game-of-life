extern crate sdl2;

use rand::Rng;
use std::thread;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;

const FACTOR: usize = 100;
const WIDTH: usize = 9 * FACTOR;
const HEIGHT: usize = 9 * FACTOR;
const SIZE: usize = 50;
const RECT_WIDTH: usize = WIDTH / SIZE;
const RECT_HEIGHT: usize = HEIGHT / SIZE;

const NORMAL_SPEED: Duration = Duration::from_secs(1);
const FAST_SPEED: Duration   = Duration::from_millis(50);

fn do_step(array: &mut [[[i32;SIZE];SIZE];2], cur_matrix: usize) 
{
    let mut counter;
    for i in 0..SIZE {
        for j in 0..SIZE {
            // Assume the world is "modular", i.e. the border of one side is connected to the
            // border on the other side.
            counter =   array[cur_matrix][(i + SIZE - 1) % SIZE][(j + SIZE - 1) % SIZE] +
                        array[cur_matrix][(i + SIZE - 1) % SIZE][(j + SIZE + 0) % SIZE] +
                        array[cur_matrix][(i + SIZE - 1) % SIZE][(j + SIZE + 1) % SIZE] +

                        array[cur_matrix][i] [(j + SIZE - 1) % SIZE] +
                        array[cur_matrix][i] [(j + SIZE + 1) % SIZE] +

                        array[cur_matrix][(i + SIZE + 1) % SIZE][(j + SIZE - 1) % SIZE] +
                        array[cur_matrix][(i + SIZE + 1) % SIZE][(j + SIZE + 0) % SIZE] +
                        array[cur_matrix][(i + SIZE + 1) % SIZE][(j + SIZE + 1) % SIZE];


            array[1 - cur_matrix][i][j] = match array[cur_matrix][i][j] {
                1 => {
                    match counter {
                        ..= 1 => 0,
                        2 | 3 => 1,
                        _     => 0,
                    }
                },
                _ => {
                    match counter {
                        3 => 1,
                        _ => 0,
                    }
                }
            }
        }
    }
}

fn init_world(array: &mut [[[i32; SIZE];SIZE];2], cur_matrix: usize, rng: &mut rand::rngs::ThreadRng, clear: bool)
{
    for i in 0..SIZE {
        for j in 0..SIZE {
            array[cur_matrix][i][j] = match clear {
                true => 0,
                false => {
                    match rng.gen_range(0..=100) {
                        0..=85 => 0,
                        _      => 1
                    }
                }
            };
        }
    }
}

fn render_step(array: &[[[i32; SIZE];SIZE];2], cur_matrix: usize, canvas: &mut WindowCanvas)
{
    canvas.set_draw_color(Color::RGB(41, 41, 41));
    canvas.clear();

    for i in 0..SIZE {
        for j in 0.. SIZE {
            let x: usize = i * RECT_WIDTH;
            let y: usize = j * RECT_HEIGHT;
            let color: Color  = match array[cur_matrix][i][j] {
                0 => Color::RGB(255, 255, 255),
                _ => Color::RGB(0, 0, 0),
            };

            canvas.set_draw_color(color);
            let _ = canvas.fill_rect(Rect::new(x as i32 + 1, y as i32 + 1, RECT_WIDTH as u32 - 1, RECT_HEIGHT as u32 - 1));
        }
    }
    canvas.present();
}


fn main() 
{
    
    let mut rng = rand::thread_rng();
    let mut cur_matrix = 0;
    let mut array: [[[i32;SIZE];SIZE];2] = [[[0;SIZE];SIZE];2];
    let mut paused: bool = false;
    let mut clicked: bool = false;
    let mut current_rect_x: usize = std::usize::MAX;
    let mut current_rect_y: usize = std::usize::MAX;
    let mut speed = NORMAL_SPEED;


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of Life in Rust", 
                                        WIDTH as u32, HEIGHT as u32)
                                .position_centered()
                                .build()
                                .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    init_world(&mut array, cur_matrix, &mut rng, false);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running 
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    paused = !paused;
                },
                Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                    speed = FAST_SPEED;
                },
                Event::KeyUp { keycode: Some(Keycode::Return), ..} => {
                    speed = NORMAL_SPEED;
                },

                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    if paused {
                        init_world(&mut array, cur_matrix, &mut rng, true);
                    }
                },

                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if paused {
                        init_world(&mut array, cur_matrix, &mut rng, false);
                    }
                },

                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, ..} => {

                    if !paused { break; }

                    clicked = true;
                    let i = x as usize / RECT_WIDTH;
                    let j = y as usize / RECT_HEIGHT;
                    array[cur_matrix][i][j] = 1 - array[cur_matrix][i][j];

                    current_rect_x = i;
                    current_rect_y = j;
                }

                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    clicked = false;
                    current_rect_x = std::usize::MAX;
                    current_rect_y = std::usize::MAX;
                }

                Event::MouseMotion {x, y, ..} => {
                    if clicked {
                        let i = x as usize / RECT_WIDTH;
                        let j = y as usize / RECT_HEIGHT;
                        if (current_rect_x != i) || (current_rect_y != j) {
                            array[cur_matrix][i][j] = 1 - array[cur_matrix][i][j];

                            current_rect_x = i;
                            current_rect_y = j;
                        }
                    }
                }

                _ => {}
            }
        } // End event loop
        
        render_step(&array, cur_matrix, &mut canvas);
        if paused {
            continue;
        }

        do_step(&mut array, cur_matrix);

        cur_matrix = 1 - cur_matrix;
        thread::sleep(speed);

    } // End game loop

}
