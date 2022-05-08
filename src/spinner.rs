use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

use strum::Display;

use crate::Spinners;

#[derive(Debug, Clone, Display)]
// TODO: Add mutex to the spinner interval and message so it can be updated from the thread
/// All the different events that can occur
pub enum Event {
    /// The spinner has finished.
    Stop,
    /// Update the spinner message
    SetMessage(String),
    /// Update the spinner interval
    SetInterval(u64),
    // SetFrames(Vec<char>),
}

#[derive(Debug, Clone)]
/// Main spinner struct
///
/// This holds all the information for the actual spinners
pub struct Spinner {
    /// The enum variant used in this spinner
    pub spinner: Spinners,
    interval: u64,
    frames: Vec<&'static str>,
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
    /// use spinners_rs::{Spinners, Spinner};
    ///
    /// let mut sp = Spinner::new(Spinners::Dots, "Doing some cool things...");
    /// sp.start();
    /// ```
    ///
    /// ## No Message:
    ///
    /// ```
    /// use spinners_rs::{Spinners, Spinner};
    ///
    /// let mut sp: Spinner = Spinners::Dots.into();
    /// sp.start();
    /// ```
    pub fn new<T, S>(spinner: T, message: S) -> Self
    where
        T: Into<Spinners> + Copy,
        S: std::fmt::Display,
    {
        let spinner_type: Spinners = spinner.into();
        let frames = spinner_type.get_frames();

        Self {
            spinner: spinner.into(),
            frames: frames.clone(),
            interval: 1000 / frames.len() as u64,
            message: message.to_string(),
            sender: None,
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
    /// Alternatively you can use the [`Spinner::stop_with_message`] or [`Spinner::stop_with_symbol`] function.
    ///
    /// # Example:
    ///
    /// ```
    /// use spinners_rs::{Spinners, Spinner};
    /// use std::{thread, time::Duration};
    ///
    /// let mut sp: Spinner = Spinners::Dots.into();
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

    /// Stops the spinner and replaces it with the given message
    ///
    /// # Example:
    ///
    /// ```
    /// use spinners_rs::{Spinners, Spinner};
    /// use std::{thread, time::Duration};
    ///
    /// let mut sp: Spinner = Spinners::Dots.into();
    /// sp.start();
    ///
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.stop_with_message("We've finished that thing!");
    /// ```
    pub fn stop_with_message<S: std::fmt::Display>(&self, message: S) {
        if let Some(sender) = &self.sender {
            sender.send(Event::Stop).unwrap();
            print!("\r{}", message);
            stdout().flush().unwrap();
        }
    }

    /// Stops the spinner and replaces the current frame with the given symbol
    ///
    /// # Example:
    ///
    /// ```
    /// use spinners_rs::{Spinners, Spinner};
    /// use std::{thread, time::Duration};
    ///
    /// let mut sp: Spinner = Spinners::Dots.into();
    /// sp.start();
    ///
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.stop_with_symbol('✓');
    /// ```
    pub fn stop_with_symbol<S: std::fmt::Display>(&self, symbol: S) {
        if let Some(sender) = &self.sender {
            sender.send(Event::Stop).unwrap();
            print!("\r{} {}", symbol, self.message);
            stdout().flush().unwrap();
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
    /// use spinners_rs::{Spinners, Spinner};
    /// use std::{thread, time::Duration};
    ///
    /// let mut sp: Spinner = Spinners::Dots.into();
    /// sp.start();
    ///
    /// // Will run through one iteration of frames
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.set_interval(500);
    ///
    /// // Will now run through two iterations of the frames
    /// thread::sleep(Duration::from_millis(1000));
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
    /// use spinners_rs::{Spinners, Spinner};
    /// use std::{thread, time::Duration};
    ///
    /// let mut sp: Spinner = Spinners::Dots.into();
    /// sp.start();
    ///
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.set_message("Doing some cool things...");
    ///
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// sp.stop();
    /// ```
    pub fn set_message<S: std::fmt::Display>(&mut self, message: S) {
        self.message = message.to_string();
        if let Some(sender) = &self.sender {
            sender
                .send(Event::SetMessage(self.message.clone()))
                .unwrap();
        }
    }

    /// Gets the spinner name capitalizes the first letter.
    ///
    /// # Example:
    ///
    /// ```
    /// use spinners_rs::{Spinners, Spinner};
    ///
    /// let sp: Spinner = Spinners::Dots.into();
    /// assert_eq!(sp.get_name(), "Dots");
    /// ```
    pub fn get_name(&self) -> String {
        let sp_string = self.spinner.to_string().chars().collect::<Vec<char>>();

        sp_string[0].to_uppercase().to_string()
            + sp_string[1..].iter().cloned().collect::<String>().as_str()
    }
}

impl From<Spinners> for Spinner {
    fn from(spinner: Spinners) -> Self {
        Spinner::new(spinner, "")
    }
}
