extern crate libc;

use std::{ptr, io};

pub fn memalloc(len: usize) -> Result<*mut libc::c_void, io::Error> {
    unsafe {
        // mmap syscall parameters


        //addr is set to NULL to let the OS pick page-aligned address
        let addr: *mut libc::c_void = ptr::null_mut();
        let prot: i32 = libc::PROT_READ | libc::PROT_WRITE;
        let flags: i32 = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;
        let fd: i32 = -1;
        let offset: i64 = 0;

        let block: *mut libc::c_void;
        block = libc::mmap(addr, len, prot, flags, fd, offset);

        match block {
            libc::MAP_FAILED => Err(io::Error::last_os_error()),
            _ => Ok(block)
        }

    }
}