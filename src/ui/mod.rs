use crate::midi::{MidiNote, MidiSet};
use concoct::{
    web::{Html, Web},
    View,
};

mod fret_diagram;
pub use fret_diagram::FretDiagram;

pub struct Piano<F> {
    pub on_click: F,
    pub keys_len: u8,
    pub midi_set: MidiSet,
}

impl<F> Piano<F> {
    pub fn new(midi_set: MidiSet, on_click: F) -> Self {
        Self {
            on_click,
            keys_len: 12,
            midi_set,
        }
    }

    pub fn with_keys_len(mut self,keys_len: u8) -> Self {
        self.keys_len = keys_len;
        self
    }

    pub fn view<E>(self) -> impl View<Web<E>>
    where
        E: 'static,
        F: FnMut(MidiNote, bool) -> E + Clone + 'static,
    {
        let keys = (0..self.keys_len)
            .into_iter()
            .map(move |x| {
                let mut on_click = self.on_click.clone();
                let midi = MidiNote::from_byte(x);
                let contains_midi = self.midi_set.contains(midi);

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
}
