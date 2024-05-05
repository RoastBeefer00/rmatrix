use clap::Parser;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::info;
use log4rs;
use matrix::{Cell, LineState};
use rand::{thread_rng, Rng};
use ratatui::{
    layout::Rect,
    prelude::{CrosstermBackend, Terminal},
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

#[derive(Parser)]
#[command(about = "Creates the matrix in the terminal. Use `c` to cycle colors, `0-9` to change speed, and `q` to quit.")]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "COLOR",
        help = "Available colors: blue, cyan, red, purple, yellow, green, rainbow"
    )]
    color: Option<String>,
    #[arg(short, long, value_name = "SPEED", help = "Speed: 1-10")]
    speed: Option<i8>,
}

struct State {
    color: String,
    speed: u64,
}

mod matrix;

fn main() -> Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let cli = Cli::parse();
    // Poll duration determines how fast the matrix falls
    let mut speed = match cli.speed {
        Some(s) => match s {
            1 => 120,
            2 => 100,
            3 => 80,
            4 => 60,
            5 => 50,
            6 => 40,
            7 => 30,
            8 => 20,
            9 => 10,
            10 => 5,
            _ => 60,
        },
        None => 60,
    };
    let mut state = if let Some(color) = cli.color.as_deref() {
        match color.to_lowercase().as_str() {
            "blue" => State {
                color: color.to_string(),
                speed,
            },
            "cyan" => State {
                color: color.to_string(),
                speed,
            },
            "red" => State {
                color: color.to_string(),
                speed,
            },
            "purple" => State {
                color: color.to_string(),
                speed,
            },
            "yellow" => State {
                color: color.to_string(),
                speed,
            },
            "rainbow" => State {
                color: color.to_string(),
                speed,
            },
            _ => State {
                color: "green".to_string(),
                speed,
            },
        }
    } else {
        State {
            color: "green".to_string(),
            speed,
        }
    };
    // Initialize ratatui and get terminal size
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let terminal_size = terminal.size().unwrap();
    let t_height = terminal_size.height;
    let t_width = terminal_size.width;
    terminal.clear()?;

    // Create new matrix where each column has its own state
    // Only need half the columns because using all looks cluttered
    let mut matrix: Vec<LineState> = Vec::new();
    for _ in 0..t_width / 2 + 1 {
        matrix.push(LineState::new(t_height.into()));
    }

    loop {
        matrix::handle_resize(&mut terminal, &mut matrix);
        // Only print matrix every other column
        // Looks better than using every column
        for line in matrix.iter_mut() {
            line.update_line();
        }

        // Draw the matrix after updating all lines
        terminal.draw(|frame| {
            let area = Rect::new(0, 0, frame.size().width, frame.size().height);
            // Get the state of every other column
            for (i, col) in area.columns().enumerate().step_by(2) {
                if i / 2 >= matrix.len() {
                    continue;
                }
                info!("Matrix len: {}", matrix.len());
                info!("Getting line: {}", i / 2);
                let line_state = matrix.get(i / 2).unwrap();
                let lines: Vec<Line> = line_state
                    .line
                    .clone()
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
                                        let color;
                                        let mut rng = thread_rng();
                                        let colors =
                                            ["blue", "cyan", "red", "purple", "yellow", "green"];
                                        let index = rng.gen_range(0..=colors.len() - 1);
                                        color = colors[index];
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
                frame.render_widget(Paragraph::new(Text::from(lines)), col);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(state.speed))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') => {
                            let mut rng = thread_rng();
                            let mut colors: Vec<&str> = vec!["blue", "cyan", "red", "purple", "yellow", "green", "rainbow"];
                            colors = colors.into_iter().filter(|color| color != &state.color.as_str()).collect::<Vec<&str>>(); 
                            let index = rng.gen_range(0..=colors.len() - 1);
                            state.color = colors[index].to_string();
                        }
                        KeyCode::Char('1') => state.speed = 120,
                        KeyCode::Char('2') => state.speed = 100,
                        KeyCode::Char('3') => state.speed = 80,
                        KeyCode::Char('4') => state.speed = 60,
                        KeyCode::Char('5') => state.speed = 50,
                        KeyCode::Char('6') => state.speed = 40,
                        KeyCode::Char('7') => state.speed = 30,
                        KeyCode::Char('8') => state.speed = 20,
                        KeyCode::Char('9') => state.speed = 10,
                        KeyCode::Char('0') => state.speed = 5,
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
