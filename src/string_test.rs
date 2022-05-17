// 要想这里出现运行按钮 main.rs 中需要有 mod string_test;

#[test]
fn basic_test() {
    let resutl = if 1 == 1 {
        "hello"
    } else {
        "world"
    };

    println!("{:?}", resutl);
}