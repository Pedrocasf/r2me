use super::JBaseType;
#[derive(Debug, Clone)]
pub struct JStrRef{
    idx:u16,
}
impl JStrRef{
    pub fn new(d:[u8;2])->JStrRef{
        JStrRef { 
            idx:u16::from_be_bytes(d)
        }
    }
}
impl JBaseType for JStrRef{}