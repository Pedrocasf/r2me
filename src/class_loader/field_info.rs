use super::attribute_pool::AttributePool;
#[derive(Debug, Clone)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: AttributePool,
}
impl FieldInfo {
    pub fn new(data: &[u8]) -> (FieldInfo, usize) {
        let access_flags = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let name_index = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let descriptor_index = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let (attributes, size) = AttributePool::new(&data[6..]);
        (
            FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            },
            size + 6,
        )
    }
}
