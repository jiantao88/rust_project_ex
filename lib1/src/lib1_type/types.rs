use lib3::lib3_type::types::creat_lib3_struct;

pub struct Lib1_struct<T> {
    name: T,
    num:i32,
}


pub fn creat_lib1_struct() -> Lib1_struct<String> {
    let s = Lib1_struct {  
        name: String::from("hello"),
                num: 10,
    };

    print!("lib1_struct created!");
    creat_lib3_struct();
    s
}