use std::{thread, time::Duration};

use colored::Colorize;
use spinners_rs::{Spinner, Spinners};

fn main() {
    let mut sp = Spinner::new(Spinners::Arrow, "Doing some things...");

    sp.start();

    thread::sleep(Duration::from_secs(3));
    sp.set_message("Doing other things...");
    thread::sleep(Duration::from_secs(3));
    sp.stop_with_symbol("✔".green());
}
