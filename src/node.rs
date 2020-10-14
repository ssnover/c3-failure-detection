use crate::log::Logger;

pub struct Node {
    pub id: u8,
}

impl Node {
    pub fn new(id: u8) -> Node {
        Node { id }
    }

    pub fn run(&mut self, _current_time: u32, _logger: &mut Logger) {}
}
