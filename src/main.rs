#![allow(unused_imports, dead_code)]

pub mod game {
    use std::thread::sleep;
    use std::time::Duration;
    use std::fmt::Display;

    pub struct Game {
        pub cells: [[u32; 6]; 6] 
    }

    impl Display for Game {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut display = String::new();
            for col in 0..self.cells.len() {
                for row in 0..self.cells[col].len() {
                    display.push(char::from_digit(self.cells[col][row], 2).expect("Error con el numero pasado"));
                }
                display.push('\n')
            }
            write!(f, "{}", display)
        }
    }

    impl Game {
        fn default() -> Self {
            let arr = [[0; 6]; 6];
            Game {
                cells: arr,
            }
        }

        pub fn init(&mut self) -> Result<(), &str> {
            for _ in 0..10 {
                self.next_instance();
                sleep(Duration::from_secs(1));
            }
            Ok(())
        }


        // pasas una copia, y ajustas los resultados en abse a esa copia, no en base a la referencia q ya se modificó
        fn next_instance(&mut self) -> Game {
            println!("{self}");
            for col in 0..self.cells.len() {
                for row in 0..self.cells[col].len() {
                    if self.cells[col][row] != 1 {
                        continue
                    }
                    self.cells[col][row] = self.check_neighbors(col, row);
                }
            }
        }

        fn check_neighbors(&mut self, col: usize, row: usize) -> u32 {
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
                        self.cells[offset_col][offset_row] = self.dead_check_neighbors(offset_col, offset_row);
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
                    return 0
                }
                4.. => {
                    return 0
                }
                _ => {
                    return 1
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
                        continue
                        //break
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
}


fn main() {
    use game::Game;
    let mut game = Game {
        cells: [
                    [0, 1, 0, 0, 0, 0],
                    [0, 1, 1, 0, 1, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0]
               ]
    };
    game.init().expect("Fallo en la inicializacion del juego");
    println!("FIN DEL JUEGO!");
}
