fn func_copy_back() -> i32 {
    let n = 32;
    n
}

fn func_non_copy_back() -> String {
    let s = String::from("hello");
    s
}

fn get_mess(mark: i32) -> &'static str {
    match mark {
        100 => "A",
        90 => "B",
        80 => "C",
        70 => "D",
        _ => "F",
    }
}
#[derive(Debug)]
struct MyError {
    details: String,
}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.details)
    }
}

fn main() {
    let s = get_mess(100);
    println!("{}", s);

    let s1 = func_non_copy_back();
    println!("{}", s1);
}
