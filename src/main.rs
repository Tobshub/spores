use spores::Request;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2442").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // the first line of the http request
    let request = Request::new(
        buf_reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>(),
    );

    println!("{:?}", request);

    let (status_line, filename) = match &request.path[..] {
        "/" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
