use clap::Parser;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use matrix::{Direction, LineState, State};
use rand::{thread_rng, Rng};
use ratatui::{
    layout::Rect,
    prelude::{CrosstermBackend, Terminal},
};
use std::io::{stdout, Result};

#[derive(Parser)]
#[command(
    about = "Creates the matrix in the terminal. Use `c` to cycle colors, `0-9` to change speed, arrow keys to change direction, and `q` to quit."
)]
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
    #[arg(
        short,
        long,
        value_name = "DIRECTION",
        help = "Direction: up, down, left, or right"
    )]
    direction: Option<String>,
    #[arg(short, long, value_name = "BOLD", help = "Make the text bold")]
    bold: bool,
}

mod matrix;

fn main() -> Result<()> {
    // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let cli = Cli::parse();
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

    let direction = match cli.direction {
        Some(d) => match d.to_lowercase().as_str() {
            "up" => Direction::Up,
            "right" => Direction::Right,
            "left" => Direction::Left,
            _ => Direction::Down,
        },
        None => Direction::Down,
    };
    let bold = cli.bold;
    let mut state = if let Some(color) = cli.color.as_deref() {
        match color.to_lowercase().as_str() {
            "blue" => State {
                color: color.to_string(),
                speed,
                direction,
                bold,
            },
            "cyan" => State {
                color: color.to_string(),
                speed,
                direction,
                bold,
            },
            "red" => State {
                color: color.to_string(),
                speed,
                direction,
                bold,
            },
            "purple" => State {
                color: color.to_string(),
                speed,
                direction,
                bold,
            },
            "yellow" => State {
                color: color.to_string(),
                speed,
                direction,
                bold,
            },
            "rainbow" => State {
                color: color.to_string(),
                speed,
                direction,
                bold,
            },
            _ => State {
                color: "green".to_string(),
                speed,
                direction,
                bold,
            },
        }
    } else {
        State {
            color: "green".to_string(),
            speed,
            direction,
            bold,
        }
    };
    // Initialize ratatui and get terminal size
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut matrix: Vec<LineState> = Vec::new();
    matrix::create_matrix(&mut matrix, &mut terminal, &state)?;

    loop {
        // Only print matrix every other column
        // Looks better than using every column
        for line in matrix.iter_mut() {
            line.update_line();
        }

        // Draw the matrix after updating all lines
        terminal.draw(|frame| {
            let area = Rect::new(0, 0, frame.size().width, frame.size().height);
            if state.direction == Direction::Up || state.direction == Direction::Down {
                // Get the state of every other column
                for (i, col) in area.columns().enumerate().step_by(2) {
                    matrix::process_matrix_cols(i, col, frame, &mut matrix, &state);
                }
            } else {
                // Get the state of every other row
                for (i, row) in area.rows().enumerate() {
                    matrix::process_matrix_rows(i, row, frame, &mut matrix, &state);
                }
            }
        })?;

        if event::poll(std::time::Duration::from_millis(state.speed))? {
            match event::read()? {
                event::Event::Resize(_, _) => {
                    terminal.autoresize()?;
                    matrix::create_matrix(&mut matrix, &mut terminal, &state)?;
                }
                event::Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('b') => state.bold = !state.bold,
                            KeyCode::Char('c') => {
                                let mut rng = thread_rng();
                                let mut colors: Vec<&str> = vec![
                                    "blue", "cyan", "red", "purple", "yellow", "green", "rainbow",
                                ];
                                colors = colors
                                    .into_iter()
                                    .filter(|color| color != &state.color.as_str())
                                    .collect::<Vec<&str>>();
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
                            KeyCode::Up => {
                                if state.direction != Direction::Up {
                                    state.direction = Direction::Up;
                                    matrix::create_matrix(&mut matrix, &mut terminal, &state)?;
                                }
                            }
                            KeyCode::Down => {
                                if state.direction != Direction::Down {
                                    state.direction = Direction::Down;
                                    matrix::create_matrix(&mut matrix, &mut terminal, &state)?;
                                }
                            }
                            KeyCode::Left => {
                                if state.direction != Direction::Left {
                                    state.direction = Direction::Left;
                                    matrix::create_matrix(&mut matrix, &mut terminal, &state)?;
                                }
                            }
                            KeyCode::Right => {
                                if state.direction != Direction::Right {
                                    state.direction = Direction::Right;
                                    matrix::create_matrix(&mut matrix, &mut terminal, &state)?;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
