use crate::evm::EVM;

pub mod memory;
pub mod evm;
pub mod stack;
pub mod state;
pub mod storage;
fn main() {
    // Input params
    let sender: String = "0xSenderAddress".to_string();
    let program: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x01];
    let gas: u64 = 10_000;
    let value: u128 = 100;
    let calldata: Vec<u8> = vec![1, 2, 3];


    let mut evm : EVM = EVM::new(sender, program, gas, value, calldata);
    evm.execute();
}
