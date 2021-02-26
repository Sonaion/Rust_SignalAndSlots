use std::sync::{Arc, Mutex};
use sigs_slots::*;

#[derive(Debug)]
struct Lol {
    data: i32,
}

impl Lol {
    pub fn new(data: i32) -> Lol {
        Lol {
            data,
        }
    }

    pub fn print(&mut self) -> Result<(), String> {
        println!("{:?}", self);
        Ok(())
    }

    pub fn print_add(&mut self, val: i32) -> Result<(), String> {
        self.data += val;
        println!("{:?}", self);
        Ok(())
    }
}

fn print() -> Result<(), String> {
    println!("Hello World");
    Ok(())
}

fn print_num(num: i32) -> Result<(), String> {
    println!("Hello {}", num);
    Ok(())
}


fn main() {
    let mut handler = SON_SIGNAL.lock().unwrap();

    //usage of functions
    let print_slot = create::none_slot(Box::new(print));
    let print_slot_num = create::int_slot(Box::new(print_num));

    handler.connect(String::from("print"), print_slot);
    //clone for use in multiple signals
    handler.connect(String::from("print"), print_slot_num.clone());
    handler.connect(String::from("print_num"), print_slot_num);

    //note that only print_slot will be executed, because of the fitting argument
    handler.emit(String::from("print"), SlotArgs::None);

    //here print_num executes right the print_slot_num
    handler.emit(String::from("print"), SlotArgs::Int(32));
    handler.emit(String::from("print_num"), SlotArgs::Int(32));


    //usage of methods

    let lol1 = Arc::new(Mutex::new(Lol::new(42)));
    let print_method_slot = create::none_method_slot(Box::new(Lol::print), Arc::clone(&lol1));
    let print_method_add_slot = create::int_method_slot(Box::new(Lol::print_add), Arc::clone(&lol1));

    handler.connect(String::from("print_method"), print_method_slot);
    handler.connect(String::from("print_method"), print_method_add_slot.clone());
    handler.connect(String::from("print_add_method"), print_method_add_slot);

    handler.emit(String::from("print_method"), SlotArgs::None);
    handler.emit(String::from("print_method"), SlotArgs::Int(42));
    handler.emit(String::from("print_add_method"), SlotArgs::Int(42));

}
