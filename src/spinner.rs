use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

use strum::Display;
use thiserror::Error as ThisError;

use crate::Spinners;

#[derive(Debug, Clone, ThisError, Display)]
pub enum Error {
    UnknownSpinner(String),
}

#[derive(Debug, Clone, Display)]
pub enum Event {
    Stop,
    SetMessage(String),
    SetInterval(u64),
    // SetFrames(Vec<char>),
}

#[derive(Debug, Clone)]
pub struct Spinner {
    pub spinner: Spinners,
    interval: u64,
    frames: Vec<char>,
    sender: Option<Sender<Event>>,
    message: String,
}

impl Spinners {
    pub fn into_spinner(self) -> Result<Spinner, Error> {
        Spinner::new(self, "")
    }
}

impl Spinner {
    pub fn new<S: Into<String>>(spinner: Spinners, message: S) -> Result<Self, Error> {
        let frames = spinner.get_frames();

        if let Ok(frames) = frames {
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
        if let Some(sender) = &self.sender {
            sender.send(Event::SetInterval(interval)).unwrap();
        }
    }

    pub fn set_message<S: Into<String>>(&mut self, message: S) {
        self.message = message.into();
        if let Some(sender) = &self.sender {
            sender
                .send(Event::SetMessage(self.message.clone()))
                .unwrap();
        }
    }

    pub fn start(&mut self) {
        let spinner = self.clone();

        let (sender, recv) = channel::<Event>();

        thread::spawn(move || 'outer: loop {
            let mut interval = spinner.interval;
            let mut message = spinner.message.clone();
            let frames = spinner.frames.clone();

            let mut stdout = stdout();

            for frame in frames.iter() {
                match recv.try_recv() {
                    Ok(Event::Stop) | Err(TryRecvError::Disconnected) => break 'outer,
                    Ok(Event::SetMessage(message_)) => message = message_,
                    Ok(Event::SetInterval(interval_)) => interval = interval_,
                    Err(TryRecvError::Empty) => {}
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
            sender.send(Event::Stop).unwrap();
        }
    }
}
