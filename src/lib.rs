pub mod emulnet;
pub mod params;

pub struct Application {
    params: params::Params,
    net: emulnet::EmulNet,
    nodes: Vec<u8>,
}

impl Application {
    pub fn new(insert_failure: bool) -> Application {
        let params = params::Params {
            number_of_nodes: 10,
            insert_failure,
        };
        let mut net = emulnet::EmulNet::new(params);
        let mut nodes: Vec<u8> = vec![];
        for _ in 0..params.number_of_nodes {
            nodes.push(net.initialize_new_endpoint());
        }
        let app = Application { params, net, nodes };
        app
    }
    pub fn run(&self) {}
}