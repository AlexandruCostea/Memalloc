extern crate libc;

use std::ptr;

use super::block_header::BlockHeader;
use super::memorize::memorize;
use super::forget::forget;



pub fn rememorize(block: *mut libc::c_void, size: usize) -> *mut libc::c_void {

    if block.is_null() || size <= 0 {
        return ptr::null_mut();
    }

    unsafe {
        let header: *mut BlockHeader = block.sub(size_of::<BlockHeader>()) as *mut BlockHeader;

        if (*header).size >= size {
            return block;
        }

        let new_address: *mut libc::c_void = memorize(size);
        if !new_address.is_null() {
            libc::memcpy(new_address, block as *const libc::c_void, (*header).size);
            forget(block);
        }

        new_address
    }
}