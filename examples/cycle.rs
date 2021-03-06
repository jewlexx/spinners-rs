use std::{thread, time::Duration};

use spinners_rs::{Spinner, Spinners};

use strum::IntoEnumIterator;

fn main() {
    let sps = Spinners::iter().collect::<Vec<Spinners>>();
    let len = sps.len();
    let sp = sps.get(0).unwrap();
    let mut spinner: Spinner = (*sp).into();
    spinner.start();

    for (i, sp) in sps[..].iter().enumerate() {
        spinner.set_spinner(*sp);
        spinner.set_message(format!(
            " {:0>2}/{} {}                                                  ",
            i + 1,
            len + 1,
            spinner.get_name(),
        ));

        thread::sleep(Duration::from_millis(1000));
    }
}
