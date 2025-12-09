use diff::find_diff;
use diff::read_into_vec;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, json, Value};
use std::fs::OpenOptions;


const SCM: &str = ".scm";

pub fn new_scm() -> Map<String, Value>{
    if let Err(_) = OpenOptions::new().read(true).truncate(true).create(true).open(SCM){
        panic!("Couldn't create SCM");
    }

    let mut scm_map: Map<String, Value> = Map::new();

    scm_map.insert("Commits".to_string(), Vec::<Map<String, Value>>::new());
    scm_map.insert("Current".to_string(), Map::<String, Value>::new());
    


}

pub fn commit(){
    todo!()
}


pub fn revert(){
    todo!()
}

//struct Commit{
//    message: String,
//    diff:  (Vec<(String, usize)>, Vec<(String, usize)>), 
//}
//

//fn new_commit(msg: String, new_file: String, old_file: String) -> Commit{
//    let d = find_diff(read_into_vec(&old_file), read_into_vec(&new_file));

//    return Commit{
//        message: msg,
//        diff: d,
//    }
//}

//pub struct File{
//    commit_hist: Vec<Commit>,
//    latest_version:String,

//}

//fn new_file(commit_msg: String, ver_1: String) -> File{
//    File{
//        commit_hist: vec![new_commit(commit_msg, ver_1.clone(), String::new())],
//        latest_version: ver_1,
//    }
//}


//currently a one file repository :/
//needs to store multiple I think
//pub struct Repository{
//    files: HashMap<String, File>,
//}

//pub fn new_repository()-> Repository{
    //initial empty commit
//    Repository{
//        files: HashMap::new(),
//    }
//}



//impl Repository{
//    pub fn new_file(&mut self, name: String){
        //creates a new empty file
//        self.files.insert(name, new_file("Created new file.".to_string(), "".to_string()));
//    }

    //pub fn commit(&mut self, msg: &str, new_file: &str){
    //    self.commits.push(new_commit(msg.to_string(), new_file.clone().to_string(), self.latest_version.clone()));
    //    self.latest_version = new_file.to_string();
    //}

//    pub fn view(&self, file_name: String){
//        println!("{}", self.files.get(&file_name).unwrap().latest_version);
//    }
//}
