// https://www.youtube.com/watch?v=nnqZtBgnWKM
const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

const RESET:  &str  = "\x1b[0m";
const ORANGE    :&str = "\x1b[93m";
const EWD :&str = "\x1b[31m";
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
                    1 => "0xF0 0x9F 0x94 0xB4", // red
                    2 => "0xF0 0x9F 0x9F 0xA1", // yellow
                    _ => "0xE2 0x97 0x8F", // black
                })
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{}", row_str);
        }

        println!("{}--------------------{}", ORANGE, RESET);

        // put the winner here!

        println!("{}--------------------{}", ORANGE, RESET);
    }
}

fn main() {
    println!("Hello, world!");
}
