use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //println!("{i} Connection establised!");

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
    }
}
