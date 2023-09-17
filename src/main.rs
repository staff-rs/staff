use std::panic;

use concoct::{
    view::{once, View},
    web::{class, on, Html, Web},
};
use staff::{
    chord::{chords, Chords},
    midi::{MidiNote, MidiSet},
    Note,
};

struct State {
    keys: MidiSet,
}

enum Event {
    Key(MidiNote, bool),
}

fn piano(midi_set: MidiSet) -> impl View<Web<Event>> {
    let keys = (0..29)
        .into_iter()
        .map(|x| {
            let midi = MidiNote::from_byte(x);
            let contains_midi = midi_set.contains(midi);

            let mut class_name = String::from("key");
            if !midi.pitch().is_natural() {
                class_name.push_str(" black")
            }
            if contains_midi {
                class_name.push_str(" selected");
            }

            let html = Html::li()
                .class(class_name)
                .on("click", move |_| Event::Key(midi, !contains_midi));
            (midi, html)
        })
        .collect::<Vec<_>>();

    Html::ul().class("keys").view(keys)
}

fn app(state: &State) -> impl View<Web<Event>> {
    let chords = chords(state.keys.clone().into_iter().collect::<Vec<_>>())
        .map(|chord| ((), Html::li().view(chord.to_string())))
        .collect::<Vec<_>>();
    (
        Html::ul().class("chords").view(chords),
        piano(state.keys.clone()),
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
                    state.keys.push(midi);
                } else {
                    state.keys.remove(midi);
                }
            }
        },
        app,
    );
}
