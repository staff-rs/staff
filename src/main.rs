use clap::{ArgEnum, Parser, Subcommand};
use staff::{
    midi::{MidiNote, Octave},
    note::{Accidental, Flat},
    Chord, Natural, Note, Pitch, Scale,
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
}

fn print_scale<A: Accidental>(root_note: Note<A>, mode: Mode) {
    let scale = match mode {
        Mode::Major | Mode::Ionian => Scale::major(root_note),
        Mode::Minor | Mode::Aeolian => Scale::natural_minor(root_note),
        _ => todo!(),
    };
    for note in scale {
        println!("{}", note);
    }
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
                    print_scale(root_note, *mode);
                }
                Some('#') => {
                    let root_note = match chars.next() {
                        Some('#') => Note::double_sharp(natural),
                        None => Note::sharp(natural),
                        _ => todo!(),
                    };
                    print_scale(root_note, *mode);
                }
                None => {
                    let root_note: Note<Flat> = natural.into();
                    print_scale(root_note, *mode);
                }
                _ => todo!(),
            };
        }
    }
}
