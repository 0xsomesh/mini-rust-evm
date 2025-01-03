use crate::stack::Stack;
use crate::memory::Memory;
use crate::storage::Storage;

pub struct State {
    stack: Stack<[u8; 32]>, 
    memory: Memory,
    storage: Storage,
    pc: usize,

    sender: String,
    program: Vec<u8>,
    gas: u64,
    value: u128,
    calldata: Vec<u8>,

    stop_flag: bool,
    revert_flag: bool,

    return_data: Vec<u8>,
    logs: Vec<String>
}

impl State {
    pub fn new(
        sender: String,
        program: Vec<u8>,
        gas: u64,
        value: u128,
        calldata: Vec<u8>,
    ) -> Self {
        Self {
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            sender,
            program,
            gas,
            value,
            calldata,
            stop_flag: false,
            revert_flag: false,
            return_data: Vec::new(),
            logs: Vec::new(),
        }
    }
}