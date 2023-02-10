// 要想这里出现运行按钮 main.rs 中需要有 mod string_test;

// Rust 的属性标记，基本格式：#[xxx]  (是不是类似于 Java 中的注解？)
// 在函数上加上 #[test] 可以把函数变为测试函数

// cargo.toml [features]表中的配置项与条件编译功能相关。
// #[cfg], 叫做条件编译属性，该属性允许编译器按指定的标记选择性地编译代码。

#[test]
fn basic_test() {
    let resutl = if 1 == 1 {
        "hello"
    } else {
        "world"
    };

    println!("{:?}", resutl);
}

// 在编译时一切无法确认大小或者大小可以改变的数据，都无法安全地放在栈上，最好放在堆上。
// 除了动态大小的内存需要被分配到堆上外，动态生命周期的内存也需要分配到堆上。

#[test]
fn size_test() {
    // 首先，“hello world” 作为一个字符串常量（string literal），
    // 在编译时被存入可执行文件的 .RODATA 段（GCC）或者 .RDATA 段（VC++），
    // 然后在程序加载时，获得一个固定的内存地址。
    //
    // 当执行 “hello world”.to_string() 时，在堆上，一块新的内存被分配出来，
    // 并把 “hello world” 逐个字节拷贝过去。
    // 当我们把堆上的数据赋值给 s 时，s 作为分配在栈上的一个变量，它需要知道堆上内存的地址，
    // 另外由于堆上的数据大小不确定且可以增长，我们还需要知道它的长度以及它现在有多大。
    //
    // 最终，为了表述这个字符串，我们使用了三个 word：
    // 第一个表示指针、第二个表示字符串的当前长度（11）、第三个表示这片内存的总容量（11）。
    // 在 64 位系统下，三个 word 是 24 个字节。

    let s = "hello world".to_string();

    //  String 在 Rust 中是一个智能指针，它内部是一个结构体，放在栈上，结构体中有指针指向堆内存。
    // 所以 &s 指向一个栈上的地址。
    //
    // {:p} 是通过 Pointer trait 实现。你可以看它的文档：https://doc.rust-lang.org/std/fmt/trait.Pointer.html。
    //
    // 下面的代码可以更好地理解数据在内存的什么位置：
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5ca2cfb1d03936eae4b9a77a40d9987b
    println!("addr of ss: {:p}, s: {:p}, len: {}, capacity: {}, size: {}",
             &"hello world", &s, s.len(), s.capacity(), std::mem::size_of_val(&s));
}