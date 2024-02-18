use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connexion established !");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    let request_line = http_request.get(0).unwrap();

    if request_line == "GET / HTTP/1.1" {
        let response = build_response(OK, "index.html");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = build_response(NOT_FOUND, "404.html");
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn build_response(http_status: HttpStatus, html_file_path: &str) -> String {
    let HttpStatus { status_code, status_name} = http_status;
    let status_line = format!("HTTP/1.1 {status_code} {status_name}");
    let contents = fs::read_to_string(html_file_path).unwrap();

    let length = contents.len();
    let content_length = format!("Content-Length: {length}");
    let headers = format!("{content_length}");

    let response = format!("{status_line}\r\n{headers}\r\n\r\n{contents}");
    response
}

const OK: HttpStatus = HttpStatus { status_code: 200, status_name: "OK" };
const NOT_FOUND: HttpStatus = HttpStatus { status_code: 404, status_name: "NOT_FOUND" };

struct HttpStatus<'a> {
    status_code: u64,
    status_name: &'a str,
}