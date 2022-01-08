use std::fmt::Formatter;

fn main() {
    println!("Learning print");

    println!("Expect: abc!{}", "abc");

    println!("Expect: 1 2 1!{0} {1} {0}", "1", "2");

    println!("Expect: aaa!{abc}", abc = "aaa");

    println!("Expect     5!{number:0width$}", number = 5, width = 5);
    println!("Expect 00005!{number:>0width$}", number = 5, width = 5);

    #[derive(Debug)]
    struct Abc(i32);

    println!("Print struct:{:?}", Abc(1));
    println!("Print struct:{:#?}", Abc(2));

    println!("{:?}", 100);

    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor = "actor's");

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let person = Person { name: "abc", age: 85 };
    println!("person:{:?}", person);
    println!("person:{:#?}", person);


    // 使用fmt::Display 定制打印
    #[derive(Debug)]
    struct Dog {
        name: &'static str,
        age: u8,
    }

    impl std::fmt::Display for Dog {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dog name is:{}, age is:{}", self.name, self.age)
        }
    }

    let dog = Dog { name: "aDai", age: 1 };
    println!("Dog:{}", dog);

    struct MyList(Vec<i32>);

    impl std::fmt::Display for MyList {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let value = &self.0;

            write!(f, "[")?;

            for (count,v) in value.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?
            }

            write!(f, "]")
        }
    }
    println!("MyList:{}", MyList(vec![1, 2, 3, 4, 5]));
}

mod tests {}