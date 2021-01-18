pub mod create;

use std::sync::{Mutex, Arc};
use std::ops::Deref;

use lazy_static::lazy_static; // 1.4.0

lazy_static! {
    /// A Singleton for general SIGNAL handling.
    pub static ref SON_SIGNAL: Mutex<SignalAndSlotHandler> = Mutex::new(SignalAndSlotHandler::new());
}


pub type Signal = String;

pub type FnNone = Arc<Mutex<dyn Fn() -> Result<(), String> + Send + Sync + 'static>>;
pub type FnInt = Arc<Mutex<dyn Fn(i32) -> Result<(), String> + Send + Sync + 'static>>;
pub type FnIntArray = Arc<Mutex<dyn Fn(Vec<i32>) -> Result<(), String> + Send + Sync + 'static>>;
pub type FnFloat = Arc<Mutex<dyn Fn(f32) -> Result<(), String> + Send + Sync + 'static>>;
pub type FnFloatArray = Arc<Mutex<dyn Fn(Vec<f32>) -> Result<(), String> + Send + Sync + 'static>>;
pub type FnBool = Arc<Mutex<dyn Fn(bool) -> Result<(), String> + Send + Sync + 'static>>;
pub type FnBoolArray = Arc<Mutex<dyn Fn(Vec<bool>) -> Result<(), String> + Send + Sync + 'static>>;
pub type FnString = Arc<Mutex<dyn Fn(String) -> Result<(), String> + Send + Sync + 'static>>;
pub type FnStringArray = Arc<Mutex<dyn Fn(Vec<String>) -> Result<(), String> + Send + Sync + 'static>>;

#[derive(Clone)]
pub enum Slot {
    FnNone(FnNone),
    FnInt(FnInt),
    FnIntArray(FnIntArray),
    FnFloat(FnFloat),
    FnFloatArray(FnFloatArray),
    FnBool(FnBool),
    FnBoolArray(FnBoolArray),
    FnString(FnString),
    FnStringArray(FnStringArray),
}


#[derive(Clone)]
pub enum SlotArgs {
    None,
    Int(i32),
    IntArray(Vec<i32>),
    Float(f32),
    FloatArray(Vec<f32>),
    Bool(bool),
    BoolArray(Vec<bool>),
    String(String),
    StringArray(Vec<String>),
}

pub struct SignalAndSlotHandler {
    connections: Vec<(Signal, Slot)>,
}

impl SignalAndSlotHandler {
    pub fn new() -> SignalAndSlotHandler {
        SignalAndSlotHandler {
            connections: vec![],
        }
    }


    pub fn connect(&mut self, signal: Signal, slot: Slot) {
        self.connections.push((signal, slot));
    }

    pub fn emit(&self, signal: Signal, slot_args: SlotArgs) {
        let slots = self.connections
            .iter()
            .filter(|(sig, _slot)| *sig == signal)
            .map(|(_sig, slot)| slot.clone())
            .collect::<Vec<Slot>>();

        let mut handle_vector = vec![];
        for slot in slots {
            let slot_arg_temp = slot_args.clone();
            use self::Slot::*;
            use self::SlotArgs::*;
            match (slot, slot_arg_temp) {
                (FnNone(func_mtx), None) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func() {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnInt(func_mtx), Int(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnIntArray(func_mtx), IntArray(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnFloat(func_mtx), Float(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnFloatArray(func_mtx), FloatArray(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnBool(func_mtx), Bool(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnBoolArray(func_mtx), BoolArray(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnString(func_mtx), String(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                (FnStringArray(func_mtx), StringArray(data)) => {
                    let func_mtx = Arc::clone(&func_mtx);
                    let data = data.clone();
                    let handle = std::thread::spawn(move || {
                        let func_guard = func_mtx.lock().unwrap();
                        let func = func_guard.deref();
                        match func(data) {
                            Ok(()) => (),
                            Err(e) => eprintln!("ERROR: {}", e),
                        }
                    });
                    handle_vector.push(handle);
                }
                _ => (),
            }
        }
        for handle in handle_vector {
            match handle.join() {
                Ok(()) => (),
                Err(e) => eprintln!("ERROR: Couldn't join on handle: {:?}", e),
            }
        }
    }
}
