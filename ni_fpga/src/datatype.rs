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

impl Datatype for u8 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadU8(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteU8(session, offset, value)
    };
}
impl<const N: usize> Datatype for [u8; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayU8(session, offset, target as *mut u8, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayU8(session, offset, value.as_ptr(), N)
    };
}

impl Datatype for u16 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadU16(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteU16(session, offset, value)
    };
}
impl<const N: usize> Datatype for [u16; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayU16(session, offset, target as *mut u16, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayU16(session, offset, value.as_ptr(), N)
    };
}

impl Datatype for u32 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadU32(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteU32(session, offset, value)
    };
}
impl<const N: usize> Datatype for [u32; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayU32(session, offset, target as *mut u32, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayU32(session, offset, value.as_ptr(), N)
    };
}

impl Datatype for u64 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadU64(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteU64(session, offset, value)
    };
}
impl<const N: usize> Datatype for [u64; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayU64(session, offset, target as *mut u64, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayU64(session, offset, value.as_ptr(), N)
    };
}

impl Datatype for i8 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadI8(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteI8(session, offset, value)
    };
}
impl<const N: usize> Datatype for [i8; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayI8(session, offset, target as *mut i8, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayI8(session, offset, value.as_ptr(), N)
    };
}

impl Datatype for i16 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadI16(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteI16(session, offset, value)
    };
}
impl<const N: usize> Datatype for [i16; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayI16(session, offset, target as *mut i16, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayI16(session, offset, value.as_ptr(), N)
    };
}

impl Datatype for i32 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadI32(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteI32(session, offset, value)
    };
}
impl<const N: usize> Datatype for [i32; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayI32(session, offset, target as *mut i32, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayI32(session, offset, value.as_ptr(), N)
    };
}

impl Datatype for i64 {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadI64(session, offset, target)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteI64(session, offset, value)
    };
}
impl<const N: usize> Datatype for [i64; N] {
    const FFI_READ: ffi::Func<*mut Self> = |session, offset, target| unsafe {
        ffi::ReadArrayI64(session, offset, target as *mut i64, N)
    };
    const FFI_WRITE: ffi::Func<Self> = |session, offset, value| unsafe {
        ffi::WriteArrayI64(session, offset, value.as_ptr(), N)
    };
}
