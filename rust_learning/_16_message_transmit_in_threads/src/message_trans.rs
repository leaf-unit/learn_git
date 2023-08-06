use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub fn message_trans(){
    let (tx,rx)=mpsc::channel();
    thread::spawn(move||{
        let val = String::from("hi");
        tx.send(val).unwrap();
//        println!("{}",val); //val had been moved

    });
    let received = rx.recv().unwrap();
    println!("Got {}",received);

    let (server,client)=mpsc::channel();
    let server2 = server.clone();
    thread::spawn(move||{
        let words_v = vec![
            "apple".to_string(),
            "banana".to_string(),
            "tomato".to_string()
            ];
        for word in words_v{
            server.send(word).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(move||{
        let words_v = vec![
            "AX".to_string(),
            "BX".to_string(),
            "CX".to_string()
            ];
        for word in words_v{
            server2.send(word).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    for word in client{
        println!("This channal2 ,Got {}",word);
    }
}