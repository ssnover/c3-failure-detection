pub mod emulnet;
pub mod log;
pub mod node;
pub mod params;

use rand::Rng;
use std::convert::TryInto;

pub struct Application {
    params: params::Params,
    _net: emulnet::EmulNet,
    nodes: Vec<node::Node>,
    failed_nodes: Vec<node::Node>,
    logger: log::Logger,
}

impl Application {
    pub fn new(params: params::Params) -> Application {
        let mut net = emulnet::EmulNet::new(params);
        let mut nodes: Vec<node::Node> = vec![];
        for _ in 0..params.number_of_nodes {
            nodes.push(node::Node::new(net.initialize_new_endpoint()));
        }
        let app = Application {
            params,
            _net: net,
            nodes,
            failed_nodes: vec![],
            logger: log::Logger::new("debug.log").unwrap(),
        };
        app
    }

    pub fn run(&mut self) {
        for timestep in 0..self.params.total_runtime {
            for node in &mut self.nodes {
                node.run(timestep)
            }

            if timestep == 100 && self.params.insert_single_failure {
                // Move a random node to the failed node list
                let mut rng = rand::thread_rng();
                let random_node_id = rng.gen_range(0, self.params.number_of_nodes);
                self.failed_nodes.push(
                    self.nodes.remove(
                        self.nodes
                            .iter()
                            .position(|n| u32::from(n.id) == random_node_id)
                            .unwrap(),
                    ),
                );
                // Log the failure for the node so the timestep is known
                self.logger.log_failure_event(
                    timestep,
                    random_node_id.try_into().unwrap(),
                    random_node_id.try_into().unwrap(),
                );
            } else if timestep == 100 && self.params.insert_multiple_failures {
                let mut rng = rand::thread_rng();
                let random_node_id = rng.gen_range(0, self.params.number_of_nodes / 2);
                for id in random_node_id..(random_node_id + (self.params.number_of_nodes / 2)) {
                    self.failed_nodes.push(
                        self.nodes.remove(
                            self.nodes
                                .iter()
                                .position(|n| u32::from(n.id) == id)
                                .unwrap(),
                        ),
                    );
                    self.logger.log_failure_event(
                        timestep,
                        id.try_into().unwrap(),
                        id.try_into().unwrap(),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_no_failures() {
        let mut builder = params::ParamsBuilder::new();
        builder.with_number_of_nodes(10);
        let mut app = Application::new(builder.build());
        app.run();

        assert_eq!(app.nodes.len(), 10);
        assert_eq!(app.failed_nodes.len(), 0);
        assert_eq!(app.logger.count_join_events(), 10 * 10);
        assert_eq!(app.logger.count_failure_events(), 0);
    }

    #[test]
    fn test_single_failure() {
        let mut builder = params::ParamsBuilder::new();
        builder
            .with_number_of_nodes(10)
            .with_insert_single_failure(true);
        let mut app = Application::new(builder.build());
        app.run();

        assert_eq!(app.nodes.len(), 9);
        assert_eq!(app.failed_nodes.len(), 1);
        assert_eq!(app.logger.count_join_events(), 10 * 10);
        assert_eq!(app.logger.count_failure_events(), 10);
    }

    #[test]
    fn test_single_failure_with_message_drop() {
        let mut builder = params::ParamsBuilder::new();
        builder
            .with_number_of_nodes(10)
            .with_insert_single_failure(true)
            .with_drop_messages(true)
            .with_dropped_message_probability(0.1);
        let mut app = Application::new(builder.build());
        app.run();

        assert_eq!(app.nodes.len(), 9);
        assert_eq!(app.failed_nodes.len(), 1);
        assert_eq!(app.logger.count_join_events(), 10 * 10);
        assert_eq!(app.logger.count_failure_events(), 10);
    }

    #[test]
    fn test_multiple_failures_with_message_drop() {
        let mut builder = params::ParamsBuilder::new();
        builder
            .with_number_of_nodes(10)
            .with_insert_multiple_failures(true);
        let mut app = Application::new(builder.build());
        app.run();

        assert_eq!(app.nodes.len(), 9);
        assert_eq!(app.failed_nodes.len(), 1);
        assert_eq!(app.logger.count_join_events(), 10 * 10);
        assert_eq!(app.logger.count_failure_events(), 5*5 + 5);
    }
}
