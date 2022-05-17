// 函数参数是一种特殊变量，它是函数签名的一部分。
// Rust 要求函数参数必须明确指定数据类型，但不能指定默认值。
// 函数参数分为可变和不可变参数，默认是不可变参数。
// 当需要可变操作时，可以使用 mut 关键字。
// 函数使用逗号分隔。
// 函数只能有唯一的返回值，如果需要返回多个值，可以使用元组类型。
// Rust 中每个函数都有返回值，即使是没有显式返回值函数，也会隐式的返回一个单元值 ()。
fn add(x: i32, y :i32) -> i32 {
    // 函数体由一系列语句和一个可选的结尾表达式构成。

    x + y // 等同于 return x + y;
    // 对于流程控制结构中的循环或者条件判断分支，如果需要提前退出函数并返回指定的值，
    // 必须显式地使用 return 语句来返回。
}

#[test]
fn add_test() {
    let result = add(2, 4);
    println!("{:?}", result);
}