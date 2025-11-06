#[derive(Debug)]
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Queue {
    vals: Option<Node>,
}


pub trait Push {
    fn push(self, val: T) -> Self; 
}

pub trait Pop {
    fn pop(self, val: T) -> Self;
}


impl<T> Push<T> for Queue<T> {
    fn push (self, val: T) -> Queue<T>{
        match self.vals {
        Some(v) => return Queue{ vals: Some(Node{data:T, next: Some(Box::new(v))})},
        None => return Queue{vals: Some(Node{data:T, next: None})},
    
        }
    }
}


impl Pop for Queue {
    fn pop(self) -> (Option<String>, Queue) {
        match q.vals{
        Some(v) =>
            match v.next {
                Some(n) => return (Some(v.data), Queue{vals: Some(*n)}),
                None => return (Some(v.data), Queue{vals: None}),
            }
            None => return (None, Queue{vals: None}),
        }
    }
    
}

