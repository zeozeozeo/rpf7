mod consts;
mod hash;
mod lut;
mod ng;
mod rpf;

pub use rpf::*;

#[derive(Debug, thiserror::Error)]
pub enum RpfError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("unsupported RPF version {0:#x}, expected {RPF7_MAGIC:#x}")]
    VersionMismatch(u32),
    #[error("failed to convert C string")]
    IntoStringError(#[from] std::ffi::IntoStringError),
}

type RpfResult<T> = Result<T, RpfError>;
