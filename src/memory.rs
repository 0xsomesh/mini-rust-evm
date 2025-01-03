pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: Vec::new(),
        }
    }

    fn expand_memory(&mut self, offset: usize, size: usize) {
        let required_size = offset + size;
        if required_size > self.memory.len() {
            self.memory.resize(required_size, 0);
        }
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) {
        self.expand_memory(offset, data.len());
        self.memory[offset..offset + data.len()].copy_from_slice(data);
    }

    pub fn read(&self, offset: usize, size: usize) -> Vec<u8> {
        if offset + size > self.memory.len() {
            let mut result = vec![0; size];
            let copy_size = self.memory.len().saturating_sub(offset);
            result[..copy_size].copy_from_slice(&self.memory[offset..]);
            result
        } else {
            self.memory[offset..offset + size].to_vec()
        }
    }

    pub fn size(&self) -> usize {
        self.memory.len()
    }
}