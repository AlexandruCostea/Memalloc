extern crate libc;

use std::ptr;

use super::{block_header::BlockHeader, SBRK_THRESHOLD, GLOBAL_MEMALLOC_LOCK, HEAD_MEM, TAIL_MEM, HEAD_FREE};


pub fn memorize(size: usize) -> *mut libc::c_void {

    let total_size: usize = size_of::<BlockHeader>() + size;
    let block: *mut libc::c_void;

    if size <= 0 {
        return ptr::null_mut();
    }

    unsafe {        
        let header: *mut BlockHeader = get_free_block(size);
        if header != ptr::null_mut() {
            let block: *mut libc::c_void = header as *mut libc::c_void;
            let post_header_memory: *mut libc::c_void = block.add(size_of::<BlockHeader>());
            return post_header_memory;
        }

        if total_size <= SBRK_THRESHOLD {
            let page_size: usize = libc::sysconf(libc::_SC_PAGESIZE) as usize;

            // make allocation size a multiple of page size
            // page_size is added at the end so we can't get less than size bytes of available memory after
            // alligning returned address

            let total_size_page_alligned: usize = total_size + page_size - total_size % page_size + page_size;
            let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();

            let addr: *mut libc::c_void = libc::sbrk(total_size_page_alligned as isize);

            if addr != ptr::null_mut() {
                let addr: usize = addr as usize;
                let addr: usize = addr + page_size - addr % page_size;
                block = addr as *mut libc::c_void;
            }

            else {
                block = addr;
            }
        }

        else {
            // mmap syscall parameters
            //addr is set to NULL to let the OS pick page-aligned address
            let addr: *mut libc::c_void = ptr::null_mut();
            let prot: i32 = libc::PROT_READ | libc::PROT_WRITE;
            let flags: i32 = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;
            let fd: i32 = -1;
            let offset: i64 = 0;

            block = libc::mmap(addr, total_size, prot, flags, fd, offset);
        }

        match block {
            libc::MAP_FAILED => ptr::null_mut(),
            _ => {
                let header: *mut BlockHeader = block as *mut BlockHeader;
                (*header).size = size;
                (*header).next = ptr::null_mut();

                let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();

                if HEAD_MEM == ptr::null_mut() {
                    HEAD_MEM = header;
                }
                if TAIL_MEM == ptr::null_mut() {
                    TAIL_MEM = header;
                }

                else {
                    (*TAIL_MEM).next = header;
                    TAIL_MEM = header;
                }
                let post_header_memory: *mut libc::c_void = block.add(size_of::<BlockHeader>());

                post_header_memory
            }
        }
        
    }
}


fn get_free_block(size: usize) -> *mut BlockHeader {
    
    let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();
    unsafe {
        let mut block: *mut BlockHeader = HEAD_FREE;

        if block == ptr::null_mut() {
            return ptr::null_mut();
        }

        if (*block).size >= size {
            HEAD_FREE = (*HEAD_FREE).next;
            (*block).next = HEAD_MEM;
            HEAD_MEM = block;
            return block;
        }

        loop {
            let next_block = (*block).next;
            if (*next_block).size >= size {
                (*block).next = (*next_block).next;
                (*next_block).next = HEAD_MEM;
                HEAD_MEM = next_block;
                return next_block;
            }
            if next_block == ptr::null_mut() {
                return ptr::null_mut();
            }
            block = (*block).next;
        }
    }
}