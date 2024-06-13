use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};



struct TPool {
    threads : Vec<Work>,
    sender: Sender<Msg>,
}
enum  Msg{
    NewJob(Job),
    Stop
}
impl Drop for TPool {
    fn drop(&mut self) {
        for _ in &mut self.threads {
            self.sender.send(Msg::Stop).unwrap();
        }
        println!("Shuttinng down all works.");
        for work in &mut self.threads {
            if let Some(thread) = work.thread.take() {
                print!("Shutting down work {}",work.id);
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
struct Work{
    id :usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Work {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Msg>>>) -> Work {
        let handle = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv().unwrap() {
                Msg::NewJob(job) => {
                    println!("work {} got a job,exec...",id);
                    job();
                }
                Msg::Stop => {
                    println!("work {} was told to terminate.",id);
                    break;
                }
            };


        });
        Work{id,thread: Some(handle)}
    }
}

impl TPool {
    fn new(size: usize) ->TPool {
        assert!(size>0);
        let cannel = mpsc::channel();
        let receiver = Arc::new(Mutex::new(cannel.1));

        let mut threads = Vec::with_capacity(size);
        for n in 0..size {
            let work = Work::new(n,Arc::clone(&receiver));
            threads.push(work);
        }
        TPool{ threads,sender:cannel.0 }
    }

    pub(crate) fn execute<F>(&self, f:F)
       where
           F: FnOnce() ,
           F: Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Msg::NewJob(job)).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = TPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();


        pool.execute(||{
            handle_connection(stream);
        });

    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}