use super::JBaseType;
#[derive(Debug, Clone)]
pub struct JMethodRef{
    classRefIdx:u16,
    nameTypeDescriptorIdx:u16
}
impl JMethodRef{
    pub fn new(d:[u8;4])->JMethodRef{
        JMethodRef { 
            classRefIdx: u16::from_be_bytes(d[0..2].try_into().unwrap()),
            nameTypeDescriptorIdx: u16::from_be_bytes(d[2..4].try_into().unwrap())
        }
    }
}
impl JBaseType for JMethodRef{}