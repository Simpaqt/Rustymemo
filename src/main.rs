// Create a function to  create files(new notes).
// Create a function to list existing notes
// Create a function to Delete notes
// Create a TUI graphics
// Create a fuzzy finder
// Create a struct for the filetypes

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use dirs;
use std::io::{self, Write};
use std::{
    fs::{self, File},
    path::Path,
    process::Command,
};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

struct Notes {
    id: u32,
    name: String,
    date: u32,
    filetype: String,
}

struct App {
    notes: Vec<String>,
    state: ListState,
    directory: String,
    input: String,
    mode: AppMode,
    pending_delete: bool,
}

#[derive(PartialEq)]
enum AppMode {
    Normal,
    Create,
}

impl App {
    fn new(directory: &str) -> App {
        let mut state = ListState::default();
        let notes = list_notes(directory).unwrap_or_default();
        // Select the first note if the list is non-empty
        if !notes.is_empty() {
            state.select(Some(0));
        }
        App {
            notes,
            state,
            directory: directory.to_string(),
            input: String::new(),
            mode: AppMode::Normal,
            pending_delete: false,
        }
    }

    fn refresh_notes(&mut self) {
        self.notes = list_notes(&self.directory).unwrap_or_default();
        // Maintain or set selection after refresh
        if !self.notes.is_empty() {
            if let Some(selected) = self.state.selected() {
                if selected >= self.notes.len() {
                    self.state.select(Some(self.notes.len() - 1));
                }
            } else {
                self.state.select(Some(0));
            }
        } else {
            self.state.select(None);
        }
    }
}

fn create_new_notes(file_path: &str) -> Result<(), std::io::Error> {
    fs::File::create(file_path)?;
    Ok(())
}

fn list_notes(directory: &str) -> Result<Vec<String>, std::io::Error> {
    let entries = fs::read_dir(directory)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    Ok(entries)
}

fn delete_note(file_path: &str) -> Result<(), std::io::Error> {
    fs::remove_file(file_path)?;
    Ok(())
}

fn get_notes_dir() -> String {
    dirs::home_dir()
        .map(|path| path.join("notes").to_str().unwrap().to_string())
        .unwrap_or_else(|| "notes".to_string())
}

fn run_tui(directory: &str) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(directory);

    loop {
        terminal.draw(|f| {
            let chunks = if app.mode == AppMode::Create {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                    .split(f.size())
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size())
            };

            let items: Vec<ListItem> = app
                .notes
                .iter()
                .map(|note| ListItem::new(note.as_str()).style(Style::default().fg(Color::White)))
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(format!(
                            "Notes (Mode: {})",
                            match app.mode {
                                AppMode::Normal => "Normal",
                                AppMode::Create => "Create",
                            }
                        ))
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut app.state);

            if app.mode == AppMode::Create {
                let input_block = Block::default()
                    .title("New Note Name")
                    .borders(Borders::ALL);
                let input = Paragraph::new(app.input.as_str()).block(input_block);
                f.render_widget(input, chunks[1]);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('i') => {
                        app.mode = AppMode::Create;
                        app.input.clear();
                        app.pending_delete = false;
                    }
                    KeyCode::Char('o') => {
                        if let Some(selected) = app.state.selected() {
                            if let Some(note) = app.notes.get(selected) {
                                let file_path = format!("{}/{}", app.directory, note);
                                disable_raw_mode()?;
                                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                Command::new("nvim").arg(&file_path).status()?;
                                enable_raw_mode()?;
                                execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                                terminal.clear()?;
                                app.pending_delete = false;
                            }
                        }
                    }
                    KeyCode::Char('d') => {
                        if app.pending_delete {
                            if let Some(selected) = app.state.selected() {
                                if let Some(note) = app.notes.get(selected) {
                                    let file_path = format!("{}/{}", app.directory, note);
                                    if delete_note(&file_path).is_ok() {
                                        app.refresh_notes();
                                    }
                                }
                            }
                            app.pending_delete = false;
                        } else {
                            app.pending_delete = true;
                        }
                    }
                    KeyCode::Char('j') => {
                        let selected = app.state.selected().unwrap_or(0);
                        if selected < app.notes.len() - 1 {
                            app.state.select(Some(selected + 1));
                        }
                        app.pending_delete = false;
                    }
                    KeyCode::Char('k') => {
                        let selected = app.state.selected().unwrap_or(0);
                        if selected > 0 {
                            app.state.select(Some(selected - 1));
                        }
                        app.pending_delete = false;
                    }
                    _ => {
                        app.pending_delete = false;
                    }
                },
                AppMode::Create => match key.code {
                    KeyCode::Enter => {
                        if !app.input.is_empty() {
                            let file_path = format!("{}/{}", app.directory, app.input);
                            if !Path::new(&file_path).exists() {
                                if create_new_notes(&file_path).is_ok() {
                                    app.refresh_notes();
                                }
                            }
                            app.input.clear();
                            app.mode = AppMode::Normal;
                        }
                    }
                    KeyCode::Esc => {
                        app.input.clear();
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let directory = get_notes_dir();
    fs::create_dir_all(&directory)?;
    run_tui(&directory)?;
    Ok(())
}
