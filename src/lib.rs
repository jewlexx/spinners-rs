use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, TryRecvError},
    thread,
    time::Duration,
};

use spinners::Spinner;

mod spinners;

