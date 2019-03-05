use cairo::Status;
use failure;

pub use failure::ResultExt;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "renderer backend failure: {}", status)]
    RendererBackend {
        status: Status,
    },
    #[fail(display = "invalid canvas configuration: {}", reason)]
    InvalidCanvasConfiguration {
        reason: &'static str,
    },
    #[fail(display = "error: {}", reason)]
    Other {
        reason: String,
    },
}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Error::RendererBackend {
            status
        }
    }
}

pub type Result<T> = ::std::result::Result<T, failure::Error>;
