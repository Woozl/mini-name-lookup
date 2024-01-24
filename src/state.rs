use tokio::sync::Mutex;

pub enum Status {
    /// Data is being loaded
    Loading,
    Indexing,
    Available,
}

#[derive(Debug)]
pub struct AppState {
    pub count: Mutex<usize>,
    // pub status: Status,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            count: Mutex::new(0),
        }
    }
}
