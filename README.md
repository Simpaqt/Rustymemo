# About

Rustymemo is a fast and intuitive CLI note-taking tool that lets you manage notes from anywhere in your terminal. Built with Rust and featuring a modern TUI interface, it's designed for developers who want quick access to their notes without leaving the command line.

## ✨ Features

- **📝 Quick Note Creation**: Create notes instantly from any directory
- **🔍 Fuzzy Search**: Find notes quickly with real-time fuzzy matching
- **⚡ Fast Navigation**: Vim-like keybindings for efficient navigation
- **🎨 Modern UI**: Clean terminal interface with helpful status bar
- **📁 Organized Storage**: All notes saved to `~/notes` directory
- **🔧 Editor Integration**: Opens notes in your preferred editor (nvim by default)

### What you can do:

1. **Create** new notes that are saved to `~/notes`
2. **Search** through notes with fuzzy matching
3. **Edit** existing notes in nvim
4. **Delete** notes with confirmation
5. **Navigate** efficiently with vim-like keybindings

# Installation

Currently the only way to install is with cargo

### Install Rust and cargo using rustup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Once installed


```
git clone https://github.com/Simpaqt/Rustymemo.git

cd Rustymemo/

cargo build --release

mv target/release/Rustymemo ~/.local/bin/
```

## 🚀 Usage

Rustymemo features an intuitive interface with helpful status bar hints. Here are the key bindings:

### Navigation
- **`j` / `k`** - Move up and down through notes
- **`↑` / `↓`** - Alternative navigation (in search mode)

### Actions
- **`i`** - Create a new note
- **`o`** - Open selected note in nvim
- **`/`** - Enter search mode for fuzzy finding
- **`dd`** - Delete note (press 'd' twice for confirmation)
- **`q`** - Quit application

### Search Mode
- **Type** - Filter notes with fuzzy matching
- **`Enter`** - Open selected note from search results
- **`Esc`** - Exit search mode

### Create Mode
- **Type** - Enter note name
- **`Enter`** - Create the note
- **`Esc`** - Cancel creation

The status bar at the bottom always shows available keybindings for the current mode!

### Showcase


https://github.com/user-attachments/assets/cd427e40-0597-4ce6-a75a-1e7b30940f23
