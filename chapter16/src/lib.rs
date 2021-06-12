/// 学习Rust并发相关的知识.
#[cfg(test)]
mod tests {
    mod thread_test {
        use std::thread;
        use std::time::Duration;

        #[test]
        fn spawn_test() {
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} form the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });

            for i in 1..5 {
                println!("hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
            }

            // 等待线程处理完. unwrap = 如果出现错误就 panic 吧
            handle.join().unwrap();
        }

        #[test]
        fn spawn_move_test() {
            let v = vec![1, 2, 3];

            let handle = thread::spawn(move || {
                println!("Here's a vector: {:?}", v);
            });
            handle.join().unwrap();
            // println!("v:{:?}", v); //v不可用,因为已经move给线程了
        }
    }

    mod channel_test {
        use std::rc::Rc;
        use std::sync::{mpsc, Mutex, Arc};
        use std::thread;
        use std::time::Duration;

        #[test]
        fn create_channel() {
            // 创建一个Channel, 发送端跟接收端
            // mpsc = multiple producers, single consumer
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();
            });

            println!("Wait a message");
            let received = rx.recv().unwrap();
            println!("Got: {}", received);
        }

        #[test]
        fn send_multiple_msg_test() {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];
                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }

        #[test]
        fn multiple_send_test() {
            let (tx, rx) = mpsc::channel();
            let tx1 = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            });

            thread::spawn(move || {
                let vals = vec![
                    String::from("more"),
                    String::from("message"),
                    String::from("for"),
                    String::from("you"),
                ];
                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }

        #[test]
        fn mutex_test() {
            let m = Mutex::new(5);
            {
                let mut num = m.lock().unwrap();
                *num = 6;
            }

            println!("m = {:?}", m);
        }

        #[test]
        fn mutex_multiple_thread_test() {
            // 在多线程中应该使用 Arc 来使用多重合指针.
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..10 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();

                    *num += 1;
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("Result: {}", *counter.lock().unwrap());
        }
    }
}
