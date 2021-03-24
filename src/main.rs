fn main() {
    // chapter3_1();
    // chapter3_2();
    // chapter3_3();

    chapter3_2_1();
}


fn chapter3_2_1() {
    println!("数据类型");
    // 基本类型四种：整型、浮点、布尔、字符型
    /*
        长度	有符号	无符号
        8-bit	i8	u8
        16-bit	i16	u16
        32-bit	i32	u32
        64-bit	i64	u64
        128-bit	i128	u128
        arch	isize	usize
    */


    // 复合类型： 元组（tuple） + 数组（array）
    let tup: (i32, f64, u8) = (500, 50.1, 127);
    // 通过下标直接访问
    println!("tup: {}, {}, {}", tup.0, tup.1, tup.2);

    // 元组解构
    let (x, y, z) = tup;
    println!("x:{}, y:{}, z:{}", x, y, z);

    // 数组
    let a = [1, 2, 3, 4, 5];
    // 数组的下标使用[]
    println!("{}", a[0]);
    let b: [i64; 3] = [3, 4, 5];
    println!("b 1:{},2:{},3:{}", b[0], b[1], b[2]);
    // 定义类型为 i32类型的值 5， 重复50个
    let c = [5; 50];
    println!("c 0:{},24:{},49:{}", c[0], c[24], c[49]);
}

fn chapter3_1() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn chapter3_2() {
    println!("变量与常量");
    const MAX_POINTS: u32 = 100_100;
    println!("cost points:{}", MAX_POINTS);
}

fn chapter3_3() {
    println!("隐藏变量");
    let x = 5;

    let x = x + 1;

    let x = x * 2;
    println!("x:{}", x);

    // 与 mut 的区别：mut可以更改至的内容
    // let创建一个新的变量，并且可以修改新变量的类型

    let spaces = "        ";
    let spaces = spaces.len();
    println!("spaces:{}", spaces);
}