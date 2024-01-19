use super::JBaseType;
#[derive(Debug, Clone)]
pub struct JInteger{
    data:u32
}
impl JInteger{
    pub fn new(d:[u8;4])->JInteger{
        JInteger{
            data:u32::from_be_bytes(d)
        }
    }
}
impl JBaseType for JInteger{}