use concoct::{
    view::{self, View},
    web::{Html, Web},
};
use staff::{
    chord::chords,
    midi::{MidiNote, MidiSet},
    ui::Piano,
};
use std::panic;

struct State {
    keys: MidiSet,
}

enum Event {
    Key(MidiNote, bool),
}

fn app(state: &State) -> impl View<Web<Event>> {
    let chords: Vec<_> = chords(state.keys.clone().into_iter().collect::<Vec<_>>())
        .map(|chord| ((), Html::li().view(chord.to_string())))
        .collect();

    (
        view::once(
            Html::header().view((
                Html::h1().view("Chord Analyzer"),
                Html::div().view((
                    Html::span().view("Build with "),
                    Html::a()
                        .attr("href", "https://github.com/matthunz/staff")
                        .view("Staff"),
                    Html::span().view(" and "),
                    Html::a()
                        .attr("href", "https://github.com/concoct-rs/concoct")
                        .view("Concoct"),
                )),
            )),
        ),
        Piano::new(state.keys.clone(), |midi, contains_midi| {
            Event::Key(midi, contains_midi)
        })
        .with_keys_len(29)
        .view(),
        Html::ul().class("chords").view(chords),
    )
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    concoct::web::run(
        State {
            keys: MidiSet::default(),
        },
        |state, event| match event {
            Event::Key(midi, is_set) => {
                if is_set {
                    state.keys.remove(midi);
                } else {
                    state.keys.push(midi);
                }
            }
        },
        app,
    );
}
