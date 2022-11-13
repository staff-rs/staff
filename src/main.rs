use clap::{arg, Parser, Subcommand, ValueEnum};
use staff::{
    guitar::{Fretboard, STANDARD},
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
        names: Vec<String>,
        /// Show guitar chord
        #[arg(short, long)]
        guitar: bool,
        #[arg(short, long)]
        functions: bool,
        #[arg(long)]
        frets: bool,
    },

    /// Display a scale's notes
    Scale {
        /// Root note of the scale
        root: String,

        /// Mode of the scale
        #[arg(value_enum)]
        mode: Mode,
    },

    /// Display the sharps or flats for a key
    Key {
        /// Root note of the key
        root: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
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
        Command::Chord {
            names,
            guitar,
            functions,
            frets,
        } => {
            if *frets {
                let frets = names.iter().map(|s| s.parse::<u8>().ok());
                let midi_notes: Vec<_> = Fretboard::new(STANDARD, frets).collect();

                for i in 0..midi_notes.len() {
                    let chord = Chord::from_midi(midi_notes[i], midi_notes.iter().copied()).unwrap();
                    println!("{}", chord);
                }

                return Ok(());
            }

            for name in names {
                let chord: Chord = name.parse().unwrap();
                if *guitar {
                    let midi_notes: Vec<_> = chord.clone().midi_notes(Octave::FOUR).collect();
                    for i in 0..16 {
                        let s = i.to_string();
                        // TODO handle other cases
                        print!("{}", i);
                        if s.len() < 2 {
                            print!(" ");
                        }
                        print!("| ");

                        for note in STANDARD {
                            let mut s = String::new();
                            let n = note + Interval::new(i);

                            if let Some(note) =
                                midi_notes.iter().find(|note| note.pitch() == n.pitch())
                            {
                                if *functions {
                                    s.push_str(
                                        &(*note - MidiNote::new(chord.root(), Octave::FOUR))
                                            .to_string(),
                                    );
                                } else {
                                    s.push_str(&n.to_string());
                                }
                            }

                            for _ in 0..5 - s.len() {
                                s.push(' ');
                            }

                            print!("{}", s);
                        }
                        println!();
                    }
                } else {
                    print_notes(chord)?;
                }
            }
            Ok(())
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
