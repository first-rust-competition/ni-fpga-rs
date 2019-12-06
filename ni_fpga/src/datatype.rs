use crate::ffi;
use crate::Offset;
use crate::Session;
use crate::Status;

pub trait Datatype: Copy {
    const FFI_READ: ffi::Func<*mut Self>;
    const FFI_WRITE: ffi::Func<Self>;

    fn read(
        session: &Session,
        offset: Offset,
    ) -> Result<Self, Status> {
        let mut target = std::mem::MaybeUninit::<Self>::uninit();
        let status = Status::from(unsafe {
            Self::FFI_READ(
                session.handle,
                offset,
                target.as_mut_ptr(),
            )
        });
        match status {
            Status::Success => Ok(unsafe { target.assume_init() }),
            _ => Err(status),
        }
    }

    fn write(
        session: &Session,
        offset: Offset,
        value: Self,
    ) -> Result<(), Status> {
        let status = Status::from(unsafe {
            Self::FFI_WRITE(
                session.handle,
                offset,
                value,
            )
        });
        match status {
            Status::Success => Ok(()),
            _ => Err(status),
        }
    }
}

impl Datatype for bool {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadBool(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteBool(session, offset, value)
    };
}

impl<const N: usize> Datatype for [bool; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayBool(session, offset, target as *mut bool, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayBool(session, offset, value.as_ptr(), N)
    };
}
