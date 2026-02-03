use cosmic_comp_config::input::InputConfig;

#[derive(Debug, Clone)]
pub enum Event {
    Input(InputEvent),
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    ConfigChanged { key: String, config: InputConfig },
}

// impl InputEvent {
//     pub fn from(old: &InputConfig, new: &InputConfig) -> Vec<InputEvent> {
//         // This will convert the config to events and then send to whereever its is required accordingly.
//     }
// }
