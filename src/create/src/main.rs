
// 这是我根据 IDEA 右键的导航创建的一个 Cargo Create
fn main() {
    println!("Hello, world!");
}

// create 包
// module 模块

// create 是 rust 的基本编译单元
//   库 create : 没有 main 函数
//   二进制 create ： 有 main 函数
// 一个工程最多包含一个库 create, 但可以包含任意多个二进制 create ？
// 至少应包含一个 create，可以是二进制 create，也可以是库 create ？
// 每个 create 默认有一个隐式的根模块，也就是 src/main.rs 或者 src/lib.rs

// create 关键字表示当前 create， create::a::b::c 表示从根模块开始的绝对路径。
// self 关键字表示当前模块， self::a 表示当前模块中的子模块。
// super 关键字表示当前模块的父模块， super::a 表示当前模块的父模块中的子模块a。


// module 可以将 create 中的代码按功能进行分组，以提高可读性和重用性。
// 模块可以控制函数或类型定义的私有性。
// Rust 通过在 mod 关键字后跟模块名来定义模块，或者引用另一个文件中的模块。
// 模块名之后的大括号 "{}" 中是模块的内容。
mod chinese {
    // 子模块位于父模块的命名空间，可以使用 "::" 来访问子模块
    mod greetings {}
    mod farewells {}
}
// 一般将每个模块放在独立文件中，
// 如果 foo 模块没有子模块，则将 foo 模块放到 foo.rs 文件中
// 如果 foo 模块有子模块，可以考虑将其子模块放在 foo/文件夹（推荐）
// 或者将 foo 模块放在 foo/mod.rs, 将其子模块放在 foo/文件夹

// 1 如果 main.rs、lib.rs 和 mod.rs 文件中出现 "mod xx;" 默认优先寻找同级别文件夹下的 xx.rs 文件;
// 2 如果同级别下 xx.rs 文件没找到，则寻找 xx/mod.rs 文件;
// 3 如果其他文件如 yyy.rs 文件中出现 "mod xx;" 默认寻找 yyy/xx.rs 文件。
// Rust 通过这样的规则，不断迭代实现对深层文件夹下的模块的加载。

// Rust 默认所有模块及模块中的函数和类型都是私有的，
// 若需修改如： pub mod chinese;


// Rust 内置了 Cargo，并将其作为 create 关联器。
// cargo new bar --lib
// 创建库 create