use std::thread;
use std::time::Duration;

pub fn simple_api(){
    let v = vec![1,2,3];
    let handle =thread::spawn(move ||{
        for i in 1..10{
            println!("capture the man Vec v {:?}",v);
            println!("spawned thread:`{{`{}`}}`",i);
            thread::sleep(Duration::from_millis(1));
        
        }
    });
//    drop(v);
    for i in 1..2{
        println!("main thread:`{{`{}`}}`",i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}