use super::*;
use std::process::Command;

/// Run swaymsg to corraborate output
fn swaymsg(command: &str) -> String {
    let output = Command::new("swaymsg")
        .arg("-r")
        .arg("-t")
        .arg(command)
        .output()
        .expect("Failed to execute swaymsg")
        .stdout;
    assert!(output.len() > 0, "swaymsg returned an empty string");
    String::from_utf8(output).expect("Invalid utf-8 in swaymsg output")
}

mod can {
    use super::*;

    #[test]
    fn run_command() {
        let mut socket = Socket::new().unwrap();
        socket.run_command("nop").unwrap();
    }

    #[test]
    fn get_workspaces() {
        let mut socket = Socket::new().unwrap();
        socket.get_workspaces().unwrap();
    }
    #[test]
    fn get_outputs() {
        let mut socket = Socket::new().unwrap();
        socket.get_outputs().unwrap();
    }

    #[test]
    fn get_tree() {
        let mut socket = Socket::new().unwrap();
        socket.get_tree().unwrap();
    }

    #[test]
    fn get_marks() {
        let mut socket = Socket::new().unwrap();
        socket.get_marks().unwrap();
    }

    #[test]
    fn get_bars() {
        let mut socket = Socket::new().unwrap();
        socket.get_bars().unwrap();
    }

    #[test]
    fn get_bar_config() {
        let mut socket = Socket::new().unwrap();
        let bars = socket.get_bars().unwrap();
        for bar in bars.iter() {
            socket.get_bar_config(bar).unwrap();
        }
    }

    #[test]
    fn get_version() {
        let mut socket = Socket::new().unwrap();
        socket.get_version().unwrap();
    }

    #[test]
    fn get_binding_modes() {
        let mut socket = Socket::new().unwrap();
        socket.get_binding_modes().unwrap();
    }

    #[test]
    fn get_config() {
        let mut socket = Socket::new().unwrap();
        socket.get_config().unwrap();
    }

    #[test]
    fn send_tick() {
        let mut socket = Socket::new().unwrap();
        socket.send_tick(None).unwrap();
    }

    #[test]
    fn get_binding_state() {
        let mut socket = Socket::new().unwrap();
        socket.get_binding_state().unwrap();
    }

    #[test]
    fn get_inputs() {
        let mut socket = Socket::new().unwrap();
        socket.get_inputs().unwrap();
    }

    #[test]
    fn get_seats() {
        let mut socket = Socket::new().unwrap();
        socket.get_seats().unwrap();
    }

}

mod corroborate {
    use super::*;

    #[test]
    fn workspaces() {
        let cmd_output = swaymsg("get_workspaces");
        let shell_res = serde_json::from_str::<Vec<Workspace>>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_workspaces().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn outputs() {
        let cmd_output = swaymsg("get_outputs");
        let shell_res = serde_json::from_str::<Vec<Output>>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_outputs().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn tree() {
        let cmd_output = swaymsg("get_tree");
        let shell_res = serde_json::from_str::<Node>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_tree().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn marks() {
        let cmd_output = swaymsg("get_marks");
        let shell_res = serde_json::from_str::<Vec<String>>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_marks().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn bars() {
        let cmd_output = swaymsg("get_bar_config");
        let shell_res = serde_json::from_str::<Vec<String>>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_bars().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn version() {
        let cmd_output = swaymsg("get_version");
        let shell_res = serde_json::from_str::<Version>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_version().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn binding_modes() {
        let cmd_output = swaymsg("get_binding_modes");
        let shell_res = serde_json::from_str::<Vec<String>>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_binding_modes().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn config() {
        let cmd_output = swaymsg("get_config");
        let shell_res = serde_json::from_str::<Config>(&cmd_output).unwrap().config;
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_config().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn binding_state() {
        let cmd_output = swaymsg("get_binding_state");
        let shell_res = serde_json::from_str::<BindingState>(&cmd_output).unwrap().name;
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_binding_state().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn inputs() {
        let cmd_output = swaymsg("get_inputs");
        let shell_res = serde_json::from_str::<Vec<Input>>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_inputs().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }

    #[test]
    fn seats() {
        let cmd_output = swaymsg("get_seats");
        let shell_res = serde_json::from_str::<Vec<Seat>>(&cmd_output).unwrap();
        let mut socket = Socket::new().unwrap();
        let lib_res = socket.get_seats().unwrap();
        assert!(lib_res == shell_res, "swaymsg output does not match lib output");
    }
}
