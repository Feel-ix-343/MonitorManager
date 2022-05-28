use core::time;
use std::thread;
use config::Config;
// use clap::Parser;





/// Configuring the display outputs
mod config;

/// Get display settings and change display output
mod display;

fn main() { 
    let config_json = config::get_config_json("./settings.json"); // TODO: Change this to the install location
    let config = Config::get(config_json);

    println!("Config: {:#?}", config);
                                                                  
    // Tracking monitor change
    let mut previous_monitors: Vec<String> = vec![];
    loop {
        let monitors = display::get_monitor_names().expect("Xrandr error");
        let active_primary_monitor = &monitors[0];
        let active_secondary_monitors = monitors[1..].to_vec();

        println!("Monitors: {:?}", monitors);

        match config::WorkSpace::get(&config, &active_primary_monitor, &active_secondary_monitors) {
            Some(workspace) => {
                println!("{:#?}", workspace);
                // Only update the display if monitors have changed
                if monitors != previous_monitors {
                    display::update_outputs(&workspace.primary_monitor, &workspace.secondary_monitors);
                    for command in &config.switch_scripts {
                        match execute_monitor_switch_scripts(command) {
                            Ok(()) => (),
                            Err(e) => println!("Fail executing scripts, check the outputs; Error: {}", &e)
                        }
                    }
                }
            },
            None => println!("No workspace found. Availiable monitors are : {:?}", monitors)
        }


        let sleep_time = time::Duration::from_secs(config.reload_time);
        thread::sleep(sleep_time);

        previous_monitors = monitors;
    }
}

use std::process::Command;
fn execute_monitor_switch_scripts(commands: &Vec<String>) -> Result<(), std::io::Error> {
    for command in commands {
        println!("Command '{command}' output:\n{:?}", Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()?)
    }
    Ok(())
}



