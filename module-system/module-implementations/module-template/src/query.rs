use super::ExampleModule;
use sov_state::WorkingSet;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq)]
pub enum QueryMessage {
    GetValue,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
pub struct Response {
    pub value: Option<u32>,
}

impl<C: sov_modules_api::Context> ExampleModule<C> {
    /// Queries the state of the module.
    pub fn query_value(&self, working_set: &mut WorkingSet<C::Storage>) -> Response {
        Response {
            value: self.value.get(working_set),
        }
    }
}