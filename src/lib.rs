#![feature(box_into_inner)]
#![feature(auto_traits)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#[cfg(not(feature = "std"))]
extern crate alloc;
pub mod ClassLoader;
pub mod BaseTypes;
pub mod VirtualMachine;

use crate::ClassLoader::Class::Class;
#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use std::env;
    #[cfg(feature = "std")]
    use std::fs;
    #[cfg(feature = "log")]
    use log::trace;
    use ClassLoader::Class::Class;
    use crate::{ClassLoader, VirtualMachine};

    #[test]
    fn hello_world() -> Result<(), String>{
        #[cfg(feature = "log")]
        simple_logging::log_to_file("hello_world.log", log::LevelFilter::Trace).unwrap();
        let main_class_path = &"jars/HelloMIDlet/HelloMIDlet.class";
        //#[cfg(feature = "log")]
        //trace!("{:}", main_class_path);
        let main_class_data = fs::read(main_class_path).expect("CLASS FILE NOT FOUND");
        //#[cfg(feature = "log")]
        //trace!("{:#X?}", main_class_data);
        let main_class_obj = Class::new(main_class_data);
        #[cfg(feature = "log")]
        trace!("{:#x?}", main_class_obj);
        //let virtual_machine = VirtualMachine::new(main_class_obj);
        Ok(())
    }

}
