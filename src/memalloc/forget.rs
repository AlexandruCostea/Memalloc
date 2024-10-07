extern crate libc;

use std::ptr;

use super::{block_header::BlockHeader, SBRK_THRESHOLD, GLOBAL_MEMALLOC_LOCK, HEAD_MEM, TAIL_MEM, HEAD_FREE};


pub fn forget(block: *mut libc::c_void) {

    if block.is_null() {
        return;
    }

    let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();
    unsafe {
        let header: *mut BlockHeader = block.sub(size_of::<BlockHeader>()) as *mut BlockHeader;

        if header == TAIL_MEM {
            if HEAD_MEM == TAIL_MEM {
                HEAD_MEM = ptr::null_mut();
                TAIL_MEM = ptr::null_mut();
            } 
            else {
                let mut temp: *mut BlockHeader = HEAD_MEM;
                while !temp.is_null() {
                    if (*temp).next == TAIL_MEM {
                        (*temp).next = ptr::null_mut();
                        TAIL_MEM = temp;
                    }
                    temp = (*temp).next;
                }
            }

            let size: usize = (*header).size + size_of::<BlockHeader>();

            if size <= SBRK_THRESHOLD {
                libc::sbrk(-1 * (size as isize));
                return;
            }

            let address: *mut libc::c_void = header as *mut libc::c_void;
            libc::munmap(address, size);
            return;
        }

        set_block_free(header);
        return;
    }
}


fn set_block_free(header: *mut BlockHeader) {
    unsafe {
        let mut block: *mut BlockHeader = HEAD_MEM;

        if block == ptr::null_mut() {
            return;
        }

        if block == header {
            HEAD_MEM = (*HEAD_MEM).next;
            (*block).next = HEAD_FREE;
            HEAD_FREE = block;
            return;
        }

        loop {
            let next_block = (*block).next;
            if next_block == header {
                (*block).next = (*next_block).next;
                (*next_block).next = HEAD_FREE;
                HEAD_FREE = next_block;
                return;
            }
            if next_block == ptr::null_mut() {
                return;
            }
            block = (*block).next;
        }
    }
}