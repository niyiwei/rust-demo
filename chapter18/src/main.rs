mod test {
    mod match_test {
        #[test]
        fn match_test() {
            let favorite_color: Option<&str> = None;
            let is_tuesday = false;
            let age: Result<u8, _> = "34".parse();
            if let Some(color) = favorite_color {
                println!("Using your favorite color, {} as the background", color);
            } else if is_tuesday {
                println!("Tuesday is green day!");
            } else if let Ok(age) = age {
                if age > 30 {
                    println!("Using purple as the background color");
                } else {
                    println!("Using orange as the background color");
                }
            } else {
                println!("Using blue as the background color");
            }
        }

        #[test]
        fn while_let_test() {
            let mut stack = Vec::new();
            stack.push(1);
            stack.push(2);
            stack.push(3);

            while let Some(top) = stack.pop() {
                println!("{}", top);
            }
        }

        #[test]
        fn params_test() {
            let (x, y, z) = (1, 2, 3);
            assert_eq!(1, x);
            assert_eq!(2, y);
            assert_eq!(3, z);
        }

        #[test]
        fn method_params_test() {
            let point = (3, 5);
            print_coordinates(&point);
        }

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }

        #[test]
        fn match_value_test() {
            let x = 1;
            match x {
                1 => println!("one"),
                2 => println!("one"),
                3 => println!("one"),
                4 => println!("one"),
                _ => println!("anything"),
            }
        }

        #[test]
        fn match_named_variable_test() {
            let x = Some(5);
            let y = 10;
            match x {
                // 匹配x里面的值
                Some(50) => println!("Got 50"),
                // 这里的y是新赋予的值，表示 x只要是有值则进去，不是我们定义的那个y
                Some(y) => println!("Matched, y = {:?}", y),
                // 兜底匹配.
                _ => println!("Default case, x={:?}", x),
            }
            println!("at the end: x={:?}, y={:?}", x, y);
        }

        #[test]
        fn multiple_match_test() {
            let x = 1;
            match x {
                1 | 2 => println!("one or two"),
                3 => println!("three"),
                _ => println!("anything"),
            }
        }

        #[test]
        fn match_in_test() {
            let x = 5;
            match x {
                1..=5 => {
                    println!("one through five");
                }
                _ => {
                    println!("anything");
                }
            }

            let x = 'c';
            match x {
                'a'..='j' => {
                    println!("early ASCII letter");
                }
                'k'..='z' => {
                    println!("later ASCII letter");
                }
                _ => {
                    println!("something else");
                }
            }
        }

        #[test]
        fn point_test() {
            struct Point {
                x: i32,
                y: i32,
            }

            let p = Point { x: 0, y: 7 };
            let Point { x: a, y: b } = p;
            assert_eq!(0, a);
            assert_eq!(7, b);

            // 如果字段匹配,则可以直接用.
            let Point { x, y } = p;
            assert_eq!(0, x);
            assert_eq!(7, y);

            match p {
                Point { x, y: 0 } => {
                    println!("One the x axis at {}", x);
                }
                Point { x: 0, .. } => {
                    println!("One the y axis at {}", x);
                }
                Point { x, y } => {
                    println!("On neither axis: ({}, {})", x, y);
                }
            }
        }

        #[test]
        fn match_enum_test() {
            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }
            let msg = Message::ChangeColor(0, 160, 255);
            match msg {
                Message::Quit => {
                    println!("The quit variant has no data to destructure.");
                }
                Message::Move { x, y } => {
                    println!("Move in the x direction {} and in the y direction {}", x, y);
                }
                Message::Write(text) => {
                    println!("Text message: {}", text);
                }
                Message::ChangeColor(r, g, b) => {
                    println!("Change the color to red {}, green {}, and blue {}", r, g, b);
                }
            }
        }

        #[test]
        fn match_where_test() {
            let num = Some(4);
            match num {
                // 有值,并且等于6
                Some(6) => println!("value is six"),
                // 有值, 并且增加if条件
                Some(x) if x < 5 => println!("less than five: {}", x),
                // 有值就行
                Some(x) => println!("{}", x),
                None => (),
            }

            let x = 4;
            let y = false;
            match x {
                4 | 5 | 6 if y => println!("yes"),
                _ => println!("no"),
            }
        }
    }
}

fn main() {
    println!("Chapter18 条件匹配.");
}
