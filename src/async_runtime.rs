use lazy_static::lazy_static;
use std::future::Future;
use std::sync::Mutex;
use tokio::runtime::{Builder, Runtime};

lazy_static! {
    static ref RUNTIME: Mutex<Runtime> = Mutex::new(
        // Do not make a new thread since this runtime is only used for network requests
        Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    );
}

pub fn blocking<F: Future>(future: F) -> F::Output {
    RUNTIME.lock().unwrap().block_on(future)
}
