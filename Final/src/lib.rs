use diff::find_diff;
use diff::read_into_vec;

struct Commit{
    message: String,
    diff:  (Vec<(String, usize)>, Vec<(String, usize)>), 
}


fn new_commit(msg: String, new_file: String, old_file: String) -> Commit{
    let d = find_diff(read_into_vec(&old_file), read_into_vec(&new_file));

    return Commit{
        message: msg,
        diff: d,
    }
}


//currently a one file repository :/
//needs to store multiple I think
pub struct Repository{
    commits: Vec<Commit>,
    latest_version: String,
}

pub fn new_repository()-> Repository{
    //initial empty commit
    Repository{
        commits: Vec::from([new_commit(String::new(), String::new(), String::new())]),
        latest_version: String::new(),
    }
}



impl Repository{
    pub fn commit(&mut self, msg: &str, new_file: &str){
        self.commits.push(new_commit(msg.to_string(), new_file.clone().to_string(), self.latest_version.clone()));
        self.latest_version = new_file.to_string();
    }

    pub fn view(&self){
        println!("{}", self.latest_version);
    }
}
