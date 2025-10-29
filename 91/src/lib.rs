#[derive(Debug)]
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Stack {
    vals: Option<Node>,
}

pub fn init() -> Stack {
    return Stack {
        vals: None,
    };
}

pub fn push(val: String, s: Stack) -> Stack {
    match s.vals {
        Some(v) => return Stack{vals: Some(Node{data:val, next: Some(Box::new(v))})},
        None => return Stack{vals: Some(Node{data:val, next: None})},
    }
}

pub fn pop(mut s: Stack) -> (Option<String>, Stack) {
    //given stack - if vals, vals.next -> return vals, new stack with vals.next
    //if vals, !vals.next -> return vals, Stack(None)
    //if !vals, return None, Stack(None)

    match s.vals{
        Some(v) => 
            match v.next {
                Some(n) => return (Some(v.data), Stack{vals: Some(*n)}),
                None => return (Some(v.data), Stack{vals: None}),
            }
            None => return (None, Stack{vals: None}),
    }
}
