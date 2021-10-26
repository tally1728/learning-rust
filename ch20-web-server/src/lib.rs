use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
        self.sender.send(Box::new(f)).unwrap();
    }
}

// Internal
//struct Job;
//type Job = Box<dyn FnOnce() + Send + 'static>;
type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);

            // error[E0161]: cannot move a value of type dyn FnOnce() + Send: the size of dyn
            // FnOnce() + Send cannot be statically determined
            //(*job)();

            // FnBox: workaround to (*job)() of FnOnce Trait Object
            job.call_box();

            // This is valid since Rust 1.35!
            //job();
        });
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
