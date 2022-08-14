use clap::{ArgEnum, Parser, Subcommand};
use staff::{Chord, Key, Note, Scale};
use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(Parser)]
#[clap(author, version, about = "Music theory command-line interface", long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Display a chord's notes
    Chord {
        /// Name (symbol) of the chord
        name: String,
    },

    /// Display a scale's notes
    Scale {
        /// Root note of the scale
        root: String,

        /// Mode of the scale
        #[clap(arg_enum, value_parser)]
        mode: Mode,
    },

    /// Display the sharps or flats for a key
    Key {
        /// Root note of the key
        root: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Major,
    Minor,
    Ionian,
    Aeolian,
    Dorian,
}

type Result = anyhow::Result<()>;

fn print_notes<I, T>(notes: I) -> Result
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    let mut iter = notes.into_iter().peekable();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    while let Some(note) = iter.next() {
        write!(handle, "{}", note)?;
        if iter.peek().is_some() {
            write!(handle, " ")?;
        } else {
            writeln!(handle)?;
        }
    }
    Ok(())
}

fn main() -> Result {
    let cli = App::parse();
    match &cli.command {
        Command::Chord { name } => {
            let chord: Chord = name.parse().unwrap();
            print_notes(chord)
        }
        Command::Key { root } => {
            let note: Note = root.parse().unwrap();

            let key = Key::major(note.into());
            println!("{}", key);
            Ok(())
        }
        Command::Scale { root, mode } => {
            let note: Note = root.parse().unwrap();
            let scale = match mode {
                Mode::Major | Mode::Ionian => Scale::major(note),
                Mode::Minor | Mode::Aeolian => Scale::natural_minor(note),
                Mode::Dorian => Scale::dorian(note),
            };
            print_notes(scale)
        }
    }
}
