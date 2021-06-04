extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{stdin, stdout, Write};
use std::process;
use std::usize;

#[derive(Copy, Clone, PartialEq)]
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
                        }
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

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        print!("\n\r  ────────── Tic-Tac-Toe ──────────  \n\r\n\r");
        print!(
            "      {} │ {} │ {}      \x1b[32m↑/↓/←/→\x1b[0m to move\n\r     ───┼───┼───     \x1b[32mo/x\x1b[0m to set piece\n\r      {} │ {} │ {}      \x1b[32mq\x1b[0m to quit\n\r     ───┼───┼───\n\r      {} │ {} │ {}      \x1b[34m{}\x1b[0m's turn\n\r",
            ox[0], ox[1], ox[2], ox[3], ox[4], ox[5], ox[6], ox[7], ox[8],
            match self.turn {
                0 => 'O',
                1 => 'X',
                _ => ' ',
            }
        );
    }

    fn is_over(&self) -> bool {
        let board = self.board;
        if (board[0] == board[1] && board[0] == board[2] && board[1] == board[2] && board[2] == Piece::O)
        || (board[3] == board[4] && board[3] == board[5] && board[4] == board[5] && board[5] == Piece::O)
        || (board[6] == board[7] && board[6] == board[8] && board[7] == board[8] && board[8] == Piece::O)
        || (board[0] == board[4] && board[0] == board[8] && board[4] == board[8] && board[8] == Piece::O)
        || (board[2] == board[4] && board[2] == board[6] && board[4] == board[6] && board[6] == Piece::O)
        || (board[0] == board[3] && board[0] == board[6] && board[3] == board[6] && board[6] == Piece::O)
        || (board[1] == board[4] && board[1] == board[7] && board[4] == board[7] && board[7] == Piece::O)
        || (board[2] == board[5] && board[2] == board[8] && board[5] == board[8] && board[8] == Piece::O)
        {
            print!("\n\r\x1b[34m               O wins!\n\r\x1b[0m");
            return true;
        }
        if (board[0] == board[1] && board[0] == board[2] && board[1] == board[2] && board[2] == Piece::X)
        || (board[3] == board[4] && board[3] == board[5] && board[4] == board[5] && board[5] == Piece::X)
        || (board[6] == board[7] && board[6] == board[8] && board[7] == board[8] && board[8] == Piece::X)
        || (board[0] == board[4] && board[0] == board[8] && board[4] == board[8] && board[8] == Piece::X)
        || (board[2] == board[4] && board[2] == board[6] && board[4] == board[6] && board[6] == Piece::X)
        || (board[0] == board[3] && board[0] == board[6] && board[3] == board[6] && board[6] == Piece::X)
        || (board[1] == board[4] && board[1] == board[7] && board[4] == board[7] && board[7] == Piece::X)
        || (board[2] == board[5] && board[2] == board[8] && board[5] == board[8] && board[8] == Piece::X)
        {
            print!("\n\r\x1b[34m               X wins!\n\r\x1b[0m");
            return true;
        }

        for piece in board.iter() {
            if *piece == Piece::Empty {
                return false;
            }
        }
        print!("\n\r\x1b[34m               Draw!\n\r\x1b[0m");
        return true;
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    let mut game = Game::new();
    loop {
        game.draw_board();
        game.update_board();
        if game.is_over() {
            write!(stdout, "{}", termion::cursor::Show).unwrap();
            break;
        }
    }
}
