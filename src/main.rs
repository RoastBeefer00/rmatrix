use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    layout::Rect, prelude::{CrosstermBackend, Terminal}, style::Style, text::{Line, Span, Text}, widgets::Paragraph
};
use std::io::{stdout, Result};
use rand::{thread_rng, Rng};
use log::info;
use std::num::Wrapping;

#[derive(Clone, Debug)]
struct LineState {
    stream: Stream,
    line: Vec<Cell>,
    chars: usize,
    whitespace: usize,
}

impl LineState {
    fn new(height: usize) -> Self {
        let mut rng = thread_rng();

        let stream = match rng.gen_bool(0.02) {
            true => Stream::On,
            false => Stream::Off,
        };

        Self {
            stream,
            line: vec![Cell::Whitespace; height.clone()],
            chars: rng.gen_range(5..height.clone() / 2),
            whitespace: rng.gen_range(10..height.clone()),
        }
    }

    pub fn update_lines(&mut self) {
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
                        Some(cell) => {
                            match cell {
                                Cell::Whitespace => {
                                    updated = false;
                                },
                                Cell::Sym(sym) => {
                                    match sym.white {
                                        true => {
                                            let idx = thread_rng().gen_range(0..CHARSET.len());
                                            let rand_char = CHARSET[idx] as char;
                                            sym.white = false;
                                            let next_cell = iter.next();
                                            match next_cell {
                                                Some(cell) => {
                                                    *cell = Cell::Sym(Sym {
                                                        value: rand_char.to_string(),
                                                        white: true,
                                                    });
                                                },
                                                None => {}
                                            }
                                            updated = true;
                                        },
                                        false => {
                                            if !updated {
                                                *cell = Cell::Whitespace;
                                                updated = true;
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                self.whitespace -= 1;
                info!("{}", self.whitespace);
                if self.whitespace <= 0 {
                    self.stream = Stream::On;
                    self.whitespace = rng.gen_range(10..line_len);
                }
            },
            Stream::On => {
                let line_len = self.line.len() - 1;
                let mut iter = self.line.iter_mut();
                loop {
                    let next = iter.next();
                    match next {
                        Some(cell) => {
                            match cell {
                                Cell::Whitespace => {
                                    if !updated{
                                        let idx = thread_rng().gen_range(0..CHARSET.len());
                                        let rand_char = CHARSET[idx] as char;
                                        *cell = Cell::Sym(Sym {
                                            value: rand_char.to_string(),
                                            white: true,
                                        });
                                        updated = true;
                                    }
                                },
                                Cell::Sym(sym) => {
                                    match sym.white {
                                        true => {
                                            let idx = thread_rng().gen_range(0..CHARSET.len());
                                            let rand_char = CHARSET[idx] as char;
                                            sym.white = false;
                                            let next_cell = iter.next();
                                            match next_cell {
                                                Some(cell) => {
                                                    *cell = Cell::Sym(Sym {
                                                        value: rand_char.to_string(),
                                                        white: true,
                                                    });
                                                },
                                                None => {}
                                            }
                                            updated = true;
                                        },
                                        false => {
                                            if updated {
                                                *cell = Cell::Whitespace;
                                                updated = false;
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                self.chars -= 1;
                info!("{}", self.chars);
                if self.chars <= 0 {
                    self.stream = Stream::Off;
                    self.chars = rng.gen_range(5..line_len);
                }
            },
        } 
    }
}

#[derive(Clone, Debug)]
struct Sym {
    value: String,
    white: bool,
}

#[derive(Clone, Debug)]
enum Cell {
    Sym(Sym),
    Whitespace,
}

#[derive(Clone, Debug)]
enum Stream {
    On,
    Off,
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let terminal_size = terminal.size().unwrap();
    let t_height = terminal_size.height;
    let t_width = terminal_size.width;
    terminal.clear()?;

    let mut matrix: Vec<LineState> = Vec::new(); 
    for _ in 0..t_width {
        matrix.push(LineState::new(t_height.into()));
    }

    loop {
        let terminal_size = terminal.size().unwrap();
        let t_height = terminal_size.height;
        let t_width = terminal_size.width;
        if t_width > matrix.len() as u16 {
            let sd = t_width as u32 - matrix.len() as u32;
            if sd > 0 {
                for _ in 0..sd {
                    matrix.push(LineState::new(t_height.into()));
                }
            }
        }

        let matrix_height = matrix.get(0).unwrap().line.len() as u16;
        if t_height > matrix_height {
             let sd = t_height as u32 - matrix_height as u32;
             if sd > 0 {
                 for col in &mut matrix {
                     for _ in 0..sd {
                         col.line.push(Cell::Whitespace);
                     }
                 }
             }
        }
        let mut update = false;
        for line in &mut matrix {
            if update {
                line.update_lines();
                update = false;
            } else {
                update = true;
            }
        }
        // info!("{:?}", matrix);
        terminal.draw(|frame| {
            let area = Rect::new(0, 0, frame.size().width, frame.size().height);
            for (i, col) in area.columns().enumerate() {
                let line_state = matrix.get(i).unwrap();
                let lines: Vec<Line> = line_state.line.clone().into_iter().map(|cell| {
                    match cell {
                        Cell::Sym(sym) => match sym.white {
                            true => Line::from(Span::styled(String::from(sym.value), Style::default().fg(ratatui::style::Color::White))),
                            false => Line::from(Span::styled(String::from(sym.value), Style::default().fg(ratatui::style::Color::Green))),
                        },
                        Cell::Whitespace => Line::from(String::from(" ")),
                    }
                }).collect();
                frame.render_widget(Paragraph::new(Text::from(lines)), col);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(60))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                    {
                        break;
                    }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
