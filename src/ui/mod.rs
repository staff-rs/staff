use concoct::{
    web::{Html, Web},
    View,
};
use crate::midi::{MidiNote, MidiSet};

mod fret_diagram;
pub use fret_diagram::FretDiagram;

pub fn piano<E: 'static>(
    midi_set: MidiSet,
    keys_len: u8,
    on_click: impl FnMut(MidiNote, bool) -> E + Clone + 'static,
) -> impl View<Web<E>> {
    let keys = (0..keys_len)
        .into_iter()
        .map(move |x| {
            let mut on_click = on_click.clone();
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
                .on("click", move |_| on_click(midi, contains_midi));
            (midi, html)
        })
        .collect::<Vec<_>>();

    Html::ul().class("keys").view(keys)
}
