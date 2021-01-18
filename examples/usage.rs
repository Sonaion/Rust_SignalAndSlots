use std::sync::{Arc, Mutex};
use rust_signal_and_slots::*;

fn hello() -> Result<(), String>{
    println!("was geht abbbb");
    Ok(())
}

fn print_num(num : i32) -> Result<(), String>{
    println!("this is the num {}", num);
    Ok(())
}

fn main() {
    let mut handler = SON_SIGNAL.lock().unwrap();
    let print_hello : FnNone = Arc::new(Mutex::new( ||{
        println!("Hello World");
        Ok(())
    }));

    let fun = create_none_slot(Box::new(hello));

    let fun_num  = create_int_slot(Box::new(print_num));

    handler.connect(String::from("print"), Slot::FnNone(Arc::clone(&print_hello)));
    handler.connect(String::from("print"), Slot::FnNone(Arc::clone(&fun)));
    handler.connect(String::from("num"), Slot::FnInt(Arc::clone(&fun_num)));

    handler.emit(String::from("num"), SlotArgs::Int(23));
    handler.emit(String::from("print"), SlotArgs::None);
    handler.emit(String::from("num"), SlotArgs::Int(23));
    handler.emit(String::from("print"), SlotArgs::None);

}
