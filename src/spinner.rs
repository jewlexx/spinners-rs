use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

use strum::Display;
use thiserror::Error as ThisError;

use crate::{Spinners, SPINNER_MAP};

#[derive(Debug, Clone)]
pub struct Spinner {
    pub spinner: Spinners,
    interval: u64,
    frames: Vec<char>,
    sender: Option<Sender<()>>,
    message: String,
}

#[derive(Debug, Clone, ThisError, Display)]
pub enum Error {
    UnknownSpinner(String),
}

impl Spinner {
    pub fn new<S: Into<String>>(spinner: Spinners, message: S) -> Result<Self, Error> {
        let frames = SPINNER_MAP.get(&spinner.to_string());

        if let Some(frames) = frames {
            let frames: Vec<char> = frames.chars().collect();

            Ok(Self {
                spinner,
                frames: frames.clone(),
                interval: 1000 / frames.len() as u64,
                message: message.into(),
                sender: None,
            })
        } else {
            Err(Error::UnknownSpinner(spinner.to_string()))
        }
    }

    pub fn set_interval(&mut self, interval: u64) {
        self.interval = interval;
    }

    pub fn start(&mut self) {
        let interval = self.interval;
        let frames = self.frames.clone();
        let message = self.message.clone();

        let (sender, recv) = channel::<()>();

        thread::spawn(move || 'outer: loop {
            let mut stdout = stdout();

            for frame in frames.iter() {
                match recv.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => break 'outer,
                    _ => {}
                };

                print!("\r{} {}", frame, message);
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(interval));
            }
        });

        self.sender = Some(sender);
    }

    pub fn stop(&self) {
        if let Some(sender) = &self.sender {
            sender.send(()).unwrap();
        }
    }
}
