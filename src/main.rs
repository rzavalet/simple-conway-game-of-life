extern crate sdl2;

use rand::Rng;
use std::thread;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

const FACTOR: u32 = 100;
const WIDTH: u32 = 9 * FACTOR;
const HEIGHT: u32 = 9 * FACTOR;
const SIZE: usize = 50;

fn do_step(array: &mut [[[i32;SIZE];SIZE];2], cur_matrix: usize) {
    let mut counter;
    for i in 0..SIZE {
        for j in 0..SIZE {
            // Assume the worl is "modular", i.e. the border of one side are connected to the
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

fn init_world(array: &mut [[[i32; SIZE];SIZE];2], rng: &mut rand::rngs::ThreadRng) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            array[0][i][j] = match rng.gen_range(0..=100) {
                0..=85 => 0,
                _      => 1
            };
        }
    }
}


fn main() {
    
    let mut rng = rand::thread_rng();
    let mut cur_matrix = 0;
    let mut array: [[[i32;SIZE];SIZE];2] = [[[0;SIZE];SIZE];2];
    let mut paused: bool = false;
    init_world(&mut array, &mut rng);


    let duration = Duration::from_secs(1);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 
                                        WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running 
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    paused = !paused;
                },

                _ => {}
            }
        } // End event loop
        
        if paused {
            continue;
        }

        do_step(&mut array, cur_matrix);
        canvas.set_draw_color(Color::RGB(41, 41, 41));
        canvas.clear();

        let rect_width = WIDTH / SIZE as u32;
        let rect_height = HEIGHT / SIZE as u32;
        for i in 0..SIZE {
            for j in 0.. SIZE {
                let x: i32 = i as i32 * rect_width as i32;
                let y: i32 = j as i32 * rect_height as i32;
                let color: Color  = match array[cur_matrix][i][j] {
                    0 => Color::RGB(255, 255, 255),
                    _ => Color::RGB(0, 0, 0),
                };

                canvas.set_draw_color(color);
                let _ = canvas.fill_rect(Rect::new(x + 1, y + 1, rect_width - 1, rect_height - 1));
            }
        }
        canvas.present();

        cur_matrix = 1 - cur_matrix;
        thread::sleep(duration);

    } // End game loop

}
