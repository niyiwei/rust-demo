#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    mod box_test {
        use std::ops::Deref;

        use crate::tests::box_test::List::{Cons, Nil};

        #[test]
        fn box_new() {
            let b = Box::new(5);
            println!("b = {}", b);
        }

        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        #[test]
        fn recursive_test() {
            // let list = Cons(1, Cons(2, Cons(3, Nil)));

            let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
            println!("list:{:#?}", list);
        }

        #[test]
        fn deref_trait_test() {
            let x = 5;
            let y = &x;
            println!("x:{}, y:{}", x, y);
            assert_eq!(5, x);
            // y是引用, 所以需要解引用.
            assert_eq!(5, *y);
        }

        #[test]
        fn deref_box_test() {
            let x = 5;
            let y = Box::new(x);
            assert_eq!(5, x);
            // 为啥也可以解引用?
            assert_eq!(5, *y);
        }

        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;

            // 解引用方法
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Debug)]
        struct Abc {}

        impl Drop for Abc {
            fn drop(&mut self) {
                println!("[Abc] - [drop] - [{:#?}]", self)
            }
        }

        #[test]
        fn abc_drop_test() {
            let abc = Abc {};
            println!("abc:{:?}", abc);
            println!("提前执行drop方法...");
            std::mem::drop(abc);
            // println!("abc:{:?}", abc);
            println!("over!");
        }

        #[test]
        fn my_box_deref_test() {
            let x = 5;
            let y = MyBox::new(x);

            assert_eq!(5, x);
            assert_eq!(5, *y);

            let a = "123";
            let b = MyBox::new(a);
            assert_eq!("123", a);
            assert_eq!("123", *b);
        }


        // Rc指针 ， 只鞥呢在单线程中使用
        #[test]
        fn rc_test() {
        }
    }
}


pub mod chapter15 {
    /// 智能指针
    /// Box<T> 可用于在堆上分配值
    /// Rc<T> 允许多重所有权的引用计数类型
    /// Ref<T> 和 RefMut<T> 可以通过RefCell<T>访问, 是一种可以在运行时而不是编译时执行借用规则的类型
    pub mod pointer {}
}

