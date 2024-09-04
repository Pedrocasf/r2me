#[derive(Debug, Clone)]
pub struct AttributeInfo {
    name_index: u16,
    length: u32,
    info: Vec<u8>,
}
impl AttributeInfo {
    pub fn new(data: &[u8]) -> (AttributeInfo, usize) {
        let name_index = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let length = u32::from_be_bytes(data[2..6].try_into().unwrap());
        let mut info = Vec::with_capacity(length as usize);
        for i in 6..(length - 1) as usize {
            info.push(data[i]);
        }
        (
            AttributeInfo {
                name_index,
                length,
                info,
            },
            (length + 6) as usize,
        )
    }
}
