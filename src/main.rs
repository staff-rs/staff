use clap::{Parser, Subcommand};
use staff::{
    midi::{MidiNote, Octave},
    note::Flat,
    Chord, Natural, Note, Pitch, Scale,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
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
    Scale {
        root: String,
        mode: String,
    },
}

fn main() {
    let cli = App::parse();
    match &cli.command {
        Command::Chord { name } => {
            let chord: Chord = name.parse().unwrap();
            for note in chord {
                println!("{}", note);
            }
        }
        Command::Scale { root, mode } => {
            let mut chars = root.chars();
            let natural = match chars.next().unwrap() {
                'A' => Natural::A,
                'B' => Natural::B,
                'C' => Natural::C,
                'D' => Natural::D,
                'E' => Natural::E,
                'F' => Natural::F,
                'G' => Natural::G,
                _ => todo!(),
            };

            match chars.next() {
                Some('b') => {
                    let root_note = match chars.next() {
                        Some('b') => Note::double_flat(natural),
                        None => Note::flat(natural),
                        _ => todo!(),
                    };
                    for note in Scale::major(root_note) {
                        println!("{}", note);
                    }
                }
                Some('#') => {
                    let root_note = match chars.next() {
                        Some('#') => Note::double_sharp(natural),
                        None => Note::sharp(natural),
                        _ => todo!(),
                    };
                    for note in Scale::major(root_note) {
                        println!("{}", note);
                    }
                }
                None => {
                    let root_note: Note<Flat> = natural.into();
                    for note in Scale::major(root_note) {
                        println!("{}", note);
                    }
                }
                _ => todo!(),
            };
        }
    }
}
