
// No padding needed since struct is automatically memory-aligned by compiler
pub struct BlockHeader {
    size: usize,
    is_free: bool,
    next: Box<Option<BlockHeader>>,
}


impl BlockHeader {
    pub fn new(size: usize, is_free: bool) -> BlockHeader {
        let next_block: Box<Option<BlockHeader>> = Box::new(None);
        BlockHeader{size, is_free, next: next_block}
    }
}