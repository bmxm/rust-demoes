
// give_me 是一个泛型函数
// 已编译的目标文件将包含此函数的两个专用副本。
// 可以使用 nm 命令在生成的二进制对象文件中确认这一点
// nm xx | grep "give"
// nm 是 GNU binutils 软件包中的实用程序，用于查看已编译对象文件的符号。
// 通过对二进制文件执行 nm 命令，可以使用 pipe 和 grep 查找 give_me
// 函数的前缀。

fn give_me<T>(value: T) -> T {
    value
}

#[test]
fn give_me_test() {
    let v = give_me("hello");
    println!("{}", v)
}

struct Container<T> {
    item: T
}

// impl 后面也需要 <T>
impl<T> Container<T> {
    // 这里的 new 方法莫非是个 特殊函数 ？
    // 返回值居然是个 Self ?
    fn new(item: T) -> Self {
        Container {item}
    }
}

// 这里 impl 后面没有指定类型
// 它是 impl 代码块的另一个特性，它允许你通过独立实现方法来专门化泛型
impl Container<u32> {
    // 这里的 new 方法莫非是个 特殊函数 ？
    // 返回值居然是个 Sel
    fn sum(item: u32) -> Self {
        Container {item}
    }
}


#[test]
fn container_new_test() {
   let c = Container::new("hello");
    println!("{}", c.item);
}

#[test]
fn container_sum_test() {
    let _ = Container::sum(32);
}

#[test]
fn vec_test() {
    // 如果只有这一行，编译器是会报错的，因为编译器不知道 vec 的类型
    let mut v1 = Vec::new();
    // 我们可以调用某个方法, 现在 v1 的类型是 Vec<i32>
    v1.push(1);

    // 或者提供一种类型
    // let v2: Vec<u8> = Vec::new();

    // 或者使用 turbofish 符号
    // let v3 = Vec::<u8>::new();
}