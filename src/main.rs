// https://www.youtube.com/watch?v=nnqZtBgnWKM
// cargo watch -c -w src -x run

//use std::fmt::write;

use std::io;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

const RESET:  &str  = "\x1b[0m";
const ORANGE    :&str = "\x1b[93m";
const RED :&str = "\x1b[0;31m";
type BOARD = [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum PLAYER {
    One = 1,
    Two = 2,
    None = 0,
}

impl PLAYER {
    fn from_int(int:u8) -> PLAYER{
        match int {
            1 => PLAYER::One,
            2 => PLAYER::Two,
            _ => PLAYER::None,
        }
    }
}

#[derive(Debug)]
enum MoveError{
    GameFinished,
    InvalidColumn,
    ColumnFull,
}

impl std::fmt::Display for MoveError{
    fn fmt (&self, f: &mut std::fmt::Formatter<'_ >) ->
        std::fmt::Result {
            match self {
                MoveError::ColumnFull => write!(f, "column is full"),
                MoveError::InvalidColumn => write!(f, "column must be between 1 and 7"),
                MoveError::GameFinished => write!(f, "game is already finished."),
            }
        }
    }

struct Game {
    current_move: u8,
    current_player: PLAYER,
    board: BOARD,
    is_finished: bool,
    winner: PLAYER,
}

impl Game {
    fn default() -> Game {
        Game {
            current_move: 0,
            current_player: PLAYER::One,
            board: [
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ],
            is_finished: false,
            winner: PLAYER::None,
        }
    }
    fn display_board(&self){
        println!("{}--------------------{}", ORANGE, RESET);
        println!("{}CONNECT 4 (Move {}){}", ORANGE, self.current_move, RESET);
        println!("{}--------------------{}", ORANGE, RESET);

        for row in self.board{
            let row_str:String = row
                .iter()
                .map(|&cell| match cell{
                    1 => "🔴", // red
                    2 => "🟡", // yellow
                    _ => "⚫", // black
                })
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{}", row_str);
        }

        println!("{}--------------------{}", ORANGE, RESET);

        if self.is_finished {
            match self.winner {
                PLAYER::One => println!("{} 🔴 Player One has won!{}", ORANGE, RESET),
                PLAYER::Two => println!("{} 🟡 Player Two has won!{}", ORANGE, RESET),
                PLAYER::None => println!("{} It's a draw!{}", ORANGE, RESET),
            }
        }

        println!("{}--------------------{}", ORANGE, RESET);
    }

    fn display_error (&self, error:String) {
        self.display_board();
        println!("{}Error: {}{}", RED, error, RESET);
    }

    fn calculate_winner (&mut self) -> PLAYER{
        if self.current_move < 7 {
            return PLAYER::None;
        }
        let directions = [
            (0, 1),     // horizontal
            (1, 0),     // vertical
            (1, 1),     // diagonal  (top left to bottom right)
            (-1, 1),    // diagonal (bottom left to top right)
        ];

        // testing
        for row in 0..BOARD_HEIGHT{
            for col in 0..BOARD_WIDTH{
                let cell  = self.board[row][col];
                if cell != 0 {
                    for (row_step, col_step) in directions{
                        let mut consecutive_count = 1;
                        let mut r = row as isize + row_step;
                        let mut c = col as isize + col_step;

                        while r > 0
                            && r < BOARD_HEIGHT as isize
                            && c >= 0
                            && c < BOARD_WIDTH as isize{
                                if self.board[r as usize][c as usize] == cell {
                                    consecutive_count +=1;
                                    if consecutive_count  == 4 {
                                        self.is_finished = true;
                                        return PLAYER::from_int(cell);
                                    }
                                } else {
                                    break;
                                }
                            r += row_step;
                            c += col_step;
                        }

                    }
                }
            }
        }
        if self.current_move>=BOARD_HEIGHT as u8 * BOARD_WIDTH as u8{
            self.is_finished=true;
        }
        PLAYER::None
    }

    fn play_move(&mut self, column: usize) -> Result<(), MoveError>{
        if self.is_finished {
            return Err(MoveError::GameFinished);
        }
        if column >= BOARD_HEIGHT {
            return Err(MoveError::ColumnFull);
        }
        if let Some(row) = (0..BOARD_HEIGHT)
            .rev()
            .find(|&row| self.board[row][column] == 0) {
            self.board[row][column] = self.current_player as u8;
            self.current_move += 1;
        } else {
            return Err(MoveError::InvalidColumn);
        }

        let calculated_winner = self.calculate_winner();
        if calculated_winner != PLAYER::None {
            self.winner = calculated_winner;
        } else {
            self.current_player = match self.current_player {
                PLAYER::One => PLAYER::Two,
                _ => PLAYER::One,
            };
        }
        Ok(());
    }
}

fn main() {
    let mut game = Game::default();
    game.display_board();

    while !game.is_finished{
        println!("\n");
        match game.current_player {
            PLAYER::One => println!("PLAYER ONE"),
            PLAYER::Two => println!("PLAYER TWO"),
            _ => (),
        }
        println!("Enter a column between 1 and 7: ");
        let mut user_move = String::new();
        io::stdin()
            . read_line(&mut user_move)
            . expect("Failed to read line");

        let user_move : usize = match user_move.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 7 {
                    game.display_error(MoveError::InvalidColumn.to_string());
                    continue;
                } else {
                    num
                }
            }
            Err(err) => {
                game.display_error(err.to_string());
                continue;
            }
        };
        match game.play_move(user_move -1) {
            Ok() => {
                game.display_board();
            }
            Err(err) => {
                game.display_error(err.to_string());
            }
        }
    }
}
