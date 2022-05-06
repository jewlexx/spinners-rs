use std::{thread, time::Duration};

use spinners_rs::Spinners;

use strum::IntoEnumIterator;

fn main() {
    let sps = Spinners::iter().collect::<Vec<Spinners>>();
    let len = sps.len();
    for i in 0..sps.len() {
        let mut sp = sps.get(i).unwrap().into_spinner().unwrap();
        sp.set_message(format!("{}/{}", i, len));

        sp.start();

        thread::sleep(Duration::from_millis(1000));
    }
}
