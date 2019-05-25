use std::thread;
use std::env;

use std::io;
use std::io::prelude::*;

mod chat;

fn main() {
    println!("Hello, world!");
    chat::hello();

    let args: Vec<String> = env::args().collect();

    let address_1 = "127.0.0.1:8123";
    let address_2 = "127.0.0.1:8124";

    if args.len() > 1{
        let t = thread::spawn(move || chat::host(address_1));

        let mut buffer = String::new();
        io::stdout().flush();
        io::stdin().read_line(&mut buffer).ok().unwrap();
        println!("Starting outgoing chat!");

        let t = thread::spawn(move || chat::join(address_2));

        t.join();
    }else{
        let t = thread::spawn(move || chat::host(address_2));

        let mut buffer = String::new();
        io::stdout().flush();
        io::stdin().read_line(&mut buffer).ok().unwrap();
        println!("Starting outgoing chat!");

        let t = thread::spawn(move || chat::join(address_1));

        t.join();
    }
}

