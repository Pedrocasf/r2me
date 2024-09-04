use super::{
    attribute_pool::AttributePool, constant_pool::ConstantPool, field_pool::FieldPool,
    interface_pool::InterfacePool, method_pool::MethodPool,
};
use crate::base_types::java_str_ref::JStrRef;
//use crate::BaseTypes::java_string::JString;JBaseType,
use crate::base_types::{JClassRef, JFieldRef, JInterfaceRef, JMethodRef};
use crate::class_loader::constant_pool::{ConstantPoolID, ConstantPoolType};

#[derive(Debug, Clone)]
pub struct Class {
    magic: u32,
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class_idx: u16,
    super_class_idx: u16,
    interface_pool: InterfacePool,
    field_pool: FieldPool,
    method_pool: MethodPool,
    attribute_pool: AttributePool,
}
pub enum ReturnTypes {
    ClassRef(JClassRef),
    StrRef(JStrRef),
    FieldRef(JFieldRef),
    MethodRef(JMethodRef),
    InterfaceRef(JInterfaceRef),
}
impl Class {
    pub fn new(data: Vec<u8>) -> Class {
        let (constant_pool, cpsize) = ConstantPool::new(&data[8..]);
        let (interface_pool, isize) = InterfacePool::new(&data[14 + cpsize..]);
        let (field_pool, fsize) = FieldPool::new(&data[14 + cpsize + isize..]);
        let (method_pool, msize) = MethodPool::new(&data[14 + cpsize + isize + fsize..]);
        let (attribute_pool, asize) =
            AttributePool::new(&data[14 + cpsize + isize + fsize + msize..]);
        let mut bytecode = data.clone();
        let _ = bytecode.drain(14 + cpsize + isize + fsize + msize + asize..);
        let c = Class {
            magic: u32::from_be_bytes(
                data[0..4]
                    .try_into()
                    .expect("Class must have the magic number"),
            ),
            minor: u16::from_be_bytes(data[4..6].try_into().expect("Class must have minor")),
            major: u16::from_be_bytes(data[6..8].try_into().expect("Class must have major")),
            constant_pool,
            access_flags: u16::from_be_bytes(
                data[8 + cpsize..8 + cpsize + 2]
                    .try_into()
                    .expect("Class must have access flags"),
            ),
            this_class_idx: u16::from_be_bytes(
                data[10 + cpsize..10 + cpsize + 2]
                    .try_into()
                    .expect("Class must have this class index"),
            ),
            super_class_idx: u16::from_be_bytes(
                data[12 + cpsize..12 + cpsize + 2]
                    .try_into()
                    .expect("Class must have super class index"),
            ),
            interface_pool,
            field_pool,
            method_pool,
            attribute_pool,
        };
        if c.magic != 0xCAFEBABE {
            panic!("Invalid Magic in class");
        };
        c
    }
    pub fn solve_ref_idx(&mut self, index: u16) -> ReturnTypes {
        if let Some(element) = self.constant_pool.get_element_of_index(index) {
            return self.cast_refs(element);
        }
        unreachable!()
    }
    fn cast_refs(&self, mut element: ConstantPoolType) -> ReturnTypes {
        match element.id {
            ConstantPoolID::IClassRef => {
                let deref = element
                    .val
                    .as_any()
                    .downcast_mut::<JClassRef>()
                    .unwrap()
                    .clone();
                ReturnTypes::ClassRef(deref)
            }
            ConstantPoolID::IStrRef => {
                let deref = element
                    .val
                    .as_any()
                    .downcast_mut::<JStrRef>()
                    .unwrap()
                    .clone();
                ReturnTypes::StrRef(deref)
            }
            ConstantPoolID::IFieldRef => {
                let deref = element
                    .val
                    .as_any()
                    .downcast_mut::<JFieldRef>()
                    .unwrap()
                    .clone();
                ReturnTypes::FieldRef(deref)
            }
            ConstantPoolID::IMethodRef => {
                let deref = element
                    .val
                    .as_any()
                    .downcast_mut::<JMethodRef>()
                    .unwrap()
                    .clone();
                ReturnTypes::MethodRef(deref)
            }
            ConstantPoolID::IInterfaceRef => {
                let deref = element
                    .val
                    .as_any()
                    .downcast_mut::<JInterfaceRef>()
                    .unwrap()
                    .clone();
                ReturnTypes::InterfaceRef(deref)
            }
            _ => panic!("invalid deref type"),
        }
    }
    pub fn solve_ref_idx_type(&mut self, index: u16, ref_type: ConstantPoolID) -> ReturnTypes {
        //let str_ref:Vec<(u16, Box<dyn JBaseType>)> = self.constantPool.get_elements_of_type(ConstantPoolID::StrRef);
        if let Some(element) = self
            .constant_pool
            .get_element_of_type_and_index(ref_type, index)
        {
            return self.cast_refs(element);
        }
        unreachable!()
    }
}
