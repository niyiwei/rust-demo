use std::collections::hash_map::Entry::Vacant;
// use hello_demo::eat_at_restaurant;

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
    // chapter5_2_3();

    // chapter6_1_1();
    // chapter6_1_2();
    // chapter6_1_3();
    // chapter6_1_4();

    chapter6_1_5();

    // match
    chapter6_2_1();
    chapter6_2_2();

    // eat_at_restaurant();
}

fn chapter6_2_2() {
    let five = Some(5);
    println!("five plus one result: {:?}", plus_one(five));
    let none_value = None;
    println!("none value plus one result: {:?}", plus_one(none_value));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
        // default handler 机制.
        _ => Some(-1),
    }
}

fn chapter6_2_1() {
    let nickel_result = value_in_cents(Coin::Nickel);
    println!("nickel result: {}", nickel_result);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dine => 10,
        Coin::Quarter => 25,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dine,
    Quarter,
}

// option 的使用
fn chapter6_1_5() {
    let some_number = Some(5);
    let some_string = Some("a string");
    // None值必须加上类型
    let none_value: Option<i32> = None;

    // println!("some_number:{}, some_string:{}, none_value:{}", some_number, some_string, none_value);
}

fn chapter6_1_4() {
    let m = Message::Write(String::from("哇哈哈"));
    m.call();
}

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 枚举实现方法
impl Message {
    fn call(&self) {
        println!("message call:{:#?}", self);
    }
}

fn chapter6_1_3() {
    let v4 = IpAddrKind3::V4(127, 0, 0, 1);
    let v6 = IpAddrKind3::V6(String::from("::1"));
    println!("v4:{:#?}, v6:{:#?}", v4, v6);
}

fn chapter6_1_2() {
    let v4 = IpAddrKind2::V4(String::from("127.0.0.1"));
    let v6 = IpAddrKind2::V6(String::from("::1"));
    println!("v4:{:#?}, v6:{:#?}", v4, v6);
}

fn chapter6_1_1() {
    let home = IpAddr {
        kind: IpAddrKind1::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind1::V6,
        address: String::from("::1"),
    };

    println!("home:{:#?}, loopback:{:#?}", home, loopback)
}

// 第六章：定义枚举
#[derive(Debug)]
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKind1 {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind1,
    address: String,
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

fn chapter5_2_3() {
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

