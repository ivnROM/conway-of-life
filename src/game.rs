use std::thread::sleep;
use std::time::Duration;
use std::fmt::Display;

const SIZE: usize = 12; 
const ITERATIONS: u8 = 20;
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
    //fn default() -> Self {
    //    let arr = [[0; SIZE]; SIZE];
    //    Game {
    //        cells: arr,
    //    }
    //}

    pub fn init(&mut self) -> Result<(), &str> {
        for _ in 0..ITERATIONS {
            self.cells = self.next_instance();
            sleep(Duration::from_secs(1));
        }
        Ok(())
    }


    // pasas una copia, y ajustas los resultados en abse a esa copia, no en base a la referencia q ya se modificó
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
                    // if self.cells[offset_col][offset_row] == 1 {
                    //     // neighbors += 1;V
                    //     continue
                    // }
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
                // fijarse si esto puedo reducirlo a un simple if con OR logico
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
