extern crate libc;

use super::{block_header::BlockHeader, GLOBAL_MEMALLOC_LOCK, HEAD, TAIL};

pub fn forget(block: *mut libc::c_void) {

    if block.is_null() {
        return;
    }

    let _get_lock = GLOBAL_MEMALLOC_LOCK.lock();
    unsafe {
        let header: *mut BlockHeader = block.sub(size_of::<BlockHeader>()) as *mut BlockHeader;

        if header == TAIL {
            if HEAD == TAIL {
                HEAD = std::ptr::null_mut();
                TAIL = std::ptr::null_mut();
            } 
            else {
                let mut temp: *mut BlockHeader = HEAD;
                while !temp.is_null() {
                    if (*temp).next == TAIL {
                        (*temp).next = std::ptr::null_mut();
                        TAIL = temp;
                    }
                    temp = (*temp).next;
                }
            }

            let size: usize = (*header).size + size_of::<BlockHeader>();
            let address: *mut libc::c_void = header as *mut libc::c_void;
            libc::munmap(address, size);
            return;
        }
        if valid_header(header) {
            (*header).is_free = true;
        }
        return;
    }
}

fn valid_header(header: *mut BlockHeader) -> bool {
    unsafe {
        let mut temp: *mut BlockHeader = HEAD;
        while !temp.is_null() {
            if temp == header {
                return true;
            }
            temp = (*temp).next;
        }
        false
    }
}