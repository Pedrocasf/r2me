use super::JBaseType;
#[derive(Debug, Clone)]
pub struct JClassRef{
    idx:u16,
}
impl JClassRef{
    pub fn new(d:[u8;2])->JClassRef{
        JClassRef { 
            idx:u16::from_be_bytes(d)
        }
    }
}
impl JBaseType for JClassRef{}