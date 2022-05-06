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

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Spinner {
    /// Create a new spinner along with a message
    ///
    /// # Examples
    ///
    /// ## Basic Usage:
    ///
    /// ```
    /// use spinners_rs::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new(Spinners::Dots, "Doing some cool things...").unwrap();
    /// sp.start();
    /// ```
    ///
    /// ## No Message:
    ///
    /// ```
    /// use spinners_rs::Spinners;
    ///
    /// let sp = Spinners::Dots.into_spinner().unwrap();
    /// sp.start();
    /// ```
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

    /// Start the spinner
    ///
    /// Explained more in depth in the [`Spinner::new`] function.
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

    /// Stops the spinner from running
    ///
    /// # Example:
    ///
    /// ```
    /// use spinners_rs::Spinners;
    ///
    /// let sp = Spinners::Dots.into_spinner().unwrap();
    /// sp.start();
    ///
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.stop();
    /// ```
    pub fn stop(&self) {
        if let Some(sender) = &self.sender {
            sender.send(Event::Stop).unwrap();
        }
    }

    /// Updates the frame interval
    ///
    /// This changes how fast each frame comes up
    ///
    /// This can be changed before the spinner is started or after
    ///
    /// # Example:
    ///
    /// ```
    /// use spinners_rs::Spinners;
    ///
    /// let sp = Spinners::Dots.into_spinner().unwrap();
    /// sp.start();
    ///
    /// // Will run through one iteration of frames
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.set_interval(500);
    ///
    /// // Will now run through two iterations of the frames
    /// thread::sleep(Duration::from_mills(1000));
    ///
    /// sp.stop();
    /// ```
    pub fn set_interval(&mut self, interval: u64) {
        self.interval = interval;
        if let Some(sender) = &self.sender {
            sender.send(Event::SetInterval(interval)).unwrap();
        }
    }

    /// Sets the message to display
    ///
    /// Similar to [`Spinner::set_interval`], this can be set before or after a spinner is started
    ///
    /// # Example :
    ///
    /// ```
    /// use spinners_rs::Spinners;
    ///
    /// let sp = Spinners::Dots.into_spinner().unwrap();
    /// sp.start();
    ///
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.set_message("Doing some cool things...");
    ///
    /// thread::sleep(Duration::from_mills(1000));
    ///
    /// sp.stop();
    /// ```
    pub fn set_message<S: Into<String>>(&mut self, message: S) {
        self.message = message.into();
        if let Some(sender) = &self.sender {
            sender
                .send(Event::SetMessage(self.message.clone()))
                .unwrap();
        }
    }
}

impl Spinners {
    pub fn into_spinner(self) -> Result<Spinner, Error> {
        Spinner::new(self, "")
    }
}
