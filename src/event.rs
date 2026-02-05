use cosmic_comp_config::input::InputConfig;
use crate::watcher::input::InputEvent;
#[derive(Debug, Clone)]
pub enum Event {
    Input(InputEvent),
}


// impl InputEvent {
//     pub fn from(old: &InputConfig, new: &InputConfig) -> Vec<InputEvent> {
//         // This will convert the config to events and then send to whereever its is required accordingly.
//     }
// }
