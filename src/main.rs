extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{stdin, stdout, Write};
use std::usize;

#[derive(Copy, Clone, PartialEq)]
enum Piece {
    Empty,
    X,
    O,
}

static CHECK_POS: [(usize, usize, usize); 8] = [
    (0, 1, 2),
    (3, 4, 5),
    (6, 7, 8),
    (0, 3, 6),
    (1, 4, 7),
    (2, 5, 8),
    (0, 4, 8),
    (2, 4, 6),
];

struct Game {
    board: [Piece; 9],
    pos: usize,
    turn: Piece,
    quit: bool,
    winner: Option<Piece>,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [Piece::Empty; 9],
            pos: 4,
            turn: Piece::O,
            quit: false,
            winner: None,
        }
    }

    fn update_board(&mut self) {
        let pos = self.pos;
        loop {
            match stdin().keys().next().unwrap().unwrap() {
                Key::Char('q') => {
                    self.quit = true;
                    break;
                }
                Key::Left => {
                    if pos % 3 == 0 {
                        self.pos += 2
                    } else {
                        self.pos -= 1
                    }
                    break;
                }
                Key::Right => {
                    if pos % 3 == 2 {
                        self.pos -= 2
                    } else {
                        self.pos += 1
                    }
                    break;
                }
                Key::Up => {
                    if pos < 3 {
                        self.pos += 6
                    } else {
                        self.pos -= 3
                    }
                    break;
                }
                Key::Down => {
                    if pos > 5 {
                        self.pos -= 6
                    } else {
                        self.pos += 3
                    }
                    break;
                }
                Key::Char('o') => {
                    if self.turn == Piece::O && self.board[pos] == Piece::Empty {
                        self.board[pos] = Piece::O;
                        self.turn = Piece::X;
                        break;
                    }
                }
                Key::Char('x') => {
                    if self.turn == Piece::X && self.board[pos] == Piece::Empty {
                        self.board[pos] = Piece::X;
                        self.turn = Piece::O;
                        break;
                    }
                }
                _ => (),
            }
        }
    }

    fn draw_board(&self) {
        let mut pieces: Vec<&str> = self
            .board
            .iter()
            .map(|x| match *x {
                Piece::Empty => " ",
                Piece::O => "O",
                Piece::X => "X",
            })
            .collect();
        pieces[self.pos] = match pieces[self.pos] {
            " " => "\x1b[34m+\x1b[0m",
            "O" => "\x1b[34mO\x1b[0m",
            "X" => "\x1b[34mX\x1b[0m",
            _ => " ",
        };
        let turn = match self.turn {
            Piece::O => 'O',
            Piece::X => 'X',
            Piece::Empty => ' ',
        };

        let ui_1 = format!(
            "      {} │ {} │ {}      \x1b[32m↑/↓/←/→\x1b[0m to move\n\r",
            pieces[0], pieces[1], pieces[2]
        );
        let ui_2 = format!("     ───┼───┼───     \x1b[32mo/x\x1b[0m to set piece\n\r");
        let ui_3 = format!(
            "      {} │ {} │ {}      \x1b[32mq\x1b[0m to quit\n\r",
            pieces[3], pieces[4], pieces[5]
        );
        let ui_4 = format!("     ───┼───┼───\n\r");
        let ui_5 = format!(
            "      {} │ {} │ {}      \x1b[34m{}\x1b[0m's turn\n\r",
            pieces[6], pieces[7], pieces[8], turn
        );

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        print!("\n\r  ────────── Tic-Tac-Toe ──────────  \n\r\n\r");
        print!("{}{}{}{}{}", ui_1, ui_2, ui_3, ui_4, ui_5);
    }

    fn draw_end(&self) {
        if let Some(winner) = self.winner {
            self.draw_board();
            match winner {
                Piece::O => print!("\n\r\x1b[34m               O wins! 🎉\n\r\n\r\x1b[0m"),
                Piece::X => print!("\n\r\x1b[34m               X wins! 🎉\n\r\n\r\x1b[0m"),
                Piece::Empty => print!("\n\r\x1b[34m                Draw! 🤝\n\r\n\r\x1b[0m"),
            }
        }
    }

    fn is_over(&mut self) -> bool {
        let board = self.board;
        let mut no_one_can_win = true;
        for (p1, p2, p3) in CHECK_POS.iter() {
            match (board[*p1], board[*p2], board[*p3]) {
                (Piece::O, Piece::O, Piece::O) => {
                    self.winner = Some(Piece::O);
                    return true;
                }
                (Piece::X, Piece::X, Piece::X) => {
                    self.winner = Some(Piece::X);
                    return true;
                }
                (p_1, p_2, p_3) => {
                    let mut piece_type_cnt = 0;
                    if p_1 == Piece::O || p_2 == Piece::O || p_3 == Piece::O {
                        piece_type_cnt += 1;
                    }
                    if p_1 == Piece::X || p_2 == Piece::X || p_3 == Piece::X {
                        piece_type_cnt += 1;
                    }
                    if piece_type_cnt < 2 {
                        no_one_can_win = false;
                    }
                }
            }
        }
        if no_one_can_win {
            self.winner = Some(Piece::Empty);
            return true;
        }
        false
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    let mut game = Game::new();
    loop {
        game.draw_board();
        game.update_board();
        if game.quit || game.is_over() {
            game.draw_end();
            break;
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
