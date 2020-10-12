pub mod emulnet;
pub mod node;
pub mod params;

use rand::Rng;
use params::ParamsBuilder;

pub struct Application {
    params: params::Params,
    net: emulnet::EmulNet,
    nodes: Vec<node::Node>,
    failed_nodes: Vec<node::Node>,
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
            net,
            nodes,
            failed_nodes: vec![],
        };
        app
    }

    pub fn run(&mut self) {
        let mut failure_triggered = false;
        for timestep in 0..self.params.total_runtime {
            for node in &mut self.nodes {
                node.run(timestep)
            }

            if timestep == (self.params.total_runtime / 2)
                && self.params.insert_single_failure
                && !failure_triggered
            {
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
                failure_triggered = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_no_failures() {
        let mut builder = crate::ParamsBuilder::new();
        builder.with_number_of_nodes(10);
        let mut app = crate::Application::new(builder.build());
        app.run();

        assert_eq!(app.nodes.len(), 10);
        assert_eq!(app.failed_nodes.len(), 0);
    }

    #[test]
    fn test_single_failure() {
        let mut builder = crate::ParamsBuilder::new();
        builder.with_number_of_nodes(10).with_insert_single_failure(true);
        let mut app = crate::Application::new(builder.build());
        app.run();

        assert_eq!(app.nodes.len(), 9);
        assert_eq!(app.failed_nodes.len(), 1);
    }
}