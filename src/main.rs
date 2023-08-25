extern crate sdl2;

use rand::Rng;
use std::thread;
use std::time::Duration;

use clap::Parser;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;


const DEFAULT_BOARD_WIDTH:  usize = 50;
const DEFAULT_BOARD_HEIGHT: usize = 50;

const CANVAS_WIDTH_PX: usize = 600;
const CANVAS_HEIGHT_PX: usize = 600;

const RECT_WIDTH_PX:  usize = CANVAS_WIDTH_PX  / DEFAULT_BOARD_WIDTH;
const RECT_HEIGHT_PX: usize = CANVAS_HEIGHT_PX / DEFAULT_BOARD_HEIGHT;

const NORMAL_SPEED: Duration = Duration::from_secs(1);
const FAST_SPEED:   Duration = Duration::from_millis(50);


mod colors {
    use sdl2::pixels::Color;
    pub const GRID:          Color = Color::RGB(0x93, 0xA1, 0xA1);
    pub const FILLED_SQUARE: Color = Color::RGB(0x2A, 0xA1, 0x98);
    pub const EMPTY_SQUARE:  Color = Color::RGB(0xFD, 0xF6, 0xE3);
}


// Configure command line parsing:
#[derive(Parser)]
struct Cli {

    /// Number of columns in the board.
    #[arg(long, default_value_t = DEFAULT_BOARD_WIDTH)]
    board_width: usize,

    /// Number of rows in the board.
    #[arg(long, default_value_t = DEFAULT_BOARD_HEIGHT)]
    board_height: usize,
}


// FIXME: The following two declarations allowed accessing `ModularMatrix`;
// however this seems redundant:
pub mod modular_matrix;
use crate::modular_matrix::ModularMatrix;


fn do_step(cur_matrix: &ModularMatrix<bool>, new_matrix: &mut ModularMatrix<bool>) 
{
    for i in 0 .. cur_matrix.width() {
        let col = i as isize;

        for j in 0 .. cur_matrix.height() {
            let row = j as isize;

            let neighbor_count = 0
                + if cur_matrix.get(col-1, row-1) {1} else {0} 
                + if cur_matrix.get(col-1, row+0) {1} else {0} 
                + if cur_matrix.get(col-1, row+1) {1} else {0} 
                //
                + if cur_matrix.get(col+0, row-1) {1} else {0} 
                + if cur_matrix.get(col+0, row+1) {1} else {0} 
                //
                + if cur_matrix.get(col+1, row-1) {1} else {0} 
                + if cur_matrix.get(col+1, row+0) {1} else {0} 
                + if cur_matrix.get(col+1, row+1) {1} else {0} 
            ;

            new_matrix.set(col, row, match cur_matrix.get(col, row) {
                true => {
                    match neighbor_count {
                        ..= 1 => false,
                        2 | 3 => true,
                        _     => false,
                    }
                },
                false => {
                    match neighbor_count {
                        3 => true,
                        _ => false,
                    }
                }
            });
        }
    }
}


// Initialize the board
fn init_world(matrix: &mut ModularMatrix<bool>, rng: &mut rand::rngs::ThreadRng, clear: bool)
{
    for i in 0 .. matrix.width() {
        for j in 0 .. matrix.height() {
            matrix.set(i as isize, j as isize, match clear {
                true => false,
                false => {
                    match rng.gen_range(0..=100) {
                        0..=85 => false,
                        _      => true,
                    }
                }
            });
        }
    }
}


fn render_step(matrix: &ModularMatrix<bool>, canvas: &mut WindowCanvas)
{

    // Fill the whole canvas with the color of the grid
    canvas.set_draw_color(colors::GRID);
    canvas.clear();

    for i in 0 .. matrix.width() {
        for j in 0 .. matrix.height() {
            let x: usize = i * RECT_WIDTH_PX;
            let y: usize = j * RECT_HEIGHT_PX;
            let color: Color  = match matrix.get(i as isize, j as isize) {
                false => colors::EMPTY_SQUARE,
                true  => colors::FILLED_SQUARE,
            };

            canvas.set_draw_color(color);
            let _ = canvas.fill_rect(Rect::new(x as i32 + 1, y as i32 + 1, RECT_WIDTH_PX as u32 - 1, RECT_HEIGHT_PX as u32 - 1));
        }
    }
    canvas.present();
}


fn main() 
{
    let config = Cli::parse();

    let mut rng = rand::thread_rng();
    let mut paused: bool = false;
    let mut clicked: bool = false;
    let mut current_rect_x: usize = std::usize::MAX;
    let mut current_rect_y: usize = std::usize::MAX;
    let mut speed = NORMAL_SPEED;

    let mut cur_matrix = ModularMatrix::new(config.board_width, config.board_height, false);
    let mut new_matrix = ModularMatrix::new(config.board_width, config.board_height, false);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(
        "Game of Life in Rust", CANVAS_WIDTH_PX as u32, CANVAS_HEIGHT_PX as u32
    ).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    init_world(&mut cur_matrix, &mut rng, false);

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
                        init_world(&mut cur_matrix, &mut rng, true);
                    }
                },

                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if paused {
                        init_world(&mut cur_matrix, &mut rng, false);
                    }
                },

                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, ..} => {

                    if !paused { break; }

                    clicked = true;
                    let i = (x as usize / RECT_WIDTH_PX) as isize;
                    let j = (y as usize/ RECT_HEIGHT_PX) as isize;
                    cur_matrix.set(i, j, ! cur_matrix.get(i, j));

                    current_rect_x = i as usize;
                    current_rect_y = j as usize;
                }

                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    clicked = false;
                    current_rect_x = std::usize::MAX;
                    current_rect_y = std::usize::MAX;
                }

                Event::MouseMotion {x, y, ..} => {
                    if clicked {
                        let i = (x as usize / RECT_WIDTH_PX) as isize;
                        let j = (y as usize/ RECT_HEIGHT_PX) as isize;
                        if (current_rect_x != (i as usize)) || (current_rect_y != (j as usize)) {
                            cur_matrix.set(i, j, ! cur_matrix.get(i, j));

                            current_rect_x = i as usize;
                            current_rect_y = j as usize;
                        }
                    }
                }

                _ => {}
            }
        } // End event loop
        
        render_step(&cur_matrix, &mut canvas);
        if paused {
            continue;
        }

        do_step(&cur_matrix, &mut new_matrix);

        // Swap matrices
        let old_cur_matrix = cur_matrix;
        cur_matrix = new_matrix;
        new_matrix = old_cur_matrix;

        thread::sleep(speed);

    } // End game loop

}
