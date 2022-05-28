use std::process::Command;
use std::collections::HashMap;

use crate::config::MonitorPositions::{self, *};

pub fn get_monitor_names() -> Result<Vec<String>, std::io::Error> {
    let command = String::from_utf8(Command::new("xrandr")
        .output()?
        .stdout).expect("Xrandr is not utf8?");

    // Parse xrandr output for connected monitors
    Ok(command.lines().into_iter()
        .filter(|line| line.contains(" connected"))
        .map(|line| String::from(line.split(" ").collect::<Vec<&str>>()[0]))
        .collect())
}

// xrandr command: xrandr --output $MAINDISPLAY --auto --primary --output $SECONDARYDISPLAY --auto --left-of $MAINDISPLAY

pub fn update_outputs(primary_monitor: &String, secondary_monitors: &Option<HashMap<MonitorPositions, String>>) {

    if secondary_monitors.is_none() {
        // println!("Xrandr output: {:?}", Command::new("xrandr")
        //     .arg("--output")
        //     .arg(primary_monitor)
        //     .arg("--auto")
        //     .arg("--primary")
        //     .output()
        //     .expect("Xrandr primary monitor command failed")
        //     .stdout);
        println!("Xrandr output: {:?}", Command::new("xrandr") // TODO: Make this faster
            .arg("--auto")
            .output()
            .expect("Xrandr primary monitor command failed")
            .stdout);

        return;
    }

    let secondary_monitor_commands: Vec<String> = secondary_monitors
        .as_ref()
        .unwrap()
        .into_iter()
        .flat_map(|(position, name)| {
            let xrandr_position = match position {
                LeftOf => String::from("--left-of"),
                RightOf => String::from("--right-of"),
            };
            vec![String::from("--output"), name.to_string(), String::from("--auto"), xrandr_position, primary_monitor.to_string()]
        })
    .collect();

    println!("Xrandr output: {:?}", Command::new("xrandr")
        .arg("--output")
        .arg(primary_monitor)
        .arg("--auto")
        .arg("--primary")
        .args(secondary_monitor_commands)
        .output()
        .expect("Xrandr primary monitor command failed")
        .stdout);
}

