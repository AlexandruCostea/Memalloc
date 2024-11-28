use std::{collections::HashMap, sync::{LazyLock, Mutex}};
use block_header::BlockHeader;

mod block_header;
mod memory_block_linker;

pub mod memorize;
pub mod forget;
pub mod rememorize;


static mut HEAD_MEM: *mut BlockHeader = std::ptr::null_mut();

static mut TAIL_MEM: *mut BlockHeader = std::ptr::null_mut();

static mut HEAD_FREE: *mut BlockHeader = std::ptr::null_mut();

static GLOBAL_MEMALLOC_LOCK: Mutex<()> = Mutex::new(());

static NEIGHBORS: LazyLock<Mutex<HashMap<isize, (isize, isize)>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static FREE_BLOCKS: LazyLock<Mutex<HashMap<isize, (bool, isize)>>> = LazyLock::new(|| Mutex::new(HashMap::new()));