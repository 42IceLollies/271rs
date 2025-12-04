//longest common subsequence

pub fn lcs(s1: &str, s2: &str) -> String {
    let vec_1: Vec<char> = s1.chars().collect();
    let vec_2: Vec<char> = s2.chars().collect();

    let result = lcs::compare_letts(0,vec_1, 0, vec_2, 0, Vec::new());
    

    return result.into_iter().fold(String::new(), |mut acc, x| {acc.push(x); acc});
}




fn main() {
    let mut ss = std::env::args();
    let _ = &ss.next();
    dbg!(lcs(&ss.next().unwrap(), &ss.next().unwrap()));
    return;
}



