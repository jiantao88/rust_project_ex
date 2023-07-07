pub trait Lib2_trait {
    fn new(&self) -> Self;
    fn print(&self) {
        println!("Hello, it comes from Lib1");
    }
}