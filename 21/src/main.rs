//GUESS
const RED : &str = "\u{001b}[31m";
const GREEN : &str = "\u{001b}[32m";
const YELL : &str = "\u{001b}[33m";
const WHITE: &str = "\u{001b}[0m";

const WORDS : [&str; 5] = ["rotas", "opera", "tenet", "arepo", "sator"];

fn main() {

    //playing with text in console ----------------------------------
    println!("White text. {RED}Red text. {WHITE}White text.");

    let c = "Hidey Hey!";

        
    //can either declare chrs like this and use it as an iterator
    //let chrs = c.chars();
    
    //or declare it like this - collected into a vector to iterate through it as below
    let chrs = c.chars().collect::<Vec<char>>();

    for i in 0..chrs.len(){
        print!("{}", chrs[i]);
    }


    // dev random ---------------------------------------------------------

    //open devrand file
    let mut devrnd = std::fs::File::open("/dev/urandom").unwrap();

    //creates array of u8s with value zero and as many entries as it takes to store one memory
    //address I think
    //will read a line of devrnd into thi
    //DONT FORGET THE SEMICOLON INSTEAD OF A COMMA - creates an empty array vs just an array of
    //[0,8]
    let mut buffer = [0u8; (usize::BITS/8) as usize];
    dbg!(buffer);
    //usize refers to the size of a pointer/memory address according to the system (32 bit, 64 bit,
    //etc);
    
    //reads a line of devrand into the buffer
    std::io::Read::read_exact(&mut devrnd, &mut buffer).unwrap();
    dbg!(buffer);
    //&str - string literal (Vs String data structure)
    //array can be as large as memory so it's pointers must be as large as memory (usize)
    
    let i: usize = 2;
    println!("{:?}", WORDS[i]);
    
    //reads the bytes in the buffer into a usize (little endian-ly)
    let secret = usize::from_le_bytes(buffer);

    //print out a random word from the array
    println!("{}", WORDS[secret % WORDS.len()]);

    
}


