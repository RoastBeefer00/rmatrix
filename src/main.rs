use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use clap::{Parser, Subcommand};
use ratatui::{
    layout::Rect, prelude::{CrosstermBackend, Terminal}, style::Style, text::{Line, Span, Text}, widgets::Paragraph
};
use std::io::{stdout, Result};
use matrix::{LineState, Cell};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "COLOR", help = "Available colors: blue, cyan, red, purple, yellow, green")]
    color: Option<String>,
    #[arg(short, long, value_name = "SPEED", help = "Speed: 1-10")]
    speed: Option<i8>,
}

mod matrix;

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Initialize ratatui and get terminal size
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let terminal_size = terminal.size().unwrap();
    let t_height = terminal_size.height;
    let t_width = terminal_size.width;
    terminal.clear()?;

    // Create new matrix where each column has its own state
    let mut matrix: Vec<LineState> = Vec::new(); 
    for _ in 0..t_width {
        matrix.push(LineState::new(t_height.into()));
    }

    loop {
        // Handle the terminal growing in size
        // (Currently don't care about terminal shrinking)
        // TODO: Move this to a diffirent file
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

        let matrix_height = matrix.first().unwrap().line.len() as u16;
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

        // Only print matrix every other column
        // Looks better than using every column
        // TODO: find a better way to do this
        let mut update = false;
        for line in &mut matrix {
            if update {
                line.update_line();
                update = false;
            } else {
                update = true;
            }
        }

        // Draw the matrix after updating all lines
        terminal.draw(|frame| {
            let area = Rect::new(0, 0, frame.size().width, frame.size().height);
            // Get the state of every column
            for (i, col) in area.columns().enumerate() {
                let line_state = matrix.get(i).unwrap();
                let lines: Vec<Line> = line_state.line.clone().into_iter().map(|cell| {
                    // Determine how to print each line
                    match cell {
                        Cell::Sym(sym) => match sym.white {
                            true => Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::White))),
                            false => {
                                if let Some(color) = cli.color.as_deref() {
                                    match color.to_lowercase().as_str() {
                                        "blue" => Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Blue))),
                                        "cyan" => Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Cyan))),
                                        "red" => Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Red))),
                                        "purple" => Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Magenta))),
                                        "yellow" => Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Yellow))),
                                        _ => Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Green))),
                                    }
                                } else {
                                    Line::from(Span::styled(sym.value, Style::default().fg(ratatui::style::Color::Green)))
                                }
                            }
                        },
                        Cell::Whitespace => Line::from(String::from(" ")),
                    }
                }).collect();
                // Render the line as a paragraph
                frame.render_widget(Paragraph::new(Text::from(lines)), col);
            }
        })?;

        // Poll duration determines how fast the matrix falls
        let speed = match cli.speed {
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
        if event::poll(std::time::Duration::from_millis(speed))? {
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
