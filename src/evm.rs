use crate::{opcodes::OpcodeRegistry, state::State};
pub struct EVM {
    pub state: State,
    pub registry: OpcodeRegistry,
}
 

impl EVM {
    pub fn new(
        sender: String,
        program: Vec<u8>,
        gas: u64,
        value: u128,
        calldata: Vec<u8>,
        opcode_registry: OpcodeRegistry
    ) -> Self {
        EVM {
            state: State::new(sender, program, gas, value, calldata),
            registry: opcode_registry
        }
    }

    pub fn execute(
        &mut self,
    ) {
        while self.state.pc < self.state.program.len() {
            let opcode_id = self.state.program[self.state.pc] as u32;
            self.state.pc += 1;
    
            if let Some(opcode) = self.registry.get(opcode_id) {
                if self.state.gas < opcode.gas_cost() {
                    self.state.stop_flag = true;
                }
    
                self.state.gas -= opcode.gas_cost();
                let _ = opcode.execute(&mut self.state);
            } else {
                self.state.stop_flag = true;
            }
    
            if self.state.stop_flag {
                break;
            }
        }    
    }
}