use super::AttributePool::AttributePool;
#[derive(Debug, Clone)]
pub struct FieldInfo{
    accessFlags:u16,
    nameIndex:u16,
    descriptorIndex:u16,
    attributes:AttributePool
}
impl FieldInfo{
    pub fn new(data:&[u8])->(FieldInfo,usize){
        let accessFlags = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let nameIndex = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let descriptorIndex = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let (attributes, size) = AttributePool::new(&data[6..]);
        (FieldInfo{
            accessFlags:accessFlags,
            nameIndex:nameIndex,
            descriptorIndex:descriptorIndex,
            attributes:attributes
        },size+6)
    }
}
