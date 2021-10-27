use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // prepare a channel
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // prepare workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

// for Graceful Shutdown
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("ThreadPool::drop() - Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("ThreadPool::drop() - Shutting down all workers.");
        for worker in &mut self.workers {
            println!("ThreadPool::drop() - Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Internal
//struct Job;
//type Job = Box<dyn FnOnce() + Send + 'static>;
type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    // error[E0161]: cannot move a value of type dyn FnOnce() + Send: the size of dyn
                    // FnOnce() + Send cannot be statically determined
                    //(*job)();

                    // FnBox: workaround to (*job)() of FnOnce Trait Object
                    job.call_box();

                    // This is valid since Rust 1.35!
                    //job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            };
        }));

        Worker { id, thread }
    }
}

// FnBox: workaround to (*job)() of FnOnce Trait Object
trait FnBox {
    fn call_box(self: Box<Self>);
}

// Blanket Implementation
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}
