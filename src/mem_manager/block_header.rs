
// No padding needed since struct is automatically memory-aligned by compiler
pub struct BlockHeader {
    pub size: usize,
    pub is_free: bool,
    pub next: *mut BlockHeader
}