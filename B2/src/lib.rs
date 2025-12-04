//sorry for the terrible naming scheme - I did this in rust playground and copied it over
//matchi is the index of the location along string s right after the last match in the row
pub fn compare_letts(fi: usize, f: Vec<char>, si: usize, s: Vec<char>, matchi: usize, mut common: Vec<char>) ->  Vec<char>{
   let fc = f[fi];
   let sc = s[si];
   let end_f = fi == f.len()-1;
   let end_s = si == s.len()-1;

    //match!
    if fc == sc{
        common.push(fc);
        //end of f or s -> return
        if end_f || end_s {
            return common;
        }
        //else, increment f, increment s
        else{
            return compare_letts(fi+1, f, si+1, s, si+1, common);
        }
    }
    //no match
    else{
        //end of s and end of f -> return
        if end_f && end_s{
            return common;
        }
        //end of s, not end of f -> increment f, s counter is matchi
        else if end_s{
            return compare_letts(fi+1, f, matchi, s, matchi, common);
        }

        //nothing ends -> increment s
        else{
            return compare_letts(fi, f, si+1, s, matchi, common);
        }
   }

}
