// No padding needed since struct is automatically memory-aligned by compiler
pub struct BlockHeader {
    pub size: usize,
    pub next: *mut BlockHeader
}