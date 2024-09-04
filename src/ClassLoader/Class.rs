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
use crate::ClassLoader::ConstantPool::{ConstantPoolID, ConstantPoolType};

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
    ClassRef(JClassRef),
    StrRef(JStrRef),
    FieldRef(JFieldRef),
    MethodRef(JMethodRef),
    InterfaceRef(JInterfaceRef)
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
        c
    }
    pub fn solve_ref_idx(&mut self, index:u16) -> ReturnTypes{
        if let Some(element) = self.constantPool.get_element_of_index(index){
            return self.cast_refs(element);
        }
        unreachable!()
    }
    fn cast_refs(&self, mut element:ConstantPoolType)->ReturnTypes {
        match element.id {
            ConstantPoolID::IClassRef => {
                let deref = element.val.as_any().downcast_mut::<JClassRef>().unwrap().clone();
                ReturnTypes::ClassRef(deref)
            }
            ConstantPoolID::IStrRef => {
                let deref = element.val.as_any().downcast_mut::<JStrRef>().unwrap().clone();
                ReturnTypes::StrRef(deref)
            }
            ConstantPoolID::IFieldRef => {
                let deref = element.val.as_any().downcast_mut::<JFieldRef>().unwrap().clone();
                ReturnTypes::FieldRef(deref)
            }
            ConstantPoolID::IMethodRef => {
                let deref = element.val.as_any().downcast_mut::<JMethodRef>().unwrap().clone();
                ReturnTypes::MethodRef(deref)
            }
            ConstantPoolID::IInterfaceRef => {
                let deref = element.val.as_any().downcast_mut::<JInterfaceRef>().unwrap().clone();
                ReturnTypes::InterfaceRef(deref)
            }
            _ => panic!("invalid deref type")
        }
    }
    pub fn solve_ref_idx_type(&mut self, index:u16, ref_type:ConstantPoolID)-> ReturnTypes{
        use ConstantPoolID::*;
        //let str_ref:Vec<(u16, Box<dyn JBaseType>)> = self.constantPool.get_elements_of_type(ConstantPoolID::StrRef);
        if let Some(element) = self.constantPool.get_element_of_type_and_index(ref_type, index){
            return self.cast_refs(element);
        }
        unreachable!()
    }
}