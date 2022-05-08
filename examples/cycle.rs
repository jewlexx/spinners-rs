use std::{thread, time::Duration};

use spinners_rs::{Spinner, Spinners};

use strum::IntoEnumIterator;

// This exists so that shorter names work properly with the formatted output.
const WHITESPACE: &str = "                                                  ";

fn main() {
    let sps = Spinners::iter().collect::<Vec<Spinners>>();
    let len = sps.len();
    for (i, sp) in sps.iter().enumerate() {
        let mut sp: Spinner = (*sp).into();
        sp.set_message(format!(
            " {:0>2}/{} {}{}",
            i + 1,
            len + 1,
            sp.get_name(),
            WHITESPACE
        ));
        sp.start();

        thread::sleep(Duration::from_millis(1000));
    }
}
