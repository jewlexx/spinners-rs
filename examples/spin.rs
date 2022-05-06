use std::{thread, time::Duration};

use spinners_rs::{Spinner, Spinners};

fn main() {
    let mut sp = Spinner::new(Spinners::Arrow, "yo").unwrap();

    sp.start();

    thread::sleep(Duration::from_secs(3));
}
