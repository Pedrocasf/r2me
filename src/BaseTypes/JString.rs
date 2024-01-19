use super::JBaseType;
#[derive(Debug, Clone)]
pub struct JString{
    size:u16,
    data:Vec<u8>
}
impl JString{
    pub fn new(d:&[u8])->JString{
        let sz = u16::from_be_bytes(d[0..2].try_into().expect("Not enough bytes"));
        JString{
            size:sz,
            data:d[2..2+sz as usize].to_vec()
        }
    }
}
impl JBaseType for JString{}