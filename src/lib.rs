use std::thread;
pub struct ThreadPool{
    workers: Vec<Worker>,
};
// 自定义线程池
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some workers and store them in the vector
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
        
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id:usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {
            println("Worker {id} is running");
        });
        // 每个worker都有一个线程，线程的id是worker的id
        Worker { id, thread }
    }
}