use log::info;
use rand::{rng, RngExt};
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
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
    pub bold: bool,
}

// Keep track of the state of each column individually
#[derive(Clone, Debug)]
pub struct LineState {
    pub stream: Stream,
    pub line: Vec<Cell>,
    pub chars: usize,
    pub whitespace: usize,
    pub color: Color,
    pub brightness: f32,
}

impl LineState {
    pub fn new(height: usize, color: Color) -> Self {
        let mut rng = rng();

        let stream = match rng.random_bool(0.02) {
            true => Stream::On,
            false => Stream::Off,
        };

        Self {
            stream,
            line: vec![Cell::Whitespace; height],
            chars: rng.random_range(5..height / 2),
            whitespace: rng.random_range(10..height),
            color: color.resolve(),
            brightness: rng.random_range(0.55..=1.0),
        }
    }

    // Update the line each tick
    pub fn update_line(&mut self) {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(}{][*&^%$#@!~";
        // Bump distance on every existing Sym BEFORE shift logic so new heads can stay at 0.
        for cell in &mut self.line {
            if let Cell::Sym(s) = cell {
                s.distance = s.distance.saturating_add(1);
            }
        }
        let mut rng = rng();
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
                                    let idx = rand::rng().random_range(0..CHARSET.len());
                                    let rand_char = CHARSET[idx] as char;
                                    sym.white = false;
                                    let next_cell = iter.next();
                                    if let Some(cell) = next_cell {
                                        *cell = Cell::Sym(Sym {
                                            value: rand_char.to_string(),
                                            white: true,
                                            distance: 0,
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
                if self.whitespace == 0 {
                    self.stream = Stream::On;
                    self.whitespace = rng.random_range(10..line_len);
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
                                    let idx = rand::rng().random_range(0..CHARSET.len());
                                    let rand_char = CHARSET[idx] as char;
                                    *cell = Cell::Sym(Sym {
                                        value: rand_char.to_string(),
                                        white: true,
                                        distance: 0,
                                    });
                                    updated = true;
                                }
                            }
                            Cell::Sym(sym) => match sym.white {
                                true => {
                                    let idx = rand::rng().random_range(0..CHARSET.len());
                                    let rand_char = CHARSET[idx] as char;
                                    sym.white = false;
                                    let next_cell = iter.next();
                                    if let Some(cell) = next_cell {
                                        *cell = Cell::Sym(Sym {
                                            value: rand_char.to_string(),
                                            white: true,
                                            distance: 0,
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
                if self.chars == 0 {
                    self.stream = Stream::Off;
                    self.chars = rng.random_range(5..line_len);
                }
            }
        }
    }
}

// A symbol has a character value, a head flag, and distance from head (0 = head)
#[derive(Clone, Debug)]
pub struct Sym {
    pub value: String,
    pub white: bool,
    pub distance: u8,
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
#[derive(Clone, Debug, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Color {
    Green,
    Cyan,
    Red,
    Blue,
    Purple,
    Yellow,
    Rainbow,
}

impl Color {
    fn resolve(self) -> Color {
        match self {
            Color::Rainbow => {
                let pool: Vec<Color> = Color::iter()
                    .filter(|c| !matches!(c, Color::Rainbow))
                    .collect();
                pool[rng().random_range(0..pool.len())]
            }
            c => c,
        }
    }

    fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Green => (0, 255, 70),
            Color::Cyan => (0, 255, 255),
            Color::Red => (255, 50, 50),
            Color::Blue => (60, 120, 255),
            Color::Purple => (200, 0, 255),
            Color::Yellow => (255, 220, 0),
            Color::Rainbow => (0, 255, 70),
        }
    }
}

impl Sym {
    fn style(bold: bool, color: ratatui::style::Color) -> Style {
        let mut style = Style::default().fg(color);
        if bold {
            style = style.add_modifier(ratatui::style::Modifier::BOLD);
        }
        style
    }

    fn faded(color: Color, distance: u8, fade_over: u8, brightness: f32) -> ratatui::style::Color {
        let (r, g, b) = if distance == 0 {
            (255, 255, 255)
        } else {
            color.rgb()
        };
        let fade_over = fade_over.max(1);
        let fade = if distance == 0 {
            1.0
        } else {
            (1.0 - distance.min(fade_over) as f32 / fade_over as f32).sqrt()
        };
        let factor = fade * brightness;
        ratatui::style::Color::Rgb(
            (r as f32 * factor) as u8,
            (g as f32 * factor) as u8,
            (b as f32 * factor) as u8,
        )
    }

    pub fn render_col(
        &self,
        color: Color,
        bold: bool,
        fade_over: u8,
        brightness: f32,
    ) -> Line<'static> {
        let rcolor = Self::faded(color, self.distance, fade_over, brightness);
        Line::from(Span::styled(
            self.value.clone(),
            Self::style(bold && self.distance == 0, rcolor),
        ))
    }

    pub fn render_row(
        &self,
        color: Color,
        bold: bool,
        fade_over: u8,
        brightness: f32,
    ) -> Span<'static> {
        let rcolor = Self::faded(color, self.distance, fade_over, brightness);
        Span::styled(
            self.value.clone(),
            Self::style(bold && self.distance == 0, rcolor),
        )
    }
}

impl State {
    fn color(&self) -> Color {
        Color::from_str(&self.color).unwrap_or(Color::Green)
    }

    fn is_reversed(&self) -> bool {
        self.direction == Direction::Up || self.direction == Direction::Left
    }

    pub fn render_col(&self, i: usize, line: Rect, frame: &mut Frame, matrix: &mut [LineState]) {
        if i / 2 >= matrix.len() {
            return;
        }
        let line_state = matrix.get_mut(i / 2).unwrap();
        if self.is_reversed() {
            line_state.line.reverse();
        }
        let color = line_state.color;
        let brightness = line_state.brightness;
        let fade_over = line_state.line.len().min(u8::MAX as usize) as u8;
        let lines: Vec<Line> = line_state
            .line
            .iter()
            .map(|cell| match cell {
                Cell::Sym(sym) => sym.render_col(color, self.bold, fade_over, brightness),
                Cell::Whitespace => Line::from(" "),
            })
            .collect();
        frame.render_widget(Paragraph::new(Text::from(lines)), line);
        if self.is_reversed() {
            line_state.line.reverse();
        }
    }

    pub fn render_row(&self, i: usize, line: Rect, frame: &mut Frame, matrix: &mut [LineState]) {
        if i >= matrix.len() {
            return;
        }
        let line_state = matrix.get_mut(i).unwrap();
        if self.is_reversed() {
            line_state.line.reverse();
        }
        let color = line_state.color;
        let brightness = line_state.brightness;
        let fade_over = line_state.line.len().min(u8::MAX as usize) as u8;
        let spans: Vec<Span> = line_state
            .line
            .iter()
            .map(|cell| match cell {
                Cell::Sym(sym) => sym.render_row(color, self.bold, fade_over, brightness),
                Cell::Whitespace => Span::from(" "),
            })
            .collect();
        frame.render_widget(Line::from(spans), line);
        if self.is_reversed() {
            line_state.line.reverse();
        }
    }

    pub fn build_matrix(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<Vec<LineState>> {
        let size = terminal.size().unwrap();
        terminal.clear()?;
        let vertical = self.direction == Direction::Up || self.direction == Direction::Down;
        let (count, len) = if vertical {
            (size.width / 2 + 1, size.height as usize)
        } else {
            (size.height - 1, size.width as usize)
        };
        let user_color = self.color();
        let matrix: Vec<LineState> = (0..count).map(|_| LineState::new(len, user_color)).collect();
        info!("Matrix len: {}", matrix.len());
        Ok(matrix)
    }
}

