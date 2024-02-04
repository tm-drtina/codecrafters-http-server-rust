
pub enum Status {
    Ok,
    NotFound,   
}

impl Status {
    pub fn repr(&self) -> &'static [u8] {
        match self {
            Status::Ok => b"200 OK",
            Status::NotFound => b"404 Not Found",
        }
    }
}
