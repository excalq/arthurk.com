use webserver::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("Request: {:#?}", request_line);

    let (status_line, media_type, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "text/html", "index.html"),
        "GET /main.css HTTP/1.1" => ("HTTP/1.1 200 OK", "text/css", "main.css"),
        "GET /night-sky.jpg HTTP/1.1" => ("HTTP/1.1 200 OK", "image/jpeg", "night-sky.jpg"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "text/html", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "text/html", "404.html"),
    };

    let contents = fs::read(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Type: {media_type}\r\nContent-Length: {length}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
    stream.write(&contents).unwrap();
}
