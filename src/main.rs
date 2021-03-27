fn main() {
    // chapter3_1();
    // chapter3_2();
    // chapter3_3();

    // chapter3_2_1();

    // chapter3_3_1(666, 777);
    // chapter3_3_2();
    // chapter3_3_3();
    // chapter3_5_1(2);
    // chapter3_5_2();
    // chapter3_5_3();
    // chapter3_5_4();
    // chapter3_5_5();

    // 结构体
    // chapter5_1_1();
    // chapter5_1_2();
    // chapter5_2_1();
    // chapter5_2_2();
    chapter5_2_3();
}

fn chapter5_2_2() {
    let r1 = Rectangle {
        width: 6,
        height: 6,
    };

    let r2 = Rectangle {
        width: 5,
        height: 5,
    };

    let result = r1.can_hold(&r2);
    println!("r1 can_hold r2:{}", result);
}

fn chapter5_2_3(){
    let result = Rectangle::new(10, 10);
    println!("Rectangle new result: {:?}", result);
    println!("Rectangle new result: {:#?}", result);

    let aaa = Rectangle::area(&result);
    println!("new result function area result: {}", aaa);
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 增加结构体的方法
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 不加 self，静态函数 可以直接调用
    fn new(width: i32, height: i32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}

// 调用结构体的方法
fn chapter5_2_1() {
    let data = Rectangle {
        width: 100,
        height: 100,
    };
    let result = data.area();
    println!("result: {}", result);
}

// 打印结构体
fn chapter5_1_2() {
    let data = Rectangle {
        width: 1000,
        height: 1000,
    };
    println!("output format :? area is: {:?}", data);
    println!("output format :#? area is: {:#?}", data);
}

fn chapter5_1_1() {
    let rectangle = Rectangle {
        width: 10,
        height: 10,
    };
    let result = area(&rectangle);
    println!("area is: {}", result);
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}

// for 循环
fn chapter3_5_5() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


    // rev函数反转数组
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// while 循环
fn chapter3_5_4() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFIOFF!!!")
}

// 循环 loop
fn chapter3_5_3() {
    // 循环1 loop , 类似于 while(true)
    loop {
        println!("this is loop");
        // 使用break跳出循环.
        break;
    }

    let mut counter = 0;
    // loop返回值
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("loop result is: {}", result);
}

// let声明时可以使用条件赋值
fn chapter3_5_2() {
    let condition = true;
    // if 跟 else 返回的类型必须一致.
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number)
}

// 控制流 - if
fn chapter3_5_1(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number > 1 {
        println!("number is grant 1");
    } else if number == 1 {
        println!("number is equals 1");
    } else {
        println!("number is less 1");
    }
}

fn chapter3_3_3() {
    let result = five();
    println!("result is: {}", result);

    let sixResult = six(5);
    println!("input params result is: {}", sixResult);
}

fn six(inData: i32) -> i32 {
    inData + 1
}

// 具有返回值的函数
fn five() -> i32 {
    5
}

// 表达式
fn chapter3_3_2() {
    let x = 5;

    let y = {
        let x = 3;
        // 不加分号就是表达式，加了分号就是语句，语句没有返回值
        x + 1
    };

    println!("The value of y is:{}", y);
}

// 函数参数
fn chapter3_3_1(x: i32, y: i32) {
    println!("The value of x is: {}, y:{}", x, y);
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