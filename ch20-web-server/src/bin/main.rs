use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use ch20_web_server::ThreadPool;

fn main() {
    // fn bind<A: ToSocketAddrs>(addr: A) -> Result<TcpListener>
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // fn incoming(&self) -> Incoming
    // struct Incoming: An iterator that infinitely accepts connections on a TcpListener.
    // stream: Result<TcpStream>
    for stream in listener.incoming() {
        // stream: TcpStream
        // struct TcpSteream: A TCP stream between a local and a remote socket.
        let stream = stream.unwrap();
        println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    ////////////////////////////////////////////////////////
    // Read HTTP Request
    let mut buffer = [0; 1024];
    // fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    stream.read(&mut buffer).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    ////////////////////////////////////////////////////////
    // Validate HTTP Request
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        println!("Request: GET / HTTP/1.1");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        println!("Request: GET /sleep HTTP/1.1");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        println!("Request: Wrong Path!");
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    ////////////////////////////////////////////////////////
    // Prepare HTTP Response
    // fn open<P: AsRef<Path>>(path: P) -> Result<File>
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    // fn read_to_string(&mut self, buf: &mut String) -> Result<usize>
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    ////////////////////////////////////////////////////////
    // SendHTTP Response
    // fn write(&mut self, buf: &[u8]) -> Result<usize>
    stream.write(response.as_bytes()).unwrap();
    // fn flush(&mut self) -> Result<()>
    stream.flush().unwrap();
    println!("Status: {}", status_line);
}
