use Final::*;

fn main() {
    let mut test_repo = new_repository();
    test_repo.view();
    test_repo.commit("first commit!", "This is my new file");
    test_repo.view();
}
