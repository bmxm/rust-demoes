// Pattern Destructure 模式解构 或 模式匹配
// Destructure 意思是把原来的结构肢解为单独的、局部的、原始的部分

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn main() {
    let (a, b) = (1, 2);
    println!("{}", b);
    assert_eq!(1, a);

    let point = Point{x: 3, y:4};
    println!("{}", point.x);
    println!("{}", point.y);
}