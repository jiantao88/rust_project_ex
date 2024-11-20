pub trait Lib1Trait {
    fn new(&self) -> Self;
    fn print(&self) {
        println!("Hello, it comes from Lib1");
    }
}
