use anyhow::bail;
use clap::{ArgEnum, Parser, Subcommand};
use staff::{
    note::{Accidental, Flat},
    Chord, Natural, Note, Scale,
};
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

fn print_scale<A: Accidental>(root_note: Note<A>, mode: Mode) -> Result {
    let scale = match mode {
        Mode::Major | Mode::Ionian => Scale::major(root_note),
        Mode::Minor | Mode::Aeolian => Scale::natural_minor(root_note),
        Mode::Dorian => Scale::dorian(root_note),
    };
    print_notes(scale)
}

fn main() -> Result {
    let cli = App::parse();
    match &cli.command {
        Command::Chord { name } => {
            let chord: Chord = name.parse().unwrap();
            print_notes(chord)
        }
        Command::Scale { root, mode } => {
            let mut chars = root.chars();
            let natural: Natural = if let Some(c) = chars.next() {
                c.try_into().unwrap()
            } else {
                bail!("Missing root note")
            };

            match chars.next() {
                Some('b') => {
                    let root_note = match chars.next() {
                        Some('b') => Note::double_flat(natural),
                        Some(c) => bail!("Invalid character `{}`", c),
                        _ => Note::flat(natural),
                    };
                    print_scale(root_note, *mode)
                }
                Some('#') => {
                    let root_note = match chars.next() {
                        Some('#') => Note::double_sharp(natural),
                        Some(c) => bail!("Invalid character `{}`", c),
                        None => Note::sharp(natural),
                    };
                    print_scale(root_note, *mode)
                }
                Some(c) => bail!("Invalid character `{}`", c),
                None => {
                    let root_note: Note<Flat> = natural.into();
                    print_scale(root_note, *mode)
                }
            }
        }
    }
}
