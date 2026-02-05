pub mod input;
pub use input::InputEvent;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Event {
    Input(InputEvent),
}

// impl InputEvent {
//     pub fn from(old: &InputConfig, new: &InputConfig) -> Vec<InputEvent> {
//         // This will convert the config to events and then send to whereever its is required accordingly.
//     }
// }
