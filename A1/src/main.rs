//DAG
use std::collections::HashMap;
use std::io::{self, BufRead};

//---------------- Node -----------------
#[derive(Debug)]
struct Node {
    data:String, 
    next: Vec<String>,
}

fn node(d: String, n: Vec<String>) -> Node{
    return Node{
        data: d,
        next: n,
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }
}


//----------------- DAG ------------
#[derive(Debug)]
struct DAG {
    roots: Vec<Node>,
}

fn dag() -> DAG {
    return DAG{
        roots: Vec::new(),
    }
}




// -------------- Application -------------------
fn main() {
    //make a dictionary thing for each class and its post requisite list
    //read the file into the dictionary
    //for each class make a node
    
    //read the lines out of the terminal
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    
    //let mut pairs : Vec<String> = Vec::new();
    let mut pairs = Vec::new();
    let mut term = true;

    while term{
        let mut temp = String::new();
        handle.read_line(&mut temp);
        if temp!= "\n"{
            pairs.push(temp);
        } else if !pairs.is_empty(){
            term = false;
        }

    }
    


    //create a real quick hashmap to keep track of things
    //and a DAG
    let mut dag = dag();
    let mut org = HashMap::new();
    
    //for line in lines
    for pair in pairs {
        //separate each line into before and after : 
        let (pre, mut p) = pair.rsplit_once(|char| char==(':')).unwrap();
        
        let mut post = String::from(p);

        //get rid of the \n from post 
        post.pop();
        post.pop();
            
        let mut temp = Vec::new();
        temp.push(post);

        let new_node = node((*pre).to_string(), temp);
        let post_node = node(post, Vec::new());

        //if there's no according key in hashmap, make a new node and add it
        if !org.contains_key(pre){
        //if the current head of the hashmap is a child of new node, make it the new head
            org.insert(pre, new_node);            

        }else{
       //otherwise, add the next value to the existing node
            org.get(&new_node).unwrap().next.push(post_node);
        
        }

        if dag.roots.is_empty() || dag.roots.contains(&post_node){
            std::mem::replace(&mut dag.roots[dag.roots.iter().position(|&x| x == post_node).unwrap()], new_node);
        }


    }    

}
