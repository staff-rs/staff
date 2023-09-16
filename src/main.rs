use concoct::{
    view::{once, View},
    web::{on, Html, Web},
};

enum Instrument {
    Piano,
    Strings,
}

struct State {
    instrument: Instrument,
}

enum Event {
    Instrument(Instrument),
}

fn counter(state: &State) -> impl View<Web<Event>> {
    let label = match state.instrument {
        Instrument::Piano => "Piano",
        Instrument::Strings => "Strings",
    };

    (
        Html::h1((), label),
        once(Html::button(
            on("click", |_| Event::Instrument(Instrument::Piano)),
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
            instrument: Instrument::Piano,
        },
        |state, event| match event {
            Event::Instrument(instrument) => state.instrument = instrument,
        },
        counter,
    );
}
