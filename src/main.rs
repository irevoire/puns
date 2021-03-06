use rouille::Response;
use std::sync::{mpsc::sync_channel, Arc, Mutex};
use std::thread;

mod pun;

fn main() {
    let filename = "./data/puns.csv";
    let mut pun = pun::Pun::new(filename).unwrap();

    let (sender, receiver) = sync_channel(1);

    thread::spawn(move || loop {
        sender.send(pun.get()).unwrap();
    });

    let receiver = Arc::new(Mutex::new(receiver));

    rouille::start_server("localhost:8787", move |_| {
        let pun = receiver.lock().unwrap().recv().unwrap();
        Response::text(pun)
    });
}
