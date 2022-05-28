use std::collections::HashMap;
use serde_json::from_reader;
use std::fs::File;
use std::env;
use serde_json::Value;

use crate::display;



#[derive(Debug)]
pub struct WorkSpace {
    pub name: String,
    pub primary_monitor: String,
    pub secondary_monitors: Option<HashMap<MonitorPositions, String>>
}

impl WorkSpace {
    /// Gets a configured workspace from the list of monitors
    /// Expects that workspaces are listed in the order or specificity. Ex. Workspace with one the
    /// monitor should be listed last
    pub fn get<'config>(config: &'config Config, primary_monitor: &String, secondary_monitors: &Vec<String>) -> Option<&'config WorkSpace> {
        let workspaces = &config.workspaces;
        for workspace in workspaces {
            if &workspace.primary_monitor == primary_monitor {
                if let Some(monitors) = workspace.get_secondary_monitor_list() {
                    // All monitors are contained 
                    if monitors.iter().fold(true, |a, m| a && secondary_monitors.contains(m)) { return Some(workspace) }
                }
                else { return Some(workspace); }
            }
        }
        return None;
    }

    pub fn get_secondary_monitor_list(&self) -> Option<Vec<&String>> {
        self.secondary_monitors
            .as_ref()
            .map(|map| {
            Some(map.iter()
                .flat_map(|(_, name)| vec![name])
                .collect::<Vec<&String>>())
        }).unwrap_or(None)
    }
    /// Tries to create a workspace object from a json Value. Will panic. Used by the
    /// config::Config::get. Will also replace an auto value for primary-monitor with the current active monitor
    pub fn create(value: &Value) -> WorkSpace {
        WorkSpace { // TODO: Make the workspaced not have to be in order
            name: String::from(value.get("name")
                .expect("WorkSpace object must have name")
                .as_str()
                .expect("WorkSpace name must be a json string")),
            primary_monitor: String::from(value.get("primary-monitor")
                .expect("WorkSpace object must have name")
                .as_str()
                .expect("WorkSpace name must be a json string"))
                .replace("auto", display::get_monitor_names()
                         .expect("Xrandr error getting active monitor")
                         .get(0)
                         .expect("No monitors exist")),
            secondary_monitors: value.get("secondary-monitors") // TODO: Add an auto feature
                .map(|value| value
                    .as_array()
                    .expect("secondary-monitors must be a json aray")
                    .iter()
                    .flat_map(|secondary_monitor_value| vec![
                        (
                            MonitorPositions::get(secondary_monitor_value.get("position")
                                .expect("SecondaryMonitor object must have a position")
                                .as_str()
                                .expect("SecondaryMonitor position must be a string"))
                            .expect("SecondarMonitor position must be of the right type"),
                            String::from(secondary_monitor_value.get("monitor-name")
                                .expect("SecondaryMonitor object must have a monitor-name")
                                .as_str()
                                .expect("monitor-name must be a string"))
                        )
                    ])
                    .collect()
                )
        }
    }
}


#[derive(Debug, Hash, Eq, PartialEq)]
pub enum MonitorPositions {
    RightOf,
    LeftOf
}
impl MonitorPositions {
    pub fn get(position: &str) -> Result<MonitorPositions, ()> {
        match position {
            "rightOf" => Ok(MonitorPositions::RightOf),
            "leftOf" => Ok(MonitorPositions::LeftOf),
            _ => Err(())
        }
    }
}

pub fn get_config_json(default_config_path: &str) -> Value {
    let home_dir = env::var("HOME").expect("Could not find home dir");
    let config_file = match File::open(format!("{home_dir}/.config/monitor_manager/settings.json")) {
        Ok(file) => file,
        Err(_) => File::open(default_config_path).expect("Could not find config file")
    };
    from_reader(config_file).expect("Failed to parse config json; Possible error in config file")
}

#[derive(Debug)]
pub struct Config {
    pub reload_time: u64,
    pub workspaces: Vec<WorkSpace>,
    pub switch_scripts: Option<Vec<String>>
}

impl Config {
    /// Value is gotten from the config::get_config_json function
    pub fn get(config: Value) -> Config {
        Config {
            reload_time: config.get("reload-time")
                .expect("Config file needs reload-time")
                .as_u64()
                .expect("reload-time value needs to be a positive unsigned value"),
            workspaces: config.get("workspaces")
                .expect("Config file needs workspaces")
                .as_array()
                .expect("workspaces needs to be a json array")
                .iter()
                .map(|value| WorkSpace::create(value))
                .collect(),
            switch_scripts: config.get("monitor-switch-scripts")
                .map(|value| value
                     .as_array()
                     .expect("monitor-switch-scripts must be a json array")
                     .iter()
                     .map(|array_val| String::from(array_val
                          .as_str()
                          .expect("monitor-switch-scripts array must contain json string value"))
                          )
                     .collect()
                     )
        }
    }
    // pub fn save_current_workspace(&self) {
    //     todo!()
    // }
}


