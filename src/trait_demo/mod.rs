// 特征是一个元素，它定义了一组类型可以选择性实现的"契约"或共享行为。
// 特征本身没什么用，并且需要根据类型予以实现。
// 特征有能力在不同类型之间建立关联，他们是许多语言特性的基础，例如闭包、运算符、智能指针、循环及编译器数据竞争校验等。

// trait 本质是一组方法的原型，是实现某些目的的行为集合。
// 在 Rust 中 trait 是唯一的接口抽象方式。
// Rust 通过 trait 将类型和行为明确地进行了区分，充分贯彻了 组合优于继承 和 面向接口编程 的编程思想。

// 定义两个元组结构体
struct Audio(String);
struct Video(String);

// 定义一个名为 Playable 的特征
pub trait Playable {
    // 特征里面可以有 零个或多个方法
    // 任何可实现特征的类型都应该对其提供具体实现。
    // 我们还可以在特征中定义常量，所有实现者都可以共享它。
    // 实现者可以是任何结构体、枚举、基元类型、函数及闭包，甚至特征。

    fn play(&self);
    fn pause() {
        println!("Paused");
    }

    // 关联方法 （具体方法）
    // 不需要类型的实例来调用，在其他编程语言中也被称为静态方法，
    // 如：标准库的特征 FromStr 的 from_str 方法。
    // 它是通过 String 实现的，因此允许你通过 String::from_str("foo") 从 &str 创建一个 String。

    // 实例方法 （抽象方法）
    // 这些方法需要将 self 作为其第一个参数。这仅适用于实现特征的类型实例, self 将指向实现特征的类型实例。
    // 它可以有 3 种类型：self, &self, &mut self
}

// 我们使用关键字 impl 后跟特征名称来声明特征实现
// 随后是关键字 for 和 希望实现的 特征类型，
// 其后的花括号用于编写特征实现。
impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0)
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0)
    }
}

#[test]
fn playable_test() {
   // let _ = String::from_str("foo");

    let audio = Audio("xx.mp3".to_string());
    let video = Video("xx.mkv".to_string());

    audio.play();
    video.play();

    // 这里会执行报错，抽空研究一下怎么用的？
    // Playable::pause();
}


// 特征也可以在声明中表明他们依赖于其他特征-这是一种被称为继承的特性。

trait Vehicle {
    fn get_price(&self);
}