use thiserror::Error;

use crate::Status;

#[derive(Debug, Error)]
pub enum Error {
    #[error("the FPGA returned an invalid enum discriminant: {0}")]
    InvalidEnumDiscriminant(u64),
    #[error("an FPGA operation failed: {0}")]
    FPGA(Status),
}
