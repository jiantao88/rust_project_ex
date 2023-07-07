use rand::Rng;

pub struct Lib3_struct<T> {
    _name: T,
    _num:i32,
}


pub fn creat_lib3_struct() -> Lib3_struct<String> {
    let s = Lib3_struct {  
        _name: String::from("hello"),
                _num: 10,
    };

    print!("\nlib3_struct created!");

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..100);
    println!("Generated number: {}", num);
    s
}