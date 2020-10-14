#[derive(Debug, Copy, Clone)]
pub struct Params {
    pub number_of_nodes: u32,
    pub insert_single_failure: bool,
    pub total_runtime: u32,
    pub insert_multiple_failures: bool,
    pub drop_messages: bool,
    pub dropped_message_probability: f64,
}

pub struct ParamsBuilder {
    pub number_of_nodes: Option<u32>,
    pub insert_single_failure: Option<bool>,
    pub total_runtime: Option<u32>,
    pub insert_multiple_failures: Option<bool>,
    pub drop_messages: Option<bool>,
    pub dropped_message_probability: Option<f64>,
}

impl ParamsBuilder {
    pub fn new() -> ParamsBuilder {
        ParamsBuilder {
            number_of_nodes: None,
            insert_single_failure: None,
            total_runtime: None,
            insert_multiple_failures: None,
            drop_messages: None,
            dropped_message_probability: None,
        }
    }

    pub fn build(&self) -> Params {
        Params {
            number_of_nodes: self.number_of_nodes.unwrap_or(10),
            insert_single_failure: self.insert_single_failure.unwrap_or(false),
            total_runtime: self.total_runtime.unwrap_or(700),
            insert_multiple_failures: self.insert_multiple_failures.unwrap_or(false),
            drop_messages: self.drop_messages.unwrap_or(false),
            dropped_message_probability: self.dropped_message_probability.unwrap_or(0.0),
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

    pub fn with_insert_multiple_failures(&mut self, fail: bool) -> &mut ParamsBuilder {
        self.insert_multiple_failures = Some(fail);
        self
    }

    pub fn with_drop_messages(&mut self, drop_messages: bool) -> &mut ParamsBuilder {
        self.drop_messages = Some(drop_messages);
        self
    }

    pub fn with_dropped_message_probability(
        &mut self,
        dropped_message_probability: f64,
    ) -> &mut ParamsBuilder {
        self.dropped_message_probability = Some(dropped_message_probability);
        self
    }
}
