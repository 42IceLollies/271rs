//queue is first in first out 
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Queue<T> {
    vals: Option<Node<T>>,
}


pub fn queue<T>() -> Queue<T>{
    //creates an empty queue
    return Queue{
        vals: None,
    }
}

pub trait Push<T> {
    fn push(self, val: T) -> Self; 
}

pub trait Pop<T> {
    fn pop(self) -> (Option<T>, Self);
}

fn second_last_node<T>(mut n_curr: Node<T>, mut n_prev: Node<T>) -> Node<T>{
    
    match n_curr.next {
        Some(ref v) => {
            n_prev = n_curr.clone();
            n_curr = *n_curr.next.unwrap();
            return second_last_node(n_curr, n_prev)
           }
        None => return n_prev,
     }

}

impl<T> Push<T> for Queue<T> {
    //needs to find the end of the queue and pop that
    fn push (self, val: T) -> Queue<T>{
        match self.vals {
            Some(v) => return Queue{ vals: Some(Node{data:val, next: Some(Box::new(v))})},
            None => return Queue{vals: Some(Node{data:val, next: None})},
    
        }
    }
}


impl<T> Pop<T> for Queue<T> {
    fn pop(self) -> (Option<T>, Queue<T>) {
        match self.vals{
        Some(ref v) =>
            match v.next {
                Some(n) => return (Some(second_last_node(self.vals.unwrap(), self.vals.unwrap()).next.unwrap().data), Queue{vals: Some(*n)}),
                None => return (Some(v.data), Queue{vals: None}),
            }
            None => return (None, Queue{vals: None}),
        }
    }
    
}


//==============================================================================================================

//stack is last in first out

#[derive(Debug)]
pub struct Stack<T>{
    vals: Option<Node<T>>,
}

//constructor for stack
pub fn stack<T>()-> Stack<T>{
    return Stack{
        vals: None,
    }
}


impl<T> Push<T> for Stack<T> {
    fn push(self, val: T) -> Stack<T>{
        match self.vals{
            Some(v) => return Stack{vals: Some(Node{data:val, next: Some(Box::new(v))})},
            None => return Stack{vals: Some(Node{data:val, next: None})},
        }
    }
}

impl<T> Pop<T> for Stack<T> {
    fn pop(self) -> (Option<T>, Stack<T>){
        match self.vals{
            Some(v) => 
                match v.next {
                    Some(n) => return (Some(v.data), Stack{vals:Some(*n)}),
                    None => return (Some(v.data), Stack{vals: None}),
                }
            None => return (None, Stack{vals: None}),
        }
    }
}
