pub struct Lib2_struct<T> {
    name: T,
    num:i32,
}


pub fn creat_lib2_struct() -> Lib2_struct<String> {
    let s = Lib2_struct {  
        name: String::from("hello"),
                num: 10,
    };

    print!("\nlib2_struct created!");
    s
}