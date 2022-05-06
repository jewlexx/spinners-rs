use std::{thread, time::Duration};

use spinners_rs::Spinners;

use strum::IntoEnumIterator;

fn main() {
    let sps = Spinners::iter().collect::<Vec<Spinners>>();
    let len = sps.len();
    let mut sp = sps.get(0).unwrap().into_spinner().unwrap();
    sp.start();
    for i in 0..sps.len() {
        sp.set_message(format!("{}/{}", i, len));

        thread::sleep(Duration::from_millis(1000));
    }
}
