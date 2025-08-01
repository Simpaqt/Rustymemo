# About

Rustymemo is a simple cli to allow for notetaking no matter what directory you are in. This tool was made for personal use however if you have a suggestion or want to make a pr feel free to do so.

With Rustymemo you can:

1. Make a new note that will be saved to ~/notes
2. Edit existing notes
3. Delete notes

Once you have made a note (a text or md file) You open it with nvim.

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

### Usage

Rustymemo operates on neovim bindings.

- j & k for up and down
- i for new note
- o for open (opens the file in neovim)
- dd for new line
