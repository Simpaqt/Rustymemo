use crate::notes::list_notes;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::widgets::ListState;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Create,
    Search,
}

pub struct App {
    pub notes: Vec<String>,
    pub filtered_notes: Vec<String>,
    pub state: ListState,
    pub directory: String,
    pub input: String,
    pub mode: AppMode,
    pub pending_delete: bool,
    pub search_query: String,
    matcher: SkimMatcherV2,
}

impl App {
    pub fn new(directory: &str) -> App {
        let mut state = ListState::default();
        let notes = list_notes(directory).unwrap_or_default();
        let filtered_notes = notes.clone();
        if !notes.is_empty() {
            state.select(Some(0));
        }
        App {
            notes,
            filtered_notes,
            state,
            directory: directory.to_string(),
            input: String::new(),
            mode: AppMode::Normal,
            pending_delete: false,
            search_query: String::new(),
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn refresh_notes(&mut self) {
        self.notes = list_notes(&self.directory).unwrap_or_default();
        self.update_filtered_notes();
        let notes_to_check = if self.mode == AppMode::Search {
            &self.filtered_notes
        } else {
            &self.notes
        };

        if !notes_to_check.is_empty() {
            if let Some(selected) = self.state.selected() {
                if selected >= notes_to_check.len() {
                    self.state.select(Some(notes_to_check.len() - 1));
                }
            } else {
                self.state.select(Some(0));
            }
        } else {
            self.state.select(None);
        }
    }

    pub fn update_filtered_notes(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_notes = self.notes.clone();
        } else {
            self.filtered_notes = self
                .notes
                .iter()
                .filter_map(|note| {
                    self.matcher
                        .fuzzy_match(note, &self.search_query)
                        .map(|_| note.clone())
                })
                .collect();
        }

        // Reset selection when filter changes
        if !self.filtered_notes.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state.select(None);
        }
    }

    pub fn get_current_notes(&self) -> &Vec<String> {
        if self.mode == AppMode::Search {
            &self.filtered_notes
        } else {
            &self.notes
        }
    }
}
