use crate::{stack::Stack, state::State};
pub struct EVM {
    state: State,
}
 

impl EVM {
    pub fn new(
        sender: String,
        program: Vec<u8>,
        gas: u64,
        value: u128,
        calldata: Vec<u8>,
    ) -> Self {
        EVM {
            state: State::new(sender, program, gas, value, calldata),
        }
    }

    pub fn execute(
        &mut self,
    ) {
        // TODO
    }
}