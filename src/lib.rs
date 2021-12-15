#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::de::DeserializeOwned;

use std::env;
use std::io::{Write, Read};
use std::os::unix::net::UnixStream;
use std::convert::TryInto;

pub mod types;
pub use types::*;

pub trait IPC {
    fn run_command(&mut self, payload: String) -> Vec<Result<(), String>>;
    fn get_workspaces(&mut self) -> Vec<Workspace>;
    fn get_outputs(&mut self) -> Vec<Output>;
    fn get_tree(&mut self) -> Node;
    fn get_marks(&mut self) -> Vec<String>;
    fn get_bars(&mut self) -> Vec<String>;
    fn get_bar_config(&mut self, id: String) -> Bar;
    fn get_version(&mut self) -> Version;
    fn get_binding_modes(&mut self) -> Vec<String>;
    fn get_config(&mut self) -> String;
    fn send_tick(&mut self, payload: Option<String>) -> bool;
    fn get_binding_state(&mut self) -> String;
    fn get_inputs(&mut self) -> Vec<Input>;
    fn get_seats(&mut self) -> Vec<Seat>;
}

pub struct Socket(UnixStream);

impl Socket {
    pub fn new() -> Socket {
        match env::var("SWAYSOCK") {
            Ok(val) => Socket(UnixStream::connect(val).unwrap()),
            Err(_)  => Socket(UnixStream::connect(env!("SWAYSOCK")).unwrap()),
        }
    }

    fn message<O: DeserializeOwned>(&mut self, msg_type: u32, payload: Option<String>) -> O {
        self.send(msg_type, payload);
        return self.read::<O>();
    }

    fn send(&mut self, msg_type: u32, payload: Option<String>) {
        let mut header:
            [u8; 14] = [105, 51, 45, 105, 112, 99, 0, 0, 0, 0, 0, 0, 0, 0];
        header[10..14].copy_from_slice(&msg_type.to_ne_bytes());
        if let Some(content) = payload {
            header[6..10].copy_from_slice(&(content.len() as u32).to_ne_bytes());
            self.0.write_all(&header).unwrap();
            self.0.write_all(&content.into_bytes()).unwrap();
        } else {
            self.0.write_all(&header).unwrap();
        }
    }

    fn read<O: DeserializeOwned>(&mut self) -> O {
        let mut header: [u8; 14] = [0; 14];
        self.0.read_exact(&mut header).unwrap();
        // let res_type = u32::from_ne_bytes((&header[10..14]).try_into().unwrap());
        let res_size = u32::from_ne_bytes((&header[6..10]).try_into().unwrap()) as usize;
        let mut message = vec![0; res_size];
        self.0.read_exact(&mut message).unwrap();
        return serde_json::from_slice(&message).unwrap();
    }
}

impl IPC for Socket {
    fn run_command(&mut self, payload: String) -> Vec<Result<(), String>> {
        let raw = self.message::<Vec<CommandResult>>(0, Some(payload));
        let mut res: Vec<Result<(), String>> = vec![Ok(()); raw.len()];
        for i in 0..raw.len() {
            if raw[i].success {
                res[i] = Ok(());
            } else if let Some(msg) = raw[i].error.clone() {
                res[i] = Err(msg);
            } else {
                res[i] = Err("No error message provided".to_string());
            }
        }
        return res;
    }
    fn get_workspaces(&mut self) -> Vec<Workspace> {
        return self.message::<Vec<Workspace>>(1, None);
    }
    fn get_outputs(&mut self) -> Vec<Output> {
        return self.message::<Vec<Output>>(3, None);
    }
    fn get_tree(&mut self) -> Node {
        return self.message::<Node>(4, None);
    }
    fn get_marks(&mut self) -> Vec<String> {
        return self.message::<Vec<String>>(5, None);
    }
    fn get_bars(&mut self) -> Vec<String> {
        return self.message::<Vec<String>>(6, None);
    }
    fn get_bar_config(&mut self, id: String) -> Bar {
        return self.message::<Bar>(6, Some(id));
    }
    fn get_version(&mut self) -> Version {
        return self.message::<Version>(7, None);
    }
    fn get_binding_modes(&mut self) -> Vec<String> {
        return self.message::<Vec<String>>(8, None);
    }
    fn get_config(&mut self) -> String {
        return self.message::<Config>(9, None).config;
    }
    fn send_tick(&mut self, payload: Option<String>) -> bool {
        return self.message::<CommandResult>(10, payload).success;
    }
    fn get_binding_state(&mut self) -> String {
        return self.message::<BindingState>(12, None).name;
    }
    fn get_inputs(&mut self) -> Vec<Input> {
        return self.message::<Vec<Input>>(100, None);
    }
    fn get_seats(&mut self) -> Vec<Seat> {
        return self.message::<Vec<Seat>>(101, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn swaymsg(command: &str) -> String {
        let out = Command::new("swaymsg")
            .arg("-r")
            .arg("-t").arg(command)
            .output()
            .expect("Failed executing swaymsg")
            .stdout;
        assert!(out.len() > 0, "swaymsg returned an empty string");
        return std::str::from_utf8(&out.to_owned()).expect("Invalid utf8 in swaymsg").to_string();
    }
    // Can do all commands
    #[test]
    fn can_run_command() {
        let mut socket = Socket::new();
        socket.run_command("nop".to_string());
    }
    #[test]
    fn can_get_workspaces() {
        let mut socket = Socket::new();
        socket.get_workspaces();
    }
    #[test]
    fn can_get_outputs() {
        let mut socket = Socket::new();
        socket.get_outputs();
    }
    #[test]
    fn can_get_tree() {
        let mut socket = Socket::new();
        socket.get_tree();
    }
    #[test]
    fn can_get_marks() {
        let mut socket = Socket::new();
        socket.get_marks();
    }
    #[test]
    fn can_get_bars() {
        let mut socket = Socket::new();
        socket.get_bars();
    }
    #[test]
    fn can_get_bar_config() {
        let mut socket = Socket::new();
        let bars = socket.get_bars();
        for bar in bars.iter() {
            socket.get_bar_config(bar.to_string());
        }
    }
    #[test]
    fn can_get_version() {
        let mut socket = Socket::new();
        socket.get_version();
    }
    #[test]
    fn can_get_binding_modes() {
        let mut socket = Socket::new();
        socket.get_binding_modes();
    }
    #[test]
    fn can_get_config() {
        let mut socket = Socket::new();
        socket.get_config();
    }
    #[test]
    fn can_send_tick() {
        let mut socket = Socket::new();
        socket.send_tick(None);
    }
    #[test]
    fn can_get_binding_state() {
        let mut socket = Socket::new();
        socket.get_binding_state();
    }
    #[test]
    fn can_get_inputs() {
        let mut socket = Socket::new();
        socket.get_inputs();
    }
    #[test]
    fn can_get_seats() {
        let mut socket = Socket::new();
        socket.get_seats();
    }

    #[test]
    fn match_workspaces() {
        let cmd_output = swaymsg("get_workspaces");
        let shell_res = serde_json::from_str::<Vec<Workspace>>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_workspaces();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_outputs() {
        let cmd_output = swaymsg("get_outputs");
        let shell_res = serde_json::from_str::<Vec<Output>>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_outputs();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_tree() {
        let cmd_output = swaymsg("get_tree");
        let shell_res = serde_json::from_str::<Node>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_tree();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_marks() {
        let cmd_output = swaymsg("get_marks");
        let shell_res = serde_json::from_str::<Vec<String>>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_marks();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_bars() {
        let cmd_output = swaymsg("get_bar_config");
        let shell_res = serde_json::from_str::<Vec<String>>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_bars();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_version() {
        let cmd_output = swaymsg("get_version");
        let shell_res = serde_json::from_str::<Version>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_version();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_binding_modes() {
        let cmd_output = swaymsg("get_binding_modes");
        let shell_res = serde_json::from_str::<Vec<String>>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_binding_modes();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_config() {
        let cmd_output = swaymsg("get_config");
        let shell_res = serde_json::from_str::<Config>(&cmd_output).unwrap().config;
        let mut socket = Socket::new();
        let lib_res = socket.get_config();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_binding_state() {
        let cmd_output = swaymsg("get_binding_state");
        let shell_res = serde_json::from_str::<BindingState>(&cmd_output).unwrap().name;
        let mut socket = Socket::new();
        let lib_res = socket.get_binding_state();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_inputs() {
        let cmd_output = swaymsg("get_inputs");
        let shell_res = serde_json::from_str::<Vec<Input>>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_inputs();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
    #[test]
    fn match_seats() {
        let cmd_output = swaymsg("get_seats");
        let shell_res = serde_json::from_str::<Vec<Seat>>(&cmd_output).unwrap();
        let mut socket = Socket::new();
        let lib_res = socket.get_seats();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
}

