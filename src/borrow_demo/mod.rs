
#[test]
fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    // 只读引用实现了 Copy trait，也就意味着引用的赋值、传参都会产生新的浅拷贝
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}


// 生命周期更长的 main2() 函数变量 r ，引用了生命周期更短的 local_ref() 函数里的局部变量，这违背了有关引用的约束，所以 Rust 不允许这样的代码编译通过。
// #[test]
// fn main2() {
//     let r = local_ref();
//     println!("r: {:p}", r);
// }
//
// fn local_ref<'a>() -> &'a i32 {
//     let a = 42;
//     &a
// }

#[test]
fn main3() {
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data: {:?}", data);
}

#[test]
fn main4() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
}

// 堆变量的生命周期不具备任意长短的灵活性，因为堆上内存的生死存亡，跟栈上的所有者牢牢绑定。
// 而栈上内存的生命周期，又跟栈的生命周期相关，所以我们核心只需要关心调用栈的生命周期。
//fn push_local_ref(data: &mut Vec<&u32>) {
    // 不能引用生命周期更短的值
    // let v = 42;
    // data.push(&v);
//}


// --- 可变引用

#[test]
fn main5() {
    // 这段代码在遍历可变数组 data 的过程中，还往 data 里添加新的数据，这是很危险的动作，因为它破坏了循环的不变性（loop invariant），容易导致死循环甚至系统崩溃。
    // 所以，在同一个作用域下有多个可变引用，是不安全的。
    // let mut data = vec![1, 2, 3];
    //
    // for item in data.iter_mut() {
    //     data.push(*item + 1);
    // }
}

#[test]
fn main6() {
    // 不可变数组 data1 引用了可变数组 data 中的一个元素，这是个只读引用。
    // 后续我们往 data 中添加了 100 个元素，在调用  data.push() 时，我们访问了 data 的可变引用。
    // 这段代码中，data 的只读引用和可变引用共存，似乎没有什么影响，因为 data1 引用的元素并没有任何改动。
    // 如果你仔细推敲，就会发现这里有内存不安全的潜在操作：如果继续添加元素，堆上的数据预留的空间不够了，就会重新分配一片足够大的内存，把之前的值拷过来，然后释放旧的内存。
    // 这样就会让 data1 中保存的 &data[0] 引用失效，导致内存安全问题。

    // let mut data = vec![1, 2, 3];
    // let data1 = vec![&data[0]];
    // println!("data[0]: {:p}", &data[0]);
    //
    // for i in 0..100 {
    //     data.push(i);
    // }
    //
    // println!("data[0]: {:p}", &data[0]);
    // println!("boxed: {:p}", &data1);
}

// 为了保证内存安全，Rust 对可变引用的使用也做了严格的约束：
//  在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
//  在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。