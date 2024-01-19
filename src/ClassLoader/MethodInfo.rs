use super::AttributePool::AttributePool;

#[derive(Debug, Clone)]
pub struct MethodInfo{
    accessFlags:u16,
    nameIndex:u16,
    descriptorIndex:u16,
    AttributePool:AttributePool
}
impl MethodInfo{
    pub fn new(data:&[u8])->(MethodInfo,usize){
        let accessFlags = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let nameIndex = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let descriptorIndex = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let (AttributePool, size) = AttributePool::new(&data[6..]);
        (MethodInfo{
            accessFlags:accessFlags,
            nameIndex:nameIndex,
            descriptorIndex:descriptorIndex,
            AttributePool:AttributePool
        },size+6)
    }
}