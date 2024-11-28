extern crate libc;

use std::ptr;


use super::block_header::BlockHeader;
use super::memory_block_linker::{link_removed_block, merge_block};
use super::{GLOBAL_MEMALLOC_LOCK, HEAD_MEM, TAIL_MEM, HEAD_FREE};


pub fn forget(block: *mut libc::c_void) {

    if block.is_null() {
        return;
    }

    unsafe {
        let header: *mut BlockHeader = block.sub(size_of::<BlockHeader>()) as *mut BlockHeader;
        let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();
        if header == TAIL_MEM {
            if HEAD_MEM == TAIL_MEM {
                HEAD_MEM = ptr::null_mut();
                TAIL_MEM = ptr::null_mut();
            } 
            else {
                TAIL_MEM = (*TAIL_MEM).prev;
                (*TAIL_MEM).next = ptr::null_mut();
            }

            let size: usize = (*header).size + size_of::<BlockHeader>();

            let block_address: isize = header as isize;
            link_removed_block(block_address);

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

        while block != ptr::null_mut() {
            if block == header {
                let block_address: isize = merge_block(block as isize);
                block = block_address as *mut BlockHeader;

                match block as isize == HEAD_MEM as isize {
                    true => {
                        HEAD_MEM = (*HEAD_MEM).next;
                        if HEAD_MEM != ptr::null_mut() {
                            (*HEAD_MEM).prev = ptr::null_mut();
                        }
                    }
                    false => {
                        (*(*block).prev).next = (*block).next;
                        if (*block).next != ptr::null_mut() {
                            (*(*block).next).prev = (*block).prev;
                        }
                    }
                }
                (*block).prev = ptr::null_mut();
                (*block).next = HEAD_FREE;

                if HEAD_FREE != ptr::null_mut() {
                    (*HEAD_FREE).prev = block;
                }
                HEAD_FREE = block;
                return;
            }

            block = (*block).next;
        }
    }
}