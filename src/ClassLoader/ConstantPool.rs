use crate::BaseTypes::{
    JString::JString,
    JInteger::JInteger,
    JFloat::JFloat,
    JLong::JLong,
    JDouble::JDouble,
    JClassRef::JClassRef,
    JStrRef::JStrRef,
    JFieldRef::JFieldRef,
    JMethodRef::JMethodRef,
    JInterfaceRef::JInterfaceRef,
    JNameTypeDescriptor::JNameTypeDescriptor,
    JBaseType
};
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum ConstantPoolID{
    String=1,
    Integer=3,
    Float=4,
    Long=5,
    Double=6,
    ClassRef=7,
    StrRef=8,
    FieldRef=9,
    MethodRef=10,
    InterfaceRef=11,
    NameTypeDescriptor=12,
}
impl TryFrom<u8> for ConstantPoolID{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use ConstantPoolID::*;
        match value {
            1 => Ok(String),
            3 => Ok(Integer),
            4 => Ok(Float),
            5 => Ok(Long),
            6 => Ok(Double),
            7 => Ok(ClassRef),
            8 => Ok(StrRef),
            9 => Ok(FieldRef),
            10 => Ok(MethodRef),
            11 => Ok(InterfaceRef),
            12 => Ok(NameTypeDescriptor),
            w => Err(format!("Invalid ConstatndPoolId {:#X}",w)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ConstantPoolType{
    id:ConstantPoolID,
    val:Box<dyn JBaseType>
}
impl ConstantPoolType{
    pub fn new(v:&[u8])->(ConstantPoolType,u16,bool){
        use  ConstantPoolID::*;
        match v[0].try_into().expect(&format!("Invalid ConstatndPoolId {:#X?}",v[0])){
            String             => (ConstantPoolType{ id: String             ,val: Box::new(JString            ::new(&v[1..])) },u16::from_be_bytes(v[1..3].try_into().unwrap())+3, false),
            Integer            => (ConstantPoolType{ id: Integer            ,val: Box::new(JInteger           ::new(v[1..5].try_into().unwrap()))},5,false),
            Float              => (ConstantPoolType{ id: Float              ,val: Box::new(JFloat             ::new(v[1..5].try_into().unwrap()))},5,false),
            Long               => (ConstantPoolType{ id: Long               ,val: Box::new(JLong              ::new(v[1..9].try_into().unwrap()))},9,true),
            Double             => (ConstantPoolType{ id: Double             ,val: Box::new(JDouble            ::new(v[1..9].try_into().unwrap()))},9,true),
            ClassRef           => (ConstantPoolType{ id: ClassRef           ,val: Box::new(JClassRef          ::new(v[1..3].try_into().unwrap()))},3,false),
            StrRef             => (ConstantPoolType{ id: StrRef             ,val: Box::new(JStrRef            ::new(v[1..3].try_into().unwrap()))},3,false),
            FieldRef           => (ConstantPoolType{ id: FieldRef           ,val: Box::new(JFieldRef          ::new(v[1..5].try_into().unwrap()))},5,false),
            MethodRef          => (ConstantPoolType{ id: MethodRef          ,val: Box::new(JMethodRef         ::new(v[1..5].try_into().unwrap()))},5,false),
            InterfaceRef       => (ConstantPoolType{ id: InterfaceRef       ,val: Box::new(JInterfaceRef      ::new(v[1..5].try_into().unwrap()))},5,false),
            NameTypeDescriptor => (ConstantPoolType{ id: NameTypeDescriptor ,val: Box::new(JNameTypeDescriptor::new(v[1..5].try_into().unwrap()))},5,false),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ConstantPool{
    size:u16,
    pool:Vec<ConstantPoolType>
}
impl ConstantPool{
    pub fn new(data:&[u8])->(ConstantPool,usize){
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        println!("{:#X?}",size);
        let mut i = 1;
        let mut ptr_idx = 2; 
        let mut pool = Vec::new();
        println!("{:#X?}", ptr_idx);
        while i < size{
            let (constantPoolType, typesize ,slotCount) = ConstantPoolType::new(&data[ptr_idx..]);
            i += 1;
            ptr_idx += typesize as usize;
            pool.push(constantPoolType.clone());
            if slotCount{
                pool.push(constantPoolType);
                i += 1;
            }
        }
        (ConstantPool {
            size: size,
            pool:pool
        }
        ,ptr_idx)
    }
}