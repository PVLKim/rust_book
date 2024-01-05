use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // bind works like constructor
    for stream in listener.incoming() { // .incoming() returns an iterator that gives a seq of streams
        let stream = stream.unwrap(); // single stream repr open connection betw client & server
                                                 // connection is a full request/resp process
                                                 // reason why it may fail is because iteration is done over connection attempts, which might fail
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); // BufReader adds buffering by managing calls to the std::io::Read trait methods for us
    let request_line = buf_reader // BufReader implements std::io::BufRead trait, which provides .lines
        .lines() // returns an iterator of results by splitting stream of data whenever it sees newline byte
        .next() // we only care about the first line here
        .unwrap()
        .unwrap();

    let (status_line, filename) = 
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.xhtml")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.xhtml")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();

    
    // let http_request: Vec<_> = buf_reader 
    //     .lines() 
    //     .map(|result| result.unwrap()) // this retrieves the String
    //     .take_while(|line| !line.is_empty()) // browser usually signals end of http request by sending two newline chars in a row
    //     .collect();

}

