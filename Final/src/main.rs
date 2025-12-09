use Final::*;
use std::env::args;

//commit multiple files - add new file names or find non hidden files in directory and commit them
//all (commit)
//take input from the command line


fn main() {
    let first_command: Vec<String> = args().collect();

    //commit
    match first_command[1].to_lowercase()[..]{
        "commit" => {
            commit();
        }

        "revert" => {
            revert();
        }

    }

    //revert
}
