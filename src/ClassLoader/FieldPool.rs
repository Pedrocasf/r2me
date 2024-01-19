use super::FieldInfo::FieldInfo;
#[derive(Debug, Clone)]
pub struct FieldPool{
    size:u16,
    fields:Vec<FieldInfo>
}
impl FieldPool{
    pub fn new(data:&[u8])->(FieldPool,usize){
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        println!("f{:#X?}", size);
        let mut fields = Vec::<FieldInfo>::with_capacity(size as usize);
        let mut acc = 2;
        for _ in 0..size as usize{
            let(info,size)= FieldInfo::new(&data[acc..]);
            acc += size as usize;
            fields.push(info);
        }
        (FieldPool{
            size:size,
            fields:fields
        },acc)
    }
}