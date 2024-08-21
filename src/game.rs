#![allow(unused_imports, dead_code)]
use std::thread::sleep;
use std::time::Duration;
use std::fmt::Display;

const SIZE: usize = 12; 
const ITERATIONS: u32 = 2000;
const SEC_PER_ITER: f64 = 0.1;
const WHITE: &str = "\x1b[47m";  // fondo blanco
const BLACK: &str = "\x1b[40m";  // fondo negro
const RESET: &str = "\x1b[0m";   // resetear color

pub struct Game {
    pub cells: [[u32; SIZE]; SIZE] 
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        for col in 0..self.cells.len() {
            for row in 0..self.cells[col].len() {
                match self.cells[col][row] {
                    0 => {
                        display.push_str(&get_colored(BLACK, "0"));
                    }
                    1 => {
                        display.push_str(&get_colored(WHITE, "1"));
                    }
                    _ => unreachable!()
                }
                //display.push(char::from_digit(self.cells[col][row], 2).expect("Error con el numero pasado"));
            }
            display.push('\n')
        }
        write!(f, "{}", display)
    }
}

impl Game {

    pub fn init(&mut self) -> Result<(), &str> {
        for _ in 0..ITERATIONS {
            self.cells = self.next_instance();
            sleep(Duration::from_secs_f64(SEC_PER_ITER));
        }
        Ok(())
    }


    fn next_instance(&mut self) -> [[u32; SIZE]; SIZE] {
        println!("{self}");
        let mut next_arr = [[0; SIZE]; SIZE];
        for col in 0..self.cells.len() {
            for row in 0..self.cells[col].len() {
                if self.cells[col][row] != 1 {
                    continue
                }
                next_arr[col][row] = self.check_neighbors(&mut next_arr, col, row);
            }
        }
        next_arr
    }

    fn check_neighbors(&mut self, next_arr: &mut [[u32; SIZE]; SIZE], col: usize, row: usize) -> u32 {
        let mut neighbors = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue
                }
                let offset_col = (col as isize + i) as usize;
                let offset_row = (row as isize + j) as usize;
                if self.cells.get(offset_col).is_none() {
                    // continue
                    break
                }
                if self.cells.get(offset_row).is_none() {
                    continue
                }

                if self.cells[offset_col][offset_row] == 0 {
                    next_arr[offset_col][offset_row] = self.dead_check_neighbors(offset_col, offset_row);
                    continue
                }
                neighbors += 1;
            }
        }

        match neighbors {
            0 | 1 => {
                0
            }
            4.. => {
                0
            }
            _ => {
                1
            }
        }
    }

    fn dead_check_neighbors(&mut self, col: usize, row: usize) -> u32 {
        let mut neighbors = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                let offset_col = (col as isize + i) as usize;
                let offset_row = (row as isize + j) as usize;
                if self.cells.get(offset_col).is_none() {
                    //continue
                    break
                }
                if self.cells.get(offset_row).is_none() {
                    continue
                }
                if self.cells[offset_col][offset_row] != 1 {
                    continue
                } 
                neighbors += 1;
            }
        }

        if neighbors != 3 {
            return 0
        }
        1
    }
}

fn get_colored(color: &str, text: &str) -> String {
    format!("{} {} {}", color, text, RESET)
}

//    use std::io::{Error, stdout};
//    use crossterm::{queue, Command};
//    use crossterm::cursor::{MoveTo, Hide, Show};
//    use crossterm::style::Print;
//
//    struct Position {
//         x: u16,
//         y: u16,
//    }
//
//    fn queue_command<C: Command>(command: C) -> Result<(), Error> {
//        queue!(stdout(), command)
//    }
//
//    fn move_cursor_to(pos: Position) -> Result<(), Error> {
//        let Position{x, y} = pos;
//        queue_command(MoveTo(x, y))
//    }
//
//    fn print_stdout(msg: &str) -> Result<(), Error> {
//        queue_command(Print(msg))
//    }
//}
//mod terminal {
