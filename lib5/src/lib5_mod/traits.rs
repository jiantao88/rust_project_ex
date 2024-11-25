// 在这里定义您的特征
pub trait MyTrait {
    // 添加特征方法
    fn new(&self) -> Self;

    fn print(&self) {
        println!("Hello, it comes from Lib5");
    }
}
