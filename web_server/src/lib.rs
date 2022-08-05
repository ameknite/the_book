pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}
