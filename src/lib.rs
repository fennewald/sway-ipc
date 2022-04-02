#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[cfg(feature = "pretty_errors")]
extern crate format_serde_error;

#[cfg(feature = "pretty_errors")]
use format_serde_error::SerdeError;

use serde::de::DeserializeOwned;
use std::env;
use std::fmt;
use std::io::{self, Write, Read};
use std::path::Path;
use std::os::unix::net::UnixStream;

#[cfg(test)]
mod tests;
mod types;
pub use types::*;

/// Possible Error types
#[cfg(feature = "pretty_errors")]
pub enum Error {
    IoError(io::Error),
    DeserializeError(SerdeError),
}

#[cfg(not(feature = "pretty_errors"))]
pub enum Error {
    IoError(io::Error),
    DeserializeError(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{:?}", e),
            Error::DeserializeError(e) => write!(f, "{}", e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum MessageType {
    RunCommand = 0,
    GetWorkspaces = 1,
    Subscribe = 2,
    GetOutputs = 3,
    GetTree = 4,
    GetMarks = 5,
    GetBarConfig = 6,
    GetVersion = 7,
    GetBindingModes = 8,
    GetConfig = 9,
    SendTick = 10,
    #[deprecated]
    Sync = 11,
    GetBindingState = 12,
    GetInputs = 100,
    GetSeats = 101,
}

pub struct Socket(UnixStream);

impl Socket {
    /// Create a new socket
    /// Gets path from environment
    pub fn new() -> io::Result<Socket> {
        match env::var("SWAYSOCK") {
            Ok(val) => Socket::connect(val),
            Err(_)  => Socket::connect(env!("SWAYSOCK")),
        }
    }

    /// Connect to sway server on provided path
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Socket> {
        let socket = UnixStream::connect(path)?;
        Ok(Socket(socket))
    }

    /// Send message
    fn send(&mut self, msg_type: MessageType, payload: Option<&str>) -> io::Result<()> {
        let mut header:
            [u8; 14] = [105, 51, 45, 105, 112, 99, 0, 0, 0, 0, 0, 0, 0, 0];
        header[10..14].copy_from_slice(&(msg_type as u32).to_ne_bytes());
        if let Some(content) = payload {
            header[6..10].copy_from_slice(&(content.len() as u32).to_ne_bytes());
            self.0.write_all(&header)?;
            self.0.write_all(&content.as_bytes())?;
        } else {
            self.0.write_all(&header)?;
        }
        Ok(())
    }

    /// Read message
    #[cfg(feature = "pretty_errors")]
    fn recv<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut header: [u8; 14] = [0; 14];
        self.0.read_exact(&mut header)?;
        let res_size = u32::from_ne_bytes((&header[6..10]).try_into().unwrap()) as usize;
        let mut message = vec![0; res_size];
        self.0.read_exact(&mut message)?;
        serde_json::from_slice::<T>(&message).map_err(|e| {
            Error::DeserializeError(SerdeError::new(String::from_utf8(message).unwrap(), e))
        })
    }

    #[cfg(not(feature = "pretty_errors"))]
    fn recv<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut header: [u8; 14] = [0; 14];
        self.0.read_exact(&mut header)?;
        let res_size = u32::from_ne_bytes((&header[6..10]).try_into().unwrap()) as usize;
        let mut message = vec![0; res_size];
        self.0.read_exact(&mut message)?;
        Ok(serde_json::from_slice::<T>(&message)?)
    }

    /// Send a message and receive a reply
    fn mesg<T: DeserializeOwned>(
        &mut self,
        msg_type: MessageType,
        payload: Option<&str>,
    ) -> Result<T> {
        self.send(msg_type, payload)?;
        self.recv::<T>()
    }

    pub fn run_command(
        &mut self,
        cmd: &str
    ) -> Result<Vec<std::result::Result<(), CommandError>>> {
        let results: Vec<CommandResult> = self.mesg(MessageType::RunCommand, Some(cmd))?;
        Ok(results.iter().map(|res| {
            if res.success {
                Ok(())
            } else if res.parse_error == Some(true) {
                Err(CommandError::ParseError)
            } else {
                Err(CommandError::Failed)
            }
        }).collect())
    }

    pub fn get_workspaces(&mut self) -> Result<Vec<Workspace>> {
        self.mesg(MessageType::GetWorkspaces, None)
    }
    pub fn get_outputs(&mut self) -> Result<Vec<Output>> {
        self.mesg(MessageType::GetOutputs, None)
    }
    pub fn get_tree(&mut self) -> Result<Node> {
        self.mesg(MessageType::GetTree, None)
    }
    pub fn get_marks(&mut self) -> Result<Vec<String>> {
        self.mesg(MessageType::GetMarks, None)
    }
    pub fn get_bars(&mut self) -> Result<Vec<String>> {
        self.mesg(MessageType::GetBarConfig, None)
    }
    pub fn get_bar_config(&mut self, id: &str) -> Result<Bar> {
        self.mesg(MessageType::GetBarConfig, Some(id))
    }
    pub fn get_version(&mut self) -> Result<Version> {
        self.mesg(MessageType::GetVersion, None)
    }
    pub fn get_binding_modes(&mut self) -> Result<Vec<String>> {
        self.mesg(MessageType::GetBindingModes, None)
    }
    pub fn get_config(&mut self) -> Result<String> {
        Ok(self.mesg::<Config>(MessageType::GetConfig, None)?.config)
    }
    pub fn send_tick(&mut self, payload: Option<&str>) -> Result<bool> {
        Ok(self.mesg::<CommandResult>(MessageType::SendTick, payload)?.success)
    }
    pub fn get_binding_state(&mut self) -> Result<String> {
        Ok(self.mesg::<BindingState>(MessageType::GetBindingState, None)?.name)
    }
    pub fn get_inputs(&mut self) -> Result<Vec<Input>> {
        self.mesg(MessageType::GetInputs, None)
    }
    pub fn get_seats(&mut self) -> Result<Vec<Seat>> {
        self.mesg(MessageType::GetSeats, None)
    }
}
