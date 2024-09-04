use super::attribute_pool::AttributePool;

#[derive(Debug, Clone)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attribute_pool: AttributePool,
}
impl MethodInfo {
    pub fn new(data: &[u8]) -> (MethodInfo, usize) {
        let access_flags = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let name_index = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let descriptor_index = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let (attribute_pool, size) = AttributePool::new(&data[6..]);
        (
            MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attribute_pool,
            },
            size + 6,
        )
    }
}
