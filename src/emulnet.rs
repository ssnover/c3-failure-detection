use std::collections::VecDeque;
use crate::params::Params;

pub struct EmulNet {
    params: Params,
    next_node_id: u8,
    message_buffer: Vec<VecDeque<(u8, Vec<u8>)>>,
}

impl EmulNet {

    /// Creates a new emulated network.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for the emulated network.
    pub fn new(params: Params) -> EmulNet {
        EmulNet {params, next_node_id: 0, message_buffer: vec![]}
    }

    /// Initializes the emulated network structure with support for a new node.
    ///
    /// Returns the id of the new node.
    pub fn initialize_new_endpoint(&mut self) -> u8 {
        let new_endpoint = self.next_node_id;
        self.message_buffer.push(VecDeque::new());
        self.next_node_id += 1;
        new_endpoint
    }

    /// Queues a message into the emulated network.
    ///
    /// # Arguments
    ///
    /// * `src` - The id of the sending node.
    /// * `dest` - The id of the node to receive the message.
    /// * `data` - The message.
    pub fn send(&mut self, src: u8, dest: u8, data: &Vec<u8>) {
        let dest = usize::from(dest);
        if dest < self.message_buffer.len() {
            self.message_buffer[dest].push_back((src, data.clone()));
        }
    }

    /// Reads a message out of the network if one is available.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node to check for messages for.
    ///
    /// If a message is available, returns it as a tuple of the id that sent the message with the message.
    pub fn recv(&mut self, id: u8) -> Option<(u8, Vec<u8>)> {
        let id = usize::from(id);
        self.message_buffer[id].pop_front()
    }
}
