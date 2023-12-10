use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
fn main() {
    let mut turn = Box::new(0);
    let mut other = Arc::new(0);
    let mut threads=Vec::new();
    let mut resources = Mutex::new(0);
    for _i in 0..5{
        let th = thread::spawn(move||
        {
            other
            println!("I'm {}",turn);
            thread::sleep(Duration::from_millis(1000));

        });
    threads.push(th);
    }
    for thread in threads{
        thread.join().unwrap();
    }
}
