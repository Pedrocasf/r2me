use super::JBaseType;
#[derive(Debug, Clone)]
pub struct JFieldRef{
    classRefIdx:u16,
    nameTypeDescriptorIdx:u16
}
impl JFieldRef{
    pub fn new(d:[u8;4])->JFieldRef{
        JFieldRef { 
            classRefIdx: u16::from_be_bytes(d[0..2].try_into().unwrap()),
            nameTypeDescriptorIdx: u16::from_be_bytes(d[2..4].try_into().unwrap())
        }
    }
}
impl JBaseType for JFieldRef{}