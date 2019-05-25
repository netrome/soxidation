use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::net::{TcpListener, TcpStream};


pub fn hello(){
    println!("Hello for chat");
}


pub fn host(address: &str){
    let listener = TcpListener::bind(address).unwrap();

    println!("Awaiting connection...");
    let mut stream = listener.accept().unwrap().0;
    println!("Accepted a connection!");

    listen(stream);
}

pub fn join(address: &str){
    let mut stream = TcpStream::connect(address).unwrap();

    talk(stream);

}

fn listen(mut stream: TcpStream){
    let mut reader = BufReader::new(stream);
    while true{
        let mut buffer = String::new();
        reader.read_line(&mut buffer);
        println!("{}", buffer);

        if buffer.len() < 3{
            panic!("Doh");
        }
    }
}

fn talk(mut stream: TcpStream){
    while true{
        let mut buffer = String::new();
        print!("➽ ");
        io::stdout().flush();
        io::stdin().read_line(&mut buffer).ok().unwrap();
        stream.write_all(&mut buffer.into_bytes());
    }
}

fn chat(mut stream: TcpStream){
    // Prints everything written to the stream and writes all input to the stream
}

