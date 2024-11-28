extern crate libc;

use std::ptr;

use super::block_header::BlockHeader;
use super::memory_block_linker::{link_added_block, split_block};
use super::{GLOBAL_MEMALLOC_LOCK, HEAD_MEM, TAIL_MEM, HEAD_FREE};



pub fn memorize(size: usize) -> *mut libc::c_void {

    let total_size: usize = (size_of::<BlockHeader>() + size + 7) & !7;
    let mut block: *mut libc::c_void;

    if size <= 0 {
        return ptr::null_mut();
    }

    unsafe {        
        block = get_free_block(size);
        if block != ptr::null_mut() {
            let memory_region: *mut libc::c_void = block.add(size_of::<BlockHeader>());
            
            return memory_region;
        }

        block = mmap_alloc(total_size);
        
        match block {
            libc::MAP_FAILED => ptr::null_mut(),
            _ => {
                let header: *mut BlockHeader = block as *mut BlockHeader;
                (*header).size = size;
                (*header).next = ptr::null_mut();
                (*header).prev = ptr::null_mut();

                let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();

                link_added_block(block as isize);
                add_block_to_mem_list(header);

                let memory_region: *mut libc::c_void = block.add(size_of::<BlockHeader>());

                memory_region
            }
        }
        
    }
}



fn mmap_alloc(total_size: usize) -> *mut libc::c_void {
    unsafe {
        // mmap syscall parameters
        //addr is set to NULL to let the OS pick page-aligned address
        let addr: *mut libc::c_void = ptr::null_mut();
        let prot: i32 = libc::PROT_READ | libc::PROT_WRITE;
        let flags: i32 = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;
        let fd: i32 = -1;
        let offset: i64 = 0;

        libc::mmap(addr, total_size, prot, flags, fd, offset)
    }
}


fn add_block_to_mem_list(header: *mut BlockHeader) {
    unsafe {
        if HEAD_MEM == ptr::null_mut() {
            HEAD_MEM = header;
        }

        if TAIL_MEM == ptr::null_mut() {
            TAIL_MEM = header;
        }

        else {
            (*TAIL_MEM).next = header;
            (*header).prev = TAIL_MEM;
            TAIL_MEM = header;
        }
    }
}


fn get_free_block(size: usize) -> *mut libc::c_void {
    
    let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();
    unsafe {
        let mut block: *mut BlockHeader = HEAD_FREE;

        loop {

            if block == ptr::null_mut() {
                return ptr::null_mut();
            }

            if (*block).size >= size {
                if (*block).size > size {
                    split_block(block as isize, size);
                }

                match block as isize == HEAD_FREE as isize {
                    true => {
                        HEAD_FREE = (*HEAD_FREE).next;
                        if HEAD_FREE != ptr::null_mut() {
                            (*HEAD_FREE).prev = ptr::null_mut();
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
                (*block).next = HEAD_MEM;

                if HEAD_MEM != ptr::null_mut() {
                    (*HEAD_MEM).prev = block;
                }
                HEAD_MEM = block;
                
                return block as *mut libc::c_void;
            }

            block = (*block).next;
        }
    }
}