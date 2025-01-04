use crate::state::State;
use std::collections::HashMap;

pub trait Opcode {
    fn identifier(&self) -> u32;
    fn gas_cost(&self) -> u64;
    fn execute(&self, state: &mut State) -> Result<(), String>;
}

///////////////////// OPCODES //////////////////////

pub struct Add;

impl Opcode for Add {
    fn identifier(&self) -> u32 {
        0x01
    }

    fn gas_cost(&self) -> u64 {
        3 
    }

    fn execute(&self, state: &mut State) -> Result<(), String> {
        let a = state.stack.pop().ok_or("Stack underflow")?;
        let b = state.stack.pop().ok_or("Stack underflow")?;
        let result = add_256_bit(a, b);
        state.stack.push(result);
        Ok(())
    }
}



///////////////// REGISTRY //////////////////////////



pub struct OpcodeRegistry {
    opcodes: HashMap<u32, Box<dyn Opcode>>,
}

impl OpcodeRegistry {
    pub fn new() -> Self {
        Self {
            opcodes: HashMap::new(),
        }
    }

    pub fn register(&mut self, opcode: Box<dyn Opcode>) {
        self.opcodes.insert(opcode.identifier(), opcode);
    }

    pub fn get(&self, identifier: u32) -> Option<&Box<dyn Opcode>> {
        self.opcodes.get(&identifier)
    }

    pub fn initiate(&mut self) {
        self.register(Box::new(Add));
    }
}



///////////////////////// UTILS ///////////////////////////

fn add_256_bit(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut carry = 0u8;

    for i in (0..32).rev() {
        let sum = a[i] as u16 + b[i] as u16 + carry as u16;
        result[i] = sum as u8;
        carry = (sum >> 8) as u8;
    }

    result
}





