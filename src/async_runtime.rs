use std::future::Future;
use std::sync::Mutex;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new().unwrap());
}

pub fn blocking<F: Future>(future: F) -> F::Output {
    RUNTIME.lock().unwrap().block_on(future)
}
