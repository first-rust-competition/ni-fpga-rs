use std::{ffi::NulError, num::TryFromIntError, sync::PoisonError};

use thiserror::Error;

use crate::status::Status;

#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum Error {
    #[allow(clippy::upper_case_acronyms)]
    #[error("an FPGA operation failed: {0}")]
    FPGA(Status),
    #[error("the FPGA returned an invalid enum discriminant: {0}")]
    InvalidEnumDiscriminant(u64),
    #[error("{0} is out of bounds for FXP<{1}, {2}, {3}>`")]
    FixedPointOutOfBounds(f64, u8, u8, bool),
    #[error("{0} ({0:b}) is out of bounds for FXP<{1}, {2}, {3}>`")]
    FixedPointRawOutOfBounds(u64, u8, u8, bool),
    #[error("{0} cannot be precisely represented as FXP<{1}, {2}, {3}>`")]
    FixedPointPrecision(f64, u8, u8, bool),
    #[error("Cannot close an unowned fpga session")]
    ClosingUnownedSession,
    #[error("An invalid datatype was passed.")]
    InvalidDatatype,
    #[error("Null C String {0}")]
    NullCString(NulError),
    #[error("No bitfile specified")]
    NoBitfileSpecified,
    #[error("No signature specified")]
    NoSignatureSpecified,
    #[error("No resource specified")]
    NoResourceSpecified,
    #[error("Resource has already been taken")]
    ResourceAlreadyTaken,
    #[error("Register {0} not found")]
    RegisterNotFound(String),
    #[error("A mutex is poisoned {0}. This is unrecoverable")]
    MutexPoisoned(String),
    #[error("Timeout is too big to pass to the FPGA")]
    TimeoutTooLong,
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::TimeoutTooLong
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Self::MutexPoisoned(value.to_string())
    }
}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Self::NullCString(value)
    }
}
