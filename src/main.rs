use opcodes::OpcodeRegistry;

use crate::evm::EVM;

pub mod memory;
pub mod evm;
pub mod stack;
pub mod state;
pub mod storage;
pub mod opcodes;
fn main() {
    // Input params (For testing purposes)
    let sender: String = "0xSenderAddress".to_string();
    let program: Vec<u8> = vec![0x01];
    let gas: u64 = 5;
    let value: u128 = 100;
    let calldata: Vec<u8> = vec![1, 2, 3];


    let mut opcode_registry: OpcodeRegistry = OpcodeRegistry::new();
    opcode_registry.initiate();

    let mut evm : EVM = EVM::new(sender, program, gas, value, calldata, opcode_registry);
    evm.execute();
}
