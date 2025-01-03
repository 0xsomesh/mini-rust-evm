pub struct Stack<T> {
    pub stack: Vec<T>,
}

pub const MAXIMUM_STACK_SIZE : i32 = 1024;


impl<T> Stack<T> {
    pub fn new() -> Self {
      Stack { stack: Vec::new() }
    }

    pub fn length(&self) -> usize {
      self.stack.len()
    }
  
    pub fn pop(&mut self) -> Option<T> {
      self.stack.pop()
    }
  
    pub fn push(&mut self, item: T) {
      if self.length() >= MAXIMUM_STACK_SIZE.try_into().unwrap()  {
        panic!("stack overflow");
      } else {
        self.stack.push(item)
      }
    }
  
    pub fn is_empty(&self) -> bool {
      self.stack.is_empty()
    }
  
    pub fn peek(&self) -> Option<&T> {
      self.stack.last()
    }
  }