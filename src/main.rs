fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;
    use std::sync::Barrier;
    use std::sync::Once;
    use std::thread::{self, JoinHandle};
    use std::time::Duration;
    use std::{error, result};

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 1..=5 {
                println!("counter :{}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("application done lagi");
        thread::sleep(Duration::from_secs(7));
    }

    #[test]
    fn test_join_thread() {
        let handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("counter :{}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }
            return counter;
        });

        println!("waiting Handle");

        let result = handle.join();
        match result {
            Ok(counter) => println!("Total Counter : {}", counter),
            Err(error) => println!("Error : {:?}", error),
        }
        println!("application done lagi");
        //thread::sleep(Duration::from_secs(7));
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        for i in 1..=5 {
            println!("counter: {}", i);
            thread::sleep(Duration::from_secs(1));
            counter = counter + 1;
        }
        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();
        println!("total counter 1 {}", result1);
        println!("total counter 2 {}", result2);
        println!(" application finish");
    }

    #[test]
    fn test_parallel() {
        let handle1: JoinHandle<i32> = thread::spawn(|| calculate());
        let handle2: JoinHandle<i32> = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();
        match result1 {
            Ok(counter) => println!("total counter 1 :{}", counter),
            Err(error) => println!("total counter 2 :{:?} ", error),
        }
        match result2 {
            Ok(counter) => println!("total counter 2:{}", counter),
            Err(error) => println!("error : {:?}", error),
        }
        println!("application finish");
    }

    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send("Hello from thread".to_string());
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message)
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from thread".to_string());
            }
            sender.send("Exit".to_string());
        });

        let handle2 = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();
            if message == "Exit" {
                break;
            }
            println!("{}", message)
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }
    #[test]
    fn test_channel_queue_iter() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from thread".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handles = vec![];
        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                println!("Join Game - {}", i);
                barrier_clone.wait();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
            TOTAL_INIT.call_once(|| {
                println!("Call Once");
                TOTAL_COUNTER += 1;
            });
            TOTAL_COUNTER
        }
    }

    #[test]
    fn test_once() {
        let mut handles = vec![];
        for i in 0..10 {
            let handle = thread::spawn(move || {
                let total = get_total();
                println!("Total : {}", total);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
