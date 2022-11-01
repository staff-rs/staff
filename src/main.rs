use clap::{arg, ArgEnum, Parser, Subcommand, ValueEnum};
use staff::{
    midi::{MidiNote, Octave},
    note::Accidental,
    Chord, Interval, Key, Note, Pitch, Scale,
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

#[derive(Clone, Copy, ValueEnum)]
enum Instrument {
    Guitar,
}

#[derive(Subcommand)]
enum Command {
    /// Display a chord's notes
    Chord {
        /// Name (symbol) of the chord
        name: String,
        /// Show guitar chord
        guitar: bool,
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
        Command::Chord { name, guitar } => {
            let chord: Chord = name.parse().unwrap();
            if *guitar {
                let midi_notes: Vec<_> = chord.into_iter().collect();

                for i in 0..16 {
                    let s = i.to_string();
                    // TODO handle other cases
                    print!("{}", i);
                    if s.len() < 2 {
                        print!(" ");
                    }
                    print!("| ");
                    for note in [
                        MidiNote::new(Pitch::E, Octave::FOUR),
                        MidiNote::new(Pitch::A, Octave::FOUR),
                        MidiNote::new(Pitch::D, Octave::FOUR),
                        MidiNote::new(Pitch::G, Octave::FOUR),
                        MidiNote::new(Pitch::B, Octave::FOUR),
                        MidiNote::new(Pitch::E, Octave::FIVE),
                    ] {
                        let mut s = String::new();
                        let n = note + Interval::new(i);
                        if midi_notes.iter().find(|pitch| **pitch == n.pitch()).is_some() {
                            s.push_str(&n.to_string());
                        }
                       

                        for _ in 0..5 - s.len() {
                            s.push(' ');
                        }

                        print!("{}", s);
                    }
                    println!();
                }
                Ok(())
            } else {
                print_notes(chord)
            }
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
