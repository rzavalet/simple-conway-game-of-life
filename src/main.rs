use rand::Rng;
use std::thread;
use std::time::Duration;

const SIZE: usize = 20;

fn do_step(array: &mut [[[i32;SIZE];SIZE];2], cur_matrix: usize) {
    let mut counter;
    for i in 1..SIZE-1 {
        for j in 1..SIZE-1 {
            counter = array[cur_matrix][i-1][j-1] +
                      array[cur_matrix][i-1][j+0] +
                      array[cur_matrix][i-1][j+1] +
                      array[cur_matrix][i+0]  [j-1] +
                      //array[cur_matrix][i+0][j+0] +
                      array[cur_matrix][i+0] [j+1] +
                      array[cur_matrix][i+1][j-1] +
                      array[cur_matrix][i+1][j+0] +
                      array[cur_matrix][i+1][j+1];


            array[1 - cur_matrix][i][j] = match array[cur_matrix][i][j] {
                1 => {
                    match counter {
                        ..= 1  => 0,
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

fn print_matrix(array: [[[i32;SIZE];SIZE];2], cur_matrix: usize) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{} ", if array[cur_matrix][i][j] == 1 {
                '\u{2588}'
            }
            else {
                ' '
            }
            );
        }
        print!("\n");

    }

}

fn main() {
    
    let duration = Duration::from_secs(1);
    let mut cur_matrix = 0;
    let mut array: [[[i32;SIZE];SIZE];2] = [[[0;SIZE];SIZE];2];

    let mut rng = rand::thread_rng();

    for i in 0..SIZE {
        for j in 0..SIZE {
            array[cur_matrix][i][j] = rng.gen_range(0..=1);
        }
    }

    for _i in 0..10 {
        print_matrix(array, cur_matrix);
        do_step(&mut array, cur_matrix);
        cur_matrix = 1 - cur_matrix;
        thread::sleep(duration);
    }

}
