use lib1::lib1_type::types::creat_lib1_struct;
use lib2::lib2_type::types::creat_lib2_struct;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn sort_score(users: &mut Vec<User>) {
    users.sort_by_key(|user| user.sign_in_count);
}
fn main() {
    println!("Hello, world!");

    creat_lib1_struct();
    creat_lib2_struct();
}
