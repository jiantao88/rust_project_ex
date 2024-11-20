use lib3::lib3_type::types::creat_lib3_struct;

pub struct Lib1Struct<T> {
    name: T,
    num: i32,
}

pub fn creat_lib1_struct() -> Lib1Struct<String> {
    let s = Lib1Struct {
        name: String::from("hello"),
        num: 10,
    };

    print!("lib1_struct created!");
    creat_lib3_struct();
    s
}
