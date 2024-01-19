use super::{
    ConstantPool::ConstantPool,
    InterfacePool::InterfacePool,
    FieldPool::FieldPool,
    MethodPool::MethodPool,
    AttributePool::AttributePool,
};
#[derive(Debug, Clone)]
pub struct Class{
    magic:u32,
    minor:u16,
    major:u16,
    constantPool:ConstantPool,
    accessFlags:u16,
    thisClassIdx:u16,
    superClassIdx:u16,
    interfacePool:InterfacePool,
    fieldPool:FieldPool,
    methodPool:MethodPool,
    attributePool:AttributePool,
    bytecode:Vec<u8>
}
impl Class{
    pub fn new(data:Vec<u8>)->Class{
        let (constantPool, cpsize) = ConstantPool::new(&data[8..]);
        let (interfacePool, isize) = InterfacePool::new(&data[14+cpsize..]);
        let (fieldPool, fsize) = FieldPool::new(&data[14+cpsize+isize..]);
        let (methodPool, msize) = MethodPool::new(&data[14+cpsize+isize+fsize..]);
        let (attributePool, asize) = AttributePool::new(&data[14+cpsize+isize+fsize+msize..]);
        let mut bytecode = data.clone();
        let _ = bytecode.drain(14+cpsize+isize+fsize+msize+asize..);
        let c = Class{
            magic:u32::from_be_bytes(data[0..4].try_into().expect("Class must have the magic number")),
            minor:u16::from_be_bytes(data[4..6].try_into().expect("Class must have minor")),
            major:u16::from_be_bytes(data[6..8].try_into().expect("Class must have major")),
            constantPool:constantPool,
            accessFlags:u16::from_be_bytes(data[8+cpsize..8+cpsize+2].try_into().expect("Class must have access flags")),
            thisClassIdx:u16::from_be_bytes(data[10+cpsize..10+cpsize+2].try_into().expect("Class must have this class index")),
            superClassIdx:u16::from_be_bytes(data[12+cpsize..12+cpsize+2].try_into().expect("Class must have super class index")),
            interfacePool:interfacePool,
            fieldPool:fieldPool,
            methodPool:methodPool,
            attributePool:attributePool,
            bytecode:bytecode.clone()
        };
        if c.magic != 0xCAFEBABE{
            panic!("Invalid Magic in class");
        };
        return c;
    }
}