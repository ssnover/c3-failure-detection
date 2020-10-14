use crate::log::Logger;
use crate::emulnet::EmulNet;

pub struct Node {
    pub id: u8,
}

impl Node {
    pub fn new(id: u8) -> Node {
        Node { id }
    }

    /// This function should be used for processing all events and received
    /// messages at a given timestep in the emulation.
    ///
    /// Expectations for side-effects generated as a result of running this function:
    /// * When a node adds to its membership list (including its own node id), it logs an event.
    /// * When a node detects another has failed, it logs an event.
    /// These actions can be performed through the `log::Logger` object.
    pub fn run(&mut self, _current_time: u32, _logger: &mut Logger, _net: &mut EmulNet) {}
}
