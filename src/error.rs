#[derive(Debug)]
pub enum Error {
    /// r2d2 error wrapper
    R2D2(r2d2::Error),
    /// rusqlite error wrapper
    Rusqlite(rusqlite::Error),
    Redis(redis::RedisError)
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::Rusqlite(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Error {
        Error::R2D2(err)
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Error {
        Error::Redis(err)
    }
}
