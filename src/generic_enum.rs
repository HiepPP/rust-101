enum Transmission<T> {
    Signal(T),
    NoSignal,
}

fn main() {
    let signal = Transmission::Signal(42);
    let no_signal = Transmission::NoSignal;
    let signal_value = match signal {
        Transmission::Signal(value) => value,
        Transmission::NoSignal => 0,
    };
    let no_signal_value = match no_signal {
        Transmission::Signal(value) => value,
        Transmission::NoSignal => 0,
    };
}
