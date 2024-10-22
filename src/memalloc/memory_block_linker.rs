extern crate libc;

use std::ptr;

use super::block_header::BlockHeader;
use super::{TAIL_MEM, NEIGHBORS};



pub fn link_added_block(block_address: isize) {
    unsafe {
        let mut neighbors = NEIGHBORS.lock().unwrap();
        neighbors.insert(block_address, (-1, -1));

        if TAIL_MEM != ptr::null_mut() {
            let tail_mem_address = TAIL_MEM as isize;
            let tail_mem_left_neighbor = neighbors.get(&tail_mem_address).unwrap().0;

            neighbors.insert(tail_mem_address, (tail_mem_left_neighbor, block_address));
            neighbors.insert(block_address, (tail_mem_address, -1));
        }

        //print the contents of the neighbors hashmap
        // for (key, value) in neighbors.iter() {
        //     println!("Key: {}, Value: {:?}", key, value);
        // }
    }
}


pub fn link_removed_block(block_address: isize) {
    let mut neighbors = NEIGHBORS.lock().unwrap();
    let left_neighbor = neighbors.get(&block_address).unwrap().0;
    let right_neighbor = neighbors.get(&block_address).unwrap().1;

    if left_neighbor != -1 {
        let left_left_neighbor = neighbors.get(&left_neighbor).unwrap().0;
        neighbors.insert(left_neighbor, (left_left_neighbor, right_neighbor));
    }

    if right_neighbor != -1 {
        let right_right_neighbor = neighbors.get(&right_neighbor).unwrap().1;
        neighbors.insert(right_neighbor, (left_neighbor, right_right_neighbor));
    }

    neighbors.remove(&block_address);

    //print the contents of the neighbors hashmap
    // for (key, value) in neighbors.iter() {
    //     println!("Key: {}, Value: {:?}", key, value);
    // }
}


pub fn split_block(block_address: isize, size: usize) {
    // TODO: Implement this function
    unsafe {
        let block = block_address as *mut BlockHeader;
        let total_size: usize = (*block).size + size_of::<BlockHeader>();
    }
}


pub fn merge_block(block_address: isize) -> bool {
    // TODO: Implement this function
    false
}