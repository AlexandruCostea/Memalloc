extern crate libc;

use std::ptr;

use super::block_header::BlockHeader;
use super::memory_block_linker::{link_removed_block, merge_block};
use super::{SBRK_THRESHOLD, GLOBAL_MEMALLOC_LOCK, HEAD_MEM, TAIL_MEM, HEAD_FREE, FREE_BLOCKS};


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

            let block_address: isize = header as isize;
            link_removed_block(block_address);

            if size <= SBRK_THRESHOLD {
                libc::sbrk(-1 * (size as isize));
                return;
            }

            let address: *mut libc::c_void = header as *mut libc::c_void;
            libc::munmap(address, size);
            return;
        }

        set_block_free(header);

        //print the contents of the free blocks hashmap
        // for (key, value) in FREE_BLOCKS.lock().unwrap().iter() {
        //     println!("Key: {}, Value: {:?}", key, value);
        // }
        return;
    }
}


fn set_block_free(header: *mut BlockHeader) {
    unsafe {
        let size: isize = ((*header).size + size_of::<BlockHeader>()) as isize;
        let mut block: *mut BlockHeader = HEAD_MEM;

        if block == ptr::null_mut() {
            return;
        }

        if block == header {
            let block_address: isize = block as isize;
            if merge_block(block_address) {
                return;
            }

            HEAD_MEM = (*HEAD_MEM).next;
            (*block).next = HEAD_FREE;
            HEAD_FREE = block;
            FREE_BLOCKS.lock().unwrap().insert(block as isize, (true, size));
            return;
        }

        loop {
            let next_block = (*block).next;
            if next_block == header {
                let next_block_address: isize = next_block as isize;
                if merge_block(next_block_address) {
                    return;
                }
                (*block).next = (*next_block).next;
                (*next_block).next = HEAD_FREE;
                HEAD_FREE = next_block;
                FREE_BLOCKS.lock().unwrap().insert(next_block as isize, (true, size));
                return;
            }
            if next_block == ptr::null_mut() {
                return;
            }
            block = (*block).next;
        }
    }
}