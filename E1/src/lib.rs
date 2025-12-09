//patch
//applying diff to previous file
//and unapplying it from file 
use diff::read_into_vec;

pub fn patch(diff: (Vec<(String, usize)>, Vec<(String, usize)>), file: String) -> String{
    let mut file_vec: Vec<String> = read_into_vec(&file).into_iter().map(|x| x.to_string()).collect();
     
   
    for change in diff.0{
        file_vec.remove(change.1);
    }

    for change in diff.1{
        file_vec.insert(change.1, change.0);
    }

    return  file_vec.into_iter().fold(String::new(), |mut acc, x| {acc.push_str(&x); acc});
    //return "hi".to_string();
}

pub fn unpatch(diff: (Vec<(String, usize)>, Vec<(String, usize)>), file: String) -> String{
    let mut file_vec: Vec<String> = read_into_vec(&file).into_iter().map(|x| x.to_string()).collect();
    
    //now the adds are removals
    for i in (0..diff.1.len()).rev(){
        file_vec.remove(diff.1[i].1);   
    }


    //and the removals are adds
    for i in (0..diff.0.len()).rev(){
        file_vec.insert(diff.0[i].1, diff.0[i].0.clone());
    }

    
    return  file_vec.into_iter().fold(String::new(), |mut acc, x| {acc.push_str(&x); acc});
}
