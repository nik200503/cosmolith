// Watch Input Config Changes

use std::{error::Error, sync::mpsc::Sender};

use cosmic_comp_config::input::InputConfig;
use cosmic_config::{Config, ConfigGet};

use crate::event::Event;
use std::sync::{Arc, Mutex};

pub struct InputState {
    touchpad: Option<InputConfig>,
    mouse: Option<InputConfig>,
    // Will be added later after identifying its type
    // keyboard:
}

pub fn start_input_watcher(
    tx: &Arc<Mutex<Sender<Event>>>,
) -> Result<Box<dyn std::any::Any + Send>, Box<dyn Error>> {
    let config = Config::new("com.system76.CosmicComp", 1)?;
    let state = Arc::new(Mutex::new(InputState {
        touchpad: config.get::<InputConfig>("input_touchpad").ok(),
        mouse: config.get::<InputConfig>("input_default").ok(),
    }));

    // // Keep the watcher alive for the lifetime of the program.
    let watcher = config.watch({
        let tx = Arc::clone(&tx);
        let state = Arc::clone(&state);
        move |cfg: &Config, keys| {
            if let Ok(_sender) = tx.lock() {
                if let Ok(mut state) = state.lock() {
                    from_input_state(&mut state, cfg, keys);
                }
            }
        }
    })?;

    Ok(Box::new(watcher))
}

pub fn from_input_state(state: &mut InputState, cfg: &Config, keys: &[String]) -> Vec<Event> {
    for key in keys {
        match key.as_str() {
            "input_touchpad" => {
                match cfg.get::<InputConfig>(key) {
                    Ok(new_config) => {
                        if let Some(old) = state.touchpad.clone() {
                            from_touchpad(old, new_config.clone());
                        }
                        state.touchpad = Some(new_config);
                    }
                    Err(e) => {
                        println!("Failed to get changed config due to the error: {:?}", e);
                    }
                }
            }
            "input_default" => {
                match cfg.get::<InputConfig>(key) {
                    Ok(new_config) => {
                        if let Some(old) = state.mouse.clone() {
                            from_touchpad(old, new_config.clone());
                        }
                        state.mouse = Some(new_config);
                    }
                    Err(e) => {
                        println!("Failed to get changed config due to the error: {:?}", e);
                    }
                }
            }
            x => {
                println!("Unknown key: {}", x);
            }
        }
    }
    vec![]
}

pub fn from_touchpad(old: InputConfig, new: InputConfig) -> Vec<Event> {
    if old == new {
        return vec![];
    }

    if old.state != new.state {
        println!("touchpad.state changed: {:?} -> {:?}", old.state, new.state);
    }
    if old.acceleration != new.acceleration {
        println!(
            "touchpad.acceleration changed: {:?} -> {:?}",
            old.acceleration, new.acceleration
        );
    }
    if old.calibration != new.calibration {
        println!(
            "touchpad.calibration changed: {:?} -> {:?}",
            old.calibration, new.calibration
        );
    }
    if old.click_method != new.click_method {
        println!(
            "touchpad.click_method changed: {:?} -> {:?}",
            old.click_method, new.click_method
        );
    }
    if old.disable_while_typing != new.disable_while_typing {
        println!(
            "touchpad.disable_while_typing changed: {:?} -> {:?}",
            old.disable_while_typing, new.disable_while_typing
        );
    }
    if old.left_handed != new.left_handed {
        println!(
            "touchpad.left_handed changed: {:?} -> {:?}",
            old.left_handed, new.left_handed
        );
    }
    if old.middle_button_emulation != new.middle_button_emulation {
        println!(
            "touchpad.middle_button_emulation changed: {:?} -> {:?}",
            old.middle_button_emulation, new.middle_button_emulation
        );
    }
    if old.rotation_angle != new.rotation_angle {
        println!(
            "touchpad.rotation_angle changed: {:?} -> {:?}",
            old.rotation_angle, new.rotation_angle
        );
    }
    if old.scroll_config != new.scroll_config {
        println!(
            "touchpad.scroll_config changed: {:?} -> {:?}",
            old.scroll_config, new.scroll_config
        );
    }
    if old.tap_config != new.tap_config {
        println!(
            "touchpad.tap_config changed: {:?} -> {:?}",
            old.tap_config, new.tap_config
        );
    }
    if old.map_to_output != new.map_to_output {
        println!(
            "touchpad.map_to_output changed: {:?} -> {:?}",
            old.map_to_output, new.map_to_output
        );
    }

    vec![]
}
