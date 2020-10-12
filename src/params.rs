#[derive(Debug, Copy, Clone)]
pub struct Params {
    pub number_of_nodes: u32,
    pub insert_single_failure: bool,
    pub total_runtime: u32,
}

pub struct ParamsBuilder {
    pub number_of_nodes: Option<u32>,
    pub insert_single_failure: Option<bool>,
    pub total_runtime: Option<u32>,
}

impl ParamsBuilder {
    pub fn new() -> ParamsBuilder {
        ParamsBuilder { number_of_nodes: None,
            insert_single_failure: None,
            total_runtime: None}
    }

    pub fn build(&self) -> Params {
        Params {
            number_of_nodes: self.number_of_nodes.unwrap_or(10),
            insert_single_failure: self.insert_single_failure.unwrap_or(false),
            total_runtime: self.total_runtime.unwrap_or(200)
        }
    }

    pub fn with_number_of_nodes(&mut self, n: u32) -> &mut ParamsBuilder {
        self.number_of_nodes = Some(n);
        self
    }

    pub fn with_insert_single_failure(&mut self, fail: bool) -> &mut ParamsBuilder {
        self.insert_single_failure = Some(fail);
        self
    }

    pub fn with_total_runtime(&mut self, runtime: u32) -> &mut ParamsBuilder {
        self.total_runtime = Some(runtime);
        self
    }
}