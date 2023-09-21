struct MinStack {
    stack: Vec<(i32, i32)>,
    min: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
       MinStack{stack: Vec::new(), min: i32::MAX} 
    }
    
    fn push(&mut self, val: i32) {
        if val < self.min {
            self.min = val;
        }
        self.stack.push((val, self.min));
    }
    
    fn pop(&mut self) {
        self.stack.pop().unwrap();
        self.min = self.stack.last().unwrap_or(&(-1, i32::MAX)).1
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

fn main() {
    println!("Hello, world!");
}
