use super::attribute_info::AttributeInfo;
#[derive(Debug, Clone)]
pub struct AttributePool {
    count: u16,
    info: Vec<AttributeInfo>,
}
impl AttributePool {
    pub fn new(data: &[u8]) -> (AttributePool, usize) {
        let count = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut info = Vec::<AttributeInfo>::with_capacity(count as usize);
        let mut acc = 2;
        for _ in 0..count as usize {
            let (inf, add) = AttributeInfo::new(&data[acc..]);
            acc += add;
            info.push(inf);
        }
        (AttributePool { count, info }, acc)
    }
}
