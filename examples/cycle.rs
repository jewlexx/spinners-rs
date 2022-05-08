use std::{thread, time::Duration};

use spinners_rs::Spinners;

use strum::IntoEnumIterator;

// This exists so that shorter names work properly with the formatted output.
const WHITESPACE: &str = "                                                  ";

fn main() {
    let sps = Spinners::iter().collect::<Vec<Spinners>>();
    let len = sps.len();
    for i in 0..sps.len() {
        let mut sp = sps.get(i).unwrap().into_spinner().unwrap();
        sp.set_message(format!(
            " {:0>2}/{} {}{}",
            i + 1,
            len + 1,
            sp.spinner,
            WHITESPACE
        ));
        sp.start();

        thread::sleep(Duration::from_millis(1000));
    }
}
