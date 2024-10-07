use std::sync::Mutex;
use block_header::BlockHeader;

pub mod block_header;

pub mod memorize;
pub mod forget;
pub mod rememorize;


const SBRK_THRESHOLD: usize = 1024 * 128; // 128 KB

static mut HEAD_MEM: *mut BlockHeader = std::ptr::null_mut();
static mut TAIL_MEM: *mut BlockHeader = std::ptr::null_mut();

static mut HEAD_FREE: *mut BlockHeader = std::ptr::null_mut();

static GLOBAL_MEMALLOC_LOCK: Mutex<()> = Mutex::new(());