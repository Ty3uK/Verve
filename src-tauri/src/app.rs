use std::sync::{Mutex, Arc};

use crate::extensions::{get_extensions, Extension};

pub struct AppState(pub Arc<Mutex<App>>);

pub struct App {
    pub extensions: Vec<Extension>,
}

impl AppState {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(App {
            extensions: get_extensions(),
        })))
    }
}
