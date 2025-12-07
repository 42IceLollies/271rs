use patch::*;

fn main() {
    let mut file = "oogadyboogady"; 
    let diff = (vec![("oogadyboogady".to_string(),0)], vec![("googoo\n".to_string(), 0), ("ga ga\n".to_string(), 1), ("peekaboo\n".to_string(), 2)]);

    
    println!("{}", file);
    
    let p = patch(diff.clone(), file.to_string());
    file = &p;

    println!("{}", file);
    
    let p = unpatch(diff, file.to_string());
    file = &p;

    println!("{}", file);
}
