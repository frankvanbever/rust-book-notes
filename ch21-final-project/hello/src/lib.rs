use std::thread;


pub struct ThreadPool {
    workers: Vec<Worker>,
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

        let mut workers= Vec::with_capacity(size);
        for id in 0..size {
            let worker = Worker::new(id);
            // create some threads and store them in the vector
            workers.push(worker);
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
    id: usize,
    work: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        Worker {
            id,
            work: thread::spawn(|| {})
        }
    }
}
