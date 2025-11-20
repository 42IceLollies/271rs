// Potential words for the picking
const WORDS : [&str; 5] = ["sator", "arepo", "tenet", "opera", "rotas"];

// Text Colors!!!!
const RED : &str = "\u{001b}[31m"; 
const GREEN : &str = "\u{001b}[32m";
const YELL : &str = "\u{001b}[33m";
const WHITE: &str = "\u{001b}[0m";

// Box bits
const TOP: &str = "┌───┬───┬───┬───┬───┐";
const MID: &str = "├───┼───┼───┼───┼───┤";
const BOT: &str = "└───┴───┴───┴───┴───┘";


fn main(){
   //This is the weirdest implementation of wordle ever
   //Idk this is like the main control function
   
    let mut temp: &str = "     ";
    let mut words: [&str; 6] = [temp,temp,temp,temp,temp,temp];


    let mut devrnd = std::fs::File::open("/dev/urandom").unwrap();
    let mut buffer = [0u8; (usize::BITS / 8) as usize];
    std::io::Read::read_exact(&mut devrnd, &mut buffer).unwrap();
    let mut secret = usize::from_ne_bytes(buffer);
    let answer : &str = &String::from(WORDS[secret%WORDS.len()]);
    
    let mut attempts: usize = 0;

    print!("\u{001b}[2J");
    println!("ONLY USE LOWER CASE!!! PLEASE!!!");


    while words[5] == temp{
        let mut guess = String::new();       
        std::io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().to_lowercase();
        if let Some(&Guess) =WORDS.iter().find(|&&x|x== guess){
            words[attempts] = Guess;
            fair_game(words, answer);
            if guess == answer{
                println!("YOU WINNER, YOU!");
                return;
            }
            attempts+=1;

        } else {
            println!("That's not a word, babes :(");
    }

    }
    println!("You lost, soz.");
}


fn letterer(letter: char, color: &str){
    // Prints a given letter in its correct color
    print!("{color} {} ", letter);
}

fn colorify(answer: &str, guess: &str){
    // Colorizes and prints the guess word, comparing each letter with correct answer
    
    //list of letters in guess and answer respectively
    let mut g_letts = guess.chars();
    let mut a_letts = answer.chars();

    print!("|");
    

    //loop through letters
    for i in 0..5{
        let c:char = g_letts.next().unwrap();
        let mut color = RED;
        let mut a_lett_copy = a_letts.clone();

        if c == a_letts.next().unwrap() {
            color = GREEN;
        } else if a_lett_copy.any(|x|x == c) {
            color = YELL;
        }

        letterer(c, color);
        print!("{WHITE}|");
    
    }
    println!("");

}


fn fair_game(words: [&str; 6], answer: &str){
    // Draws the game board, with guesses and clears screen when necesary
    

    //clears the screen, apparently
    print!("\u{001b}[2J");

    println!("{}", TOP);
    
    for i in 0..5{
        colorify(answer, words[i]);
        println!("{}", MID);
    }
    
    //I'm confused by this :/
    colorify(answer, words[5]);
    println!("{}", BOT);

}
