use crate::BaseTypes::JavaString::JString;
use super::{
    ConstantPool::ConstantPool,
    InterfacePool::InterfacePool,
    FieldPool::FieldPool,
    MethodPool::MethodPool,
    AttributePool::AttributePool,
};
use crate::BaseTypes::JavaStrRef::JStrRef;
use crate::BaseTypes::{JBaseType, JClassRef, JFieldRef, JInterfaceRef, JMethodRef};
use crate::ClassLoader::ConstantPool::ConstantPoolID;

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
}
pub enum ReturnTypes{

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
            attributePool:attributePool
        };
        if c.magic != 0xCAFEBABE{
            panic!("Invalid Magic in class");
        };
        return c;
    }
    pub fn solve_ref(&mut self, index:u16, ref_type:ConstantPoolID)-> ReturnTypes{
        use ConstantPoolID::*;
        //let str_ref:Vec<(u16, Box<dyn JBaseType>)> = self.constantPool.get_elements_of_type(ConstantPoolID::StrRef);
        if let Some(mut element) = self.constantPool.get_element_of_type_and_index(ref_type, index){
            match element.id{
                IClassRef => {
                    let deref = element.val.as_any().downcast_mut::<JClassRef>().unwrap().clone();
                }
                IStrRef => {
                    let deref = element.val.as_any().downcast_mut::<JStrRef>().unwrap().clone();
                }
                IFieldRef => {
                    let deref = element.val.as_any().downcast_mut::<JFieldRef>().unwrap().clone();
                }
                IMethodRef => {
                    let deref = element.val.as_any().downcast_mut::<JMethodRef>().unwrap().clone();
                }
                IInterfaceRef => {
                    let deref = element.val.as_any().downcast_mut::<JInterfaceRef>().unwrap().clone();
                }
                _ => None
            };
        }
        println!("Cannot find JStrRef element of index {:}", index);


        let mut boxed_java_str = self.constantPool.get_element_of_type_and_index(ConstantPoolID::IString, str_ref.get_idx())
                .expect(&format!("Cannot find JString element of index {:}", str_ref.get_idx()));
        boxed_java_str.val.as_any().downcast_mut::<JString>().unwrap().clone()
    }
}