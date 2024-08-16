use std::{
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread,
};
// 定义任务类型
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    // 为 sender 增加 Option 封装，这样可以用 take 拿走所有权，跟之前的 thread 一样
    sender: Option<mpsc::Sender<Job>>,
}

// 自定义线程池
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some workers and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
// 优雅关闭线程池
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 主动调用 drop 关闭发送端 sender
        drop(self.sender.take());
        // 关闭所有线程
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 注意这种 if let 的写法，若 worker.thread 已经是 None，什么都不会发生，
            // 符合我们的预期; 若包含一个线程，那就拿走其所有权，然后调用 join。
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // 创建一个新的worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // 创建一个新的线程，并将它绑定到一个闭包中，该闭包接收一个任务并执行它
        let thread = thread::spawn(move || loop {
            /* 
             * 这里不能使用 while let 的原因
             * Mutex 结构体没有提供显式的 unlock，要依赖作用域结束后的 drop 来自动释放
             * let job = receiver.lock().unwrap().recv().unwrap(); 在这行代码中，由于使用了 let，右边的任何临时变量会在 let 语句结束后立即被 drop，因此锁会自动释放
             * 然而 while let (还包括 if let 和 match) 直到最后一个花括号后，才触发 drop
             */
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        // 每个worker都有一个线程，线程的id是worker的id
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
