extern crate libc;

use std::ptr;

use super::block_header::BlockHeader;
use super::{FREE_BLOCKS, HEAD_FREE, NEIGHBORS, TAIL_MEM};



pub fn link_added_block(block_address: isize) {
    unsafe {
        let mut neighbors = NEIGHBORS.lock().unwrap();
        neighbors.insert(
            block_address,
            (-1, -1));

        if TAIL_MEM != ptr::null_mut() {
            let tail_mem_address: isize = TAIL_MEM as isize;
            let tail_mem_left_neighbor: isize = neighbors
                                                .get(&tail_mem_address)
                                                .unwrap().0;

            neighbors.insert(
                tail_mem_address,
                (tail_mem_left_neighbor, block_address));

            neighbors.insert(
                block_address,
                (tail_mem_address, -1));
        }
        
        let block: *mut BlockHeader = block_address as *mut BlockHeader;
        FREE_BLOCKS.lock().unwrap().insert(
            block_address,
            (false, (*block).size as isize + size_of::<BlockHeader>() as isize));
    }
}


pub fn link_removed_block(block_address: isize) {
    let mut neighbors = NEIGHBORS.lock().unwrap();

    let left_neighbor: isize = neighbors
                                .get(&block_address)
                                .unwrap().0;
    let right_neighbor: isize = neighbors
                                .get(&block_address)
                                .unwrap().1;

    if left_neighbor != -1 {
        let left_left_neighbor: isize = neighbors
                                        .get(&left_neighbor)
                                        .unwrap().0;
        neighbors.insert(
            left_neighbor,
            (left_left_neighbor, right_neighbor));
    }

    if right_neighbor != -1 {
        let right_right_neighbor: isize = neighbors
                                            .get(&right_neighbor)
                                            .unwrap().1;

        neighbors.insert(
            right_neighbor,
            (left_neighbor, right_right_neighbor));
    }

    neighbors.remove(&block_address);
    FREE_BLOCKS.lock().unwrap().remove(&block_address);
}


pub fn split_block(block_address: isize, size: usize) {
    unsafe {
        let block: *mut BlockHeader = block_address as *mut BlockHeader;
        let total_size: usize = (*block).size + size_of::<BlockHeader>();
        let required_size_alligned: usize = (size + size_of::<BlockHeader>() + 7) & !7;
        let smaller_block_address: isize = block_address + required_size_alligned as isize;

        FREE_BLOCKS.lock().unwrap().insert(
            block_address,
            (false, total_size as isize));

        if smaller_block_address + (size_of::<BlockHeader>() as isize) < block_address + total_size as isize {
            let smaller_block: *mut BlockHeader = smaller_block_address as *mut BlockHeader;

            (*smaller_block).size = total_size - required_size_alligned - size_of::<BlockHeader>();
            (*smaller_block).next = (*block).next;
            (*smaller_block).prev = (*block).prev;

            (*block).next = smaller_block;
            (*block).size = required_size_alligned - size_of::<BlockHeader>();

            let mut neighbors = NEIGHBORS.lock().unwrap();

            let left_neighbor: isize = neighbors.get(&block_address).unwrap().0;
            let right_neighbor: isize = neighbors.get(&block_address).unwrap().1;

            let right_right_neighbor: isize = neighbors
                                                .get(&right_neighbor)
                                                .unwrap().1;

            neighbors.insert(
                block_address,
                (left_neighbor, smaller_block_address));

            neighbors.insert(
                smaller_block_address,
                (block_address, right_neighbor));

            neighbors.insert(
                right_neighbor,
                (smaller_block_address, right_right_neighbor));

            let mut free_blocks = FREE_BLOCKS.lock().unwrap();

            free_blocks.insert(
                smaller_block_address,
                (true, (*smaller_block).size as isize + size_of::<BlockHeader>() as isize));

            free_blocks.insert(
                block_address,
                (false, (*block).size as isize + size_of::<BlockHeader>() as isize));
        }
    }
}


pub fn merge_block(mut block_address: isize) -> isize {
    let mut merged: bool = true;
    let mut merged1: bool;
    let mut merged2: bool;

    let block_size = FREE_BLOCKS.lock().unwrap()
                            .get(&block_address)
                            .unwrap().1;

    FREE_BLOCKS.lock().unwrap().insert(
        block_address,
        (true, block_size));

    while merged {
        let left_neighbor = NEIGHBORS.lock().unwrap()
                                    .get(&block_address)
                                    .unwrap().0;

        let right_neighbor = NEIGHBORS.lock()
                                    .unwrap().get(&block_address)
                                    .unwrap().1;

        (merged1, block_address) = try_merge_neighbors(
                                    block_address,
                                    left_neighbor);

        (merged2, block_address) = try_merge_neighbors(
                                    block_address,
                                    right_neighbor);

        merged = merged1 || merged2;
    }

    block_address
}


fn try_merge_neighbors(block_address: isize, neighbor_address: isize) -> (bool, isize) {

    if neighbor_address == -1 {
        return (false, block_address);
    }

    let is_neighbor_free = FREE_BLOCKS.lock().unwrap().
                                    get(&neighbor_address)
                                    .unwrap().0;

    if is_neighbor_free {
        let block: *mut BlockHeader = block_address as *mut BlockHeader;
        let neighbor_block: *mut BlockHeader = neighbor_address as *mut BlockHeader;

        unsafe {
            let block_size: usize = (*block).size + size_of::<BlockHeader>();
            let neighbor_size: usize = (*neighbor_block).size + size_of::<BlockHeader>();

            if block_address < neighbor_address && block_address + block_size as isize == neighbor_address {
                (*block).size = block_size + neighbor_size - size_of::<BlockHeader>();

                if neighbor_block == HEAD_FREE {
                    HEAD_FREE = (*neighbor_block).next;
                }

                if (*neighbor_block).next != ptr::null_mut() {
                    (*(*neighbor_block).next).prev = (*neighbor_block).prev;
                }

                if (*neighbor_block).prev != ptr::null_mut() {
                    (*(*neighbor_block).prev).next = (*neighbor_block).next;
                }

                FREE_BLOCKS.lock().unwrap().insert(
                    block_address,
                    (true, (*block).size as isize + size_of::<BlockHeader>() as isize));

                link_removed_block(neighbor_address);

                return (true, block_address);
            }

            else if block_address > neighbor_address && neighbor_address + neighbor_size as isize == block_address {
                (*neighbor_block).size = block_size + neighbor_size - size_of::<BlockHeader>();
                (*neighbor_block).next = (*block).next;

                FREE_BLOCKS.lock().unwrap().insert(
                    neighbor_address,
                    (true, (*neighbor_block).size as isize + size_of::<BlockHeader>() as isize));

                link_removed_block(block_address);

                return (true, neighbor_address);
            }
        }
    }

    (false, block_address)
}