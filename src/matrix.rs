use log::info;
use rand::{thread_rng, Rng};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::Stdout;

// Keep track of the state of each column individually
#[derive(Clone, Debug)]
pub struct LineState {
    //Whether the stream is on or off
    pub stream: Stream,
    // The state of the line
    pub line: Vec<Cell>,
    // How many random chars to write
    pub chars: usize,
    // How many white spaces to write
    pub whitespace: usize,
}

impl LineState {
    // Create anew line with random number of chars and whitespace to create
    pub fn new(height: usize) -> Self {
        let mut rng = thread_rng();

        let stream = match rng.gen_bool(0.02) {
            true => Stream::On,
            false => Stream::Off,
        };

        Self {
            stream,
            line: vec![Cell::Whitespace; height],
            chars: rng.gen_range(5..height / 2),
            whitespace: rng.gen_range(10..height),
        }
    }

    // Update the line each tick
    pub fn update_line(&mut self) {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(}{][*&^%$#@!~";
        let mut rng = thread_rng();
        let mut updated = false;
        match self.stream {
            Stream::Off => {
                let line_len = self.line.len() - 1;
                let mut iter = self.line.iter_mut();
                loop {
                    let next = iter.next();
                    match next {
                        Some(cell) => match cell {
                            Cell::Whitespace => {
                                updated = false;
                            }
                            Cell::Sym(sym) => match sym.white {
                                true => {
                                    let idx = thread_rng().gen_range(0..CHARSET.len());
                                    let rand_char = CHARSET[idx] as char;
                                    sym.white = false;
                                    let next_cell = iter.next();
                                    if let Some(cell) = next_cell {
                                        *cell = Cell::Sym(Sym {
                                            value: rand_char.to_string(),
                                            white: true,
                                        });
                                    }
                                    updated = true;
                                }
                                false => {
                                    if !updated {
                                        *cell = Cell::Whitespace;
                                        updated = true;
                                    }
                                }
                            },
                        },
                        None => {
                            break;
                        }
                    }
                }
                self.whitespace -= 1;
                info!("{}", self.whitespace);
                if self.whitespace == 0 {
                    self.stream = Stream::On;
                    self.whitespace = rng.gen_range(10..line_len);
                }
            }
            Stream::On => {
                let line_len = self.line.len() - 1;
                let mut iter = self.line.iter_mut();
                loop {
                    let next = iter.next();
                    match next {
                        Some(cell) => match cell {
                            Cell::Whitespace => {
                                if !updated {
                                    let idx = thread_rng().gen_range(0..CHARSET.len());
                                    let rand_char = CHARSET[idx] as char;
                                    *cell = Cell::Sym(Sym {
                                        value: rand_char.to_string(),
                                        white: true,
                                    });
                                    updated = true;
                                }
                            }
                            Cell::Sym(sym) => match sym.white {
                                true => {
                                    let idx = thread_rng().gen_range(0..CHARSET.len());
                                    let rand_char = CHARSET[idx] as char;
                                    sym.white = false;
                                    let next_cell = iter.next();
                                    if let Some(cell) = next_cell {
                                        *cell = Cell::Sym(Sym {
                                            value: rand_char.to_string(),
                                            white: true,
                                        });
                                    }
                                    updated = true;
                                }
                                false => {
                                    if updated {
                                        *cell = Cell::Whitespace;
                                        updated = false;
                                    }
                                }
                            },
                        },
                        None => {
                            break;
                        }
                    }
                }
                self.chars -= 1;
                info!("{}", self.chars);
                if self.chars == 0 {
                    self.stream = Stream::Off;
                    self.chars = rng.gen_range(5..line_len);
                }
            }
        }
    }
}

// A symbol has a character value and either is white (first of stream) or not
#[derive(Clone, Debug)]
pub struct Sym {
    pub value: String,
    pub white: bool,
}

// A cell either is a symbol or a whitespace
#[derive(Clone, Debug)]
pub enum Cell {
    Sym(Sym),
    Whitespace,
}

// The stream is either on (printing chars) or off (printing whitespace)
#[derive(Clone, Debug)]
pub enum Stream {
    On,
    Off,
}

pub fn handle_resize(terminal: &Terminal<CrosstermBackend<Stdout>>, matrix: &mut Vec<LineState>) {
    // Handle the terminal growing in size
    // (Currently don't care about terminal shrinking)
    let terminal_size = terminal.size().unwrap();
    let t_height = terminal_size.height;
    let t_width = terminal_size.width / 2;
    if t_width > matrix.len() as u16 {
        let sd = t_width as u32 - matrix.len() as u32;
        if sd > 0 {
            for _ in 0..sd {
                matrix.push(LineState::new(t_height.into()));
            }
        }
    }

    let matrix_height = matrix.first().unwrap().line.len() as u16;
    if t_height > matrix_height {
        let sd = t_height as u32 - matrix_height as u32;
        if sd > 0 {
            for col in matrix {
                for _ in 0..sd {
                    col.line.push(Cell::Whitespace);
                }
            }
        }
    }
}
