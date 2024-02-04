
pub enum Status {
    Ok,
    Created,
    NotFound,   
}

impl Status {
    pub fn repr(&self) -> &'static [u8] {
        match self {
            Status::Ok => b"200 OK",
            Status::Created => b"201 Created",
            Status::NotFound => b"404 Not Found",
        }
    }
}
