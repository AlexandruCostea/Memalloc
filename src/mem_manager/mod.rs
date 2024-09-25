use block_header::BlockHeader;
use std::sync::Mutex; 

mod block_header;

pub mod memorize;
pub mod forget;
pub mod rememorize;


static mut HEAD: *mut BlockHeader = std::ptr::null_mut();
static mut TAIL: *mut BlockHeader = std::ptr::null_mut();

static GLOBAL_MEMALLOC_LOCK: Mutex<()> = Mutex::new(());