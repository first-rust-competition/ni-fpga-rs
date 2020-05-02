use crate::ffi;
use crate::Offset;
use crate::Session;
use crate::Status;

type FFIFunc<T> = unsafe extern "C" fn(ffi::Session, ffi::Offset, T) -> ffi::Status;
type FFIArrayFunc<T> = unsafe extern "C" fn(ffi::Session, ffi::Offset, T, usize) -> ffi::Status;

fn ffi_read<T>(
    ffi_func: FFIFunc<*mut T>,
    session: &Session,
    offset: Offset,
) -> Result<T, Status> {
    let mut target = std::mem::MaybeUninit::<T>::uninit();
    let status = Status::from(unsafe {
        ffi_func(
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

fn ffi_read_array<T, const N: usize>(
    ffi_func: FFIArrayFunc<*mut T>,
    session: &Session,
    offset: Offset,
) -> Result<[T; N], Status> {
    let mut target = std::mem::MaybeUninit::<[T; N]>::uninit();
    let status = Status::from(unsafe {
        ffi_func(
            session.handle,
            offset,
            target.as_mut_ptr() as *mut T,
            N,
        )
    });
    match status {
        Status::Success => Ok(unsafe { target.assume_init() }),
        _ => Err(status),
    }
}

fn ffi_write<T>(
    ffi_func: FFIFunc<T>,
    session: &Session,
    offset: Offset,
    value: T,
) -> Result<(), Status> {
    let status = Status::from(unsafe {
        ffi_func(
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

fn ffi_write_array<T, const N: usize>(
    ffi_func: FFIArrayFunc<*const T>,
    session: &Session,
    offset: Offset,
    value: [T; N],
) -> Result<(), Status> {
    let status = Status::from(unsafe {
        ffi_func(
            session.handle,
            offset,
            value.as_ptr(),
            N,
        )
    });
    match status {
        Status::Success => Ok(()),
        _ => Err(status),
    }
}

pub trait Datatype: Copy {
    const FPGA_SIZE: usize;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status>;
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status>;
}

impl Datatype for bool {
    const FPGA_SIZE: usize = 1;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadBool, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteBool, session, offset, value)
    }
}
impl<const N: usize> Datatype for [bool; N] {
    const FPGA_SIZE: usize = N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayBool, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayBool, session, offset, value)
    }
}

impl Datatype for u8 {
    const FPGA_SIZE: usize = 8;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadU8, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteU8, session, offset, value)
    }
}
impl<const N: usize> Datatype for [u8; N] {
    const FPGA_SIZE: usize = 8*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayU8, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayU8, session, offset, value)
    }
}

impl Datatype for u16 {
    const FPGA_SIZE: usize = 16;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadU16, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteU16, session, offset, value)
    }
}
impl<const N: usize> Datatype for [u16; N] {
    const FPGA_SIZE: usize = 16*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayU16, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayU16, session, offset, value)
    }
}

impl Datatype for u32 {
    const FPGA_SIZE: usize = 32;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadU32, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteU32, session, offset, value)
    }
}
impl<const N: usize> Datatype for [u32; N] {
    const FPGA_SIZE: usize = 32*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayU32, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayU32, session, offset, value)
    }
}

impl Datatype for u64 {
    const FPGA_SIZE: usize = 64;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadU64, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteU64, session, offset, value)
    }
}
impl<const N: usize> Datatype for [u64; N] {
    const FPGA_SIZE: usize = 64*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayU64, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayU64, session, offset, value)
    }
}

impl Datatype for i8 {
    const FPGA_SIZE: usize = 8;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadI8, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteI8, session, offset, value)
    }
}
impl<const N: usize> Datatype for [i8; N] {
    const FPGA_SIZE: usize = 8*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayI8, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayI8, session, offset, value)
    }
}

impl Datatype for i16 {
    const FPGA_SIZE: usize = 16;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadI16, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteI16, session, offset, value)
    }
}
impl<const N: usize> Datatype for [i16; N] {
    const FPGA_SIZE: usize = 16*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayI16, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayI16, session, offset, value)
    }
}

impl Datatype for i32 {
    const FPGA_SIZE: usize = 32;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadI32, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteI32, session, offset, value)
    }
}
impl<const N: usize> Datatype for [i32; N] {
    const FPGA_SIZE: usize = 32*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayI32, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayI32, session, offset, value)
    }
}

impl Datatype for i64 {
    const FPGA_SIZE: usize = 64;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read(ffi::ReadI64, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write(ffi::WriteI64, session, offset, value)
    }
}
impl<const N: usize> Datatype for [i64; N] {
    const FPGA_SIZE: usize = 64*N;
    fn read(session: &Session, offset: Offset) -> Result<Self, Status> {
        ffi_read_array(ffi::ReadArrayI64, session, offset)
    }
    fn write(session: &Session, offset: Offset, value: Self) -> Result<(), Status> {
        ffi_write_array(ffi::WriteArrayI64, session, offset, value)
    }
}
