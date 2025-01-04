use crate::stack::Stack;
use crate::memory::Memory;
use crate::storage::Storage;

pub struct State {
    pub stack: Stack<[u8; 32]>, 
    pub memory: Memory,
    pub storage: Storage,
    pub pc: usize,

    pub sender: String,
    pub program: Vec<u8>,
    pub gas: u64,
    pub value: u128,
    pub calldata: Vec<u8>,

    pub stop_flag: bool,
    pub revert_flag: bool,

    pub return_data: Vec<u8>,
    pub logs: Vec<String>
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