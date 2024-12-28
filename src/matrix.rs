use log::info;
use rand::{thread_rng, Rng};
use ratatui::{
    layout::Rect,
    prelude::{CrosstermBackend, Terminal},
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};
use std::io::Result;
use std::io::Stdout;

pub struct State {
    pub color: String,
    pub speed: u64,
    pub direction: Direction,
}

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
                while let Some(cell) = iter.next() {
                    match cell {
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
                    }
                }
                self.whitespace -= 1;
                if self.whitespace == 0 {
                    self.stream = Stream::On;
                    self.whitespace = rng.gen_range(10..line_len);
                }
            }
            Stream::On => {
                let line_len = self.line.len() - 1;
                let mut iter = self.line.iter_mut();
                while let Some(cell) = iter.next() {
                    match cell {
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
                    }
                }
                self.chars -= 1;
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
//
// The stream is either on (printing chars) or off (printing whitespace)
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

pub fn process_matrix_cols(
    i: usize,
    line: Rect,
    frame: &mut Frame,
    matrix: &mut [LineState],
    state: &State,
) {
    if i / 2 >= matrix.len() {
        return;
    }
    let line_state = matrix.get_mut(i / 2).unwrap();
    if state.direction == Direction::Up || state.direction == Direction::Left {
        line_state.line.reverse();
    }
    let new_line = line_state.line.clone();
    let lines: Vec<Line> = new_line
        .into_iter()
        .map(|cell| {
            // Determine how to print each line
            match cell {
                Cell::Sym(sym) => match sym.white {
                    true => Line::from(Span::styled(
                        sym.value,
                        Style::default().fg(ratatui::style::Color::White),
                    )),
                    false => match state.color.as_str() {
                        "blue" => Line::from(Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Blue),
                        )),
                        "cyan" => Line::from(Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Cyan),
                        )),
                        "red" => Line::from(Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Red),
                        )),
                        "purple" => Line::from(Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Magenta),
                        )),
                        "yellow" => Line::from(Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Yellow),
                        )),
                        "rainbow" => {
                            let mut rng = thread_rng();
                            let colors = ["blue", "cyan", "red", "purple", "yellow", "green"];
                            let index = rng.gen_range(0..=colors.len() - 1);
                            let color = colors[index];
                            match color {
                                "blue" => Line::from(Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Blue),
                                )),
                                "cyan" => Line::from(Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Cyan),
                                )),
                                "red" => Line::from(Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Red),
                                )),
                                "purple" => Line::from(Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Magenta),
                                )),
                                "yellow" => Line::from(Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Yellow),
                                )),
                                _ => Line::from(Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Green),
                                )),
                            }
                        }
                        _ => Line::from(Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Green),
                        )),
                    },
                },
                Cell::Whitespace => Line::from(String::from(" ")),
            }
        })
        .collect();
    // Render the line as a paragraph
    frame.render_widget(Paragraph::new(Text::from(lines)), line);
    if state.direction == Direction::Up || state.direction == Direction::Left {
        line_state.line.reverse();
    }
}

pub fn process_matrix_rows(
    i: usize,
    line: Rect,
    frame: &mut Frame,
    matrix: &mut [LineState],
    state: &State,
) {
    if i >= matrix.len() {
        return;
    }
    let line_state = matrix.get_mut(i).unwrap();
    if state.direction == Direction::Up || state.direction == Direction::Left {
        line_state.line.reverse();
    }
    let new_line = line_state.line.clone();
    let lines: Vec<Span> = new_line
        .into_iter()
        .map(|cell| {
            // Determine how to print each line
            match cell {
                Cell::Sym(sym) => match sym.white {
                    true => {
                        Span::styled(sym.value, Style::default().fg(ratatui::style::Color::White))
                    }
                    false => match state.color.as_str() {
                        "blue" => Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Blue),
                        ),
                        "cyan" => Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Cyan),
                        ),
                        "red" => {
                            Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Red))
                        }
                        "purple" => Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Magenta),
                        ),
                        "yellow" => Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Yellow),
                        ),
                        "rainbow" => {
                            let mut rng = thread_rng();
                            let colors = ["blue", "cyan", "red", "purple", "yellow", "green"];
                            let index = rng.gen_range(0..=colors.len() - 1);
                            let color = colors[index];
                            match color {
                                "blue" => Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Blue),
                                ),
                                "cyan" => Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Cyan),
                                ),
                                "red" => Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Red),
                                ),
                                "purple" => Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Magenta),
                                ),
                                "yellow" => Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Yellow),
                                ),
                                _ => Span::styled(
                                    sym.value,
                                    Style::default().fg(ratatui::style::Color::Green),
                                ),
                            }
                        }
                        _ => Span::styled(
                            sym.value,
                            Style::default().fg(ratatui::style::Color::Green),
                        ),
                    },
                },
                Cell::Whitespace => Span::from(String::from(" ")),
            }
        })
        .collect();
    // Render the line
    frame.render_widget(Line::from(lines), line);
    if state.direction == Direction::Up || state.direction == Direction::Left {
        line_state.line.reverse();
    }
}

pub fn create_matrix(
    matrix: &mut Vec<LineState>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: &State,
) -> Result<()> {
    let terminal_size = terminal.size().unwrap();
    let t_height = terminal_size.height;
    let t_width = terminal_size.width;
    terminal.clear()?;

    // Create new matrix where each column has its own state
    // Only need half the columns because using all looks cluttered
    *matrix = Vec::new();
    if state.direction == Direction::Up || state.direction == Direction::Down {
        for _ in 0..t_width / 2 + 1 {
            matrix.push(LineState::new(t_height.into()));
        }
    } else {
        for _ in 0..t_height - 1 {
            matrix.push(LineState::new(t_width.into()));
        }
    }

    info!("Matrix len: {}", matrix.len());

    Ok(())
}
