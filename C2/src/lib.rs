pub fn find_diff(file1: Vec<&str>, file2: Vec<&str>) -> (Vec<(String, usize)>, Vec<(String, usize)>) {
    //find the lines that are the same
    let static_lines: Vec<String> = l_c_s_line(file1.clone(), file2.clone());
    //First index of tuple: removes, second index: adds
    let mut changes: (Vec<(String, usize)>, Vec<(String, usize)>) = (Vec::new(), Vec::new());

    //comfpare the lines
    for i in 0..file1.len(){
    //if a line is in file1 and static, it was removed
        if !static_lines.contains(&file1[i].to_string()){
            changes.0.push((file1[i].to_string().clone(),i));
        }
    }

    for i in 0..file2.len(){
    //if ai line is in file2 and static, it was added
        if !static_lines.contains(&file2[i].to_string()){
            changes.1.push((file2[i].to_string().clone(), i));
        }
    }

    return changes;
}



pub fn read_into_vec(file:&str) -> Vec<&str>{
    let sfile: String = file.to_string();
    let mut slices: Vec<&str> = Vec::new();

    //ints for storing slice indexes
    let mut fi = 0;
    let mut si = 0;

    for letter in sfile.chars(){
        if letter == '\n'{
            slices.push(&file[fi..si]);
            fi = si+1;
        }

        si+=1;
    }

    return slices;
}

