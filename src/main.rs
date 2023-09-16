use concoct::{
    view::{once, View},
    web::{class, on, Html, Web},
};
use staff::{
    chord::{chords, Chords},
    midi::{MidiNote, MidiSet},
    Note,
};

enum Instrument {
    Piano { keys: MidiSet },
    Strings,
}

struct State {
    instrument: Instrument,
}

enum Event {
    Instrument(Instrument),
    Key(MidiNote, bool),
}

fn piano(midi_set: MidiSet) -> impl View<Web<Event>> {
    let keys = (0..12)
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

            let html = Html::li(
                (
                    class(class_name),
                    on("click", move |_| Event::Key(midi, !contains_midi)),
                ),
                (),
            );
            (midi, html)
        })
        .collect::<Vec<_>>();

    Html::ul(class("keys"), keys)
}

fn counter(state: &State) -> impl View<Web<Event>> {
    (
        if let Instrument::Piano { ref keys } = state.instrument {
            let chords = chords(keys.clone().into_iter().collect::<Vec<_>>())
                .map(|chord| ((), Html::li((), chord.to_string())))
                .collect::<Vec<_>>();
            Some((Html::ul((), chords), piano(keys.clone())))
        } else {
            None
        },
        once(Html::button(
            on("click", |_| {
                Event::Instrument(Instrument::Piano {
                    keys: MidiSet::default(),
                })
            }),
            "Piano",
        )),
        once(Html::button(
            on("click", |_| Event::Instrument(Instrument::Strings)),
            "Strings",
        )),
    )
}

fn main() {
    concoct::web::run(
        State {
            instrument: Instrument::Piano {
                keys: MidiSet::default(),
            },
        },
        |state, event| match event {
            Event::Instrument(instrument) => state.instrument = instrument,
            Event::Key(midi, is_set) => match state.instrument {
                Instrument::Piano { ref mut keys } => {
                    if is_set {
                        keys.push(midi);
                    } else {
                        keys.remove(midi);
                    }
                }
                _ => todo!(),
            },
        },
        counter,
    );
}
