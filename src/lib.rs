use spinners::Spinner;

mod spinners;

pub fn spin(spinner: &Spinner) {
    let spinner = spinner.clone();
    let interval = spinner.interval;
    let frames = spinner.frames;
}
