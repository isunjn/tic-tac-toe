extern crate termion;

use std::io::{stdin, stdout, Write};
use std::process;
use std::usize;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Copy, Clone)]
enum Piece {
    Empty,
    X,
    O,
}

struct Game {
    board: [Piece; 9],
    pos: (u32, u32),
    turn: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [Piece::Empty; 9],
            pos: (1, 1),
            turn: 0,
        }
    }

    fn update_board(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        let pos = (self.pos.0 * 3 + self.pos.1) as usize;
        match stdin().keys().next().unwrap().unwrap() {
            Key::Char('q') => process::exit(0),
            Key::Left => {
                if self.pos.1 != 0 {
                    self.pos.1 -= 1;
                }
            }
            Key::Right => {
                if self.pos.1 != 2 {
                    self.pos.1 += 1;
                }
            }
            Key::Up => {
                if self.pos.0 != 0 {
                    self.pos.0 -= 1;
                }
            }
            Key::Down => {
                if self.pos.0 != 2 {
                    self.pos.0 += 1;
                }
            }
            Key::Char('o') => {
                if self.turn == 0 {
                    self.board[pos] = match self.board[pos] {
                        Piece::Empty => {
                            self.turn = 1;
                            Piece::O
                        }
                        Piece::O => Piece::O,
                        Piece::X => Piece::X,
                    };
                }
            }
            Key::Char('x') => {
                if self.turn == 1 {
                    self.board[pos] = match self.board[pos] {
                        Piece::Empty => {
                            self.turn = 0;
                            Piece::X
                        },
                        Piece::O => Piece::O,
                        Piece::X => Piece::X,
                    };
                    self.turn = 0;
                }
            }
            _ => {}
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
        let pos = (self.pos.0 * 3 + self.pos.1) as usize;
        ox[pos] = match ox[pos] {
            " " => "\x1b[34m+\x1b[0m",
            "O" => "\x1b[34mO\x1b[0m",
            "X" => "\x1b[34mX\x1b[0m",
            _ => " ",
        };

        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}Tic-Tac-Toe  <q> to exit.{}\n\r   {}'s turn\n\r",// TODO better ui
            termion::clear::All,
            termion::cursor::Goto(2, 1),
            termion::cursor::Hide,
            match self.turn {
                0 => 'O',
                1 => 'X',
                _ => ' ',
            }
        )
        .unwrap();
        write!(
            stdout,
            " {} | {} | {} \n\r───┼───┼───\n\r {} | {} | {} \n\r───┼───┼───\n\r {} | {} | {} \n\r",
            ox[0], ox[1], ox[2], ox[3], ox[4], ox[5], ox[6], ox[7], ox[8]
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    fn is_over(&self) {
        
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        game.draw_board();
        game.update_board();
        game.is_over();
    }
}
