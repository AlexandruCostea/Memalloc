extern crate libc;

use std::ptr;

use super::{block_header::BlockHeader, GLOBAL_MEMALLOC_LOCK, HEAD, TAIL};

pub fn memorize(size: usize) -> *mut libc::c_void {

    let total_size: usize = size_of::<BlockHeader>() + size;
    let block: *mut libc::c_void;

    if size <= 0 {
        return ptr::null_mut();
    }

    let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();

    unsafe {
        let header: *mut BlockHeader = get_free_block(size);
        if header != ptr::null_mut() {
            (*header).is_free = false;
            let block: *mut libc::c_void = header as *mut libc::c_void;
            let post_header_memory: *mut libc::c_void = block.add(size_of::<BlockHeader>());
            return post_header_memory;
        }

        // mmap syscall parameters
        //addr is set to NULL to let the OS pick page-aligned address
        let addr: *mut libc::c_void = ptr::null_mut();
        let prot: i32 = libc::PROT_READ | libc::PROT_WRITE;
        let flags: i32 = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;
        let fd: i32 = -1;
        let offset: i64 = 0;

        block = libc::mmap(addr, total_size, prot, flags, fd, offset);

        match block {
            libc::MAP_FAILED => ptr::null_mut(),
            _ => {
                let header: *mut BlockHeader = block as *mut BlockHeader;
                (*header).size = size;
                (*header).is_free = false;
                (*header).next = ptr::null_mut();
                if HEAD == ptr::null_mut() {
                    HEAD = header;
                }
                if TAIL == ptr::null_mut() {
                    TAIL = header;
                }

                else {
                    (*TAIL).next = header;
                    TAIL = header;
                }
                let post_header_memory: *mut libc::c_void = block.add(size_of::<BlockHeader>());

                post_header_memory
            }
        }
        
    }
}

fn get_free_block(size: usize) -> *mut BlockHeader {
    unsafe {
        let mut block: *mut BlockHeader = HEAD;
        if block == ptr::null_mut() {
            return ptr::null_mut();
        }

        loop {
            if (*block).is_free && (*block).size >= size {
                return block;
            }
            if (*block).next == ptr::null_mut() {
                return ptr::null_mut();
            }
            block = (*block).next;
        }
    }
}