use clap::{Parser, Subcommand};
use staff::{
    midi::{MidiNote, Octave},
    note::Flat,
    Chord, Natural, Note, Pitch,
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
    }
}
