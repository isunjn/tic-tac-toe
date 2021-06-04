use std::{io, usize};

#[derive(Copy, Clone)]
enum Piece {
    Empty,
    X,
    O,
}

struct Game {
    board: [Piece; 9],
    pos: (u32, u32),
}

impl Game {
    fn new() -> Game {
        Game {
            board: [Piece::Empty; 9],
            pos: (1, 1),
        }
    }

    fn move_pos(&mut self) {
        println!("Move a bit:");
        let mut input_buf = String::new();
        io::stdin()
            .read_line(&mut input_buf)
            .expect("Need an input");
        let ch = input_buf.chars().next().unwrap();
        match ch {
            'w' => {
                if self.pos.0 != 0 {
                    self.pos.0 -= 1;
                }
            }
            'a' => {
                if self.pos.1 != 0 {
                    self.pos.1 -= 1;
                }
            }
            's' => {
                if self.pos.0 != 2 {
                    self.pos.0 += 1;
                }
            }
            'd' => {
                if self.pos.1 != 2 {
                    self.pos.1 += 1;
                }
            }
            _ => (),
        }
    }

    fn draw_board(&self) {
        let mut ox: Vec<&str> = self
            .board
            .iter()
            .map(|x| match *x {
                Piece::Empty => " ",
                Piece::O => "O",
                Piece::X => "X",
            })
            .collect();
        let p = (self.pos.0 * 3 + self.pos.1) as usize;
        ox[p] = match ox[p] {
            " " => "\x1b[34m+\x1b[0m",
            "O" => "\x1b[34mO\x1b[0m",
            "X" => "\x1b[34mX\x1b[0m",
            _ => " ",
        };
        println!(
            " {} | {} | {} \n───┼───┼───\n {} | {} | {} \n───┼───┼───\n {} | {} | {} ",
            ox[0], ox[1], ox[2], ox[3], ox[4], ox[5], ox[6], ox[7], ox[8]
        );
    }

    // fn is_game_over(&self) -> bool {}
}

fn main() {
    let mut game = Game::new();
    loop {
        game.draw_board();
        game.move_pos();
    }
}
