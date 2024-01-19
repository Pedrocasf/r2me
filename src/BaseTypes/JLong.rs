use super::JBaseType;
#[derive(Debug, Clone)]
pub struct JLong{
    data:u64
}
impl JLong{
    pub fn new(d:[u8;8])->JLong{
        JLong{
            data:u64::from_be_bytes(d)
        }
    }
}
impl JBaseType for JLong{}