#[allow(clippy::type_complexity)]
pub mod create;

use std::sync::{Mutex, Arc};
use std::ops::{Deref, DerefMut};

use lazy_static::lazy_static;
use std::any::Any;
use std::thread::JoinHandle; // 1.4.0

lazy_static! {
    /// A Singleton for general SIGNAL handling.
    pub static ref SON_SIGNAL: Mutex<SignalAndSlotHandler> = Mutex::new(SignalAndSlotHandler::new());
}


pub type Signal = String;

pub type FnNone = Arc<Mutex<dyn Fn() -> Result<(), String> + Send + Sync>>;
pub type FnInt = Arc<Mutex<dyn Fn(i32) -> Result<(), String> + Send + Sync>>;
pub type FnIntArray = Arc<Mutex<dyn Fn(Vec<i32>) -> Result<(), String> + Send + Sync>>;
pub type FnFloat = Arc<Mutex<dyn Fn(f32) -> Result<(), String> + Send + Sync>>;
pub type FnFloatArray = Arc<Mutex<dyn Fn(Vec<f32>) -> Result<(), String> + Send + Sync>>;
pub type FnBool = Arc<Mutex<dyn Fn(bool) -> Result<(), String> + Send + Sync>>;
pub type FnBoolArray = Arc<Mutex<dyn Fn(Vec<bool>) -> Result<(), String> + Send + Sync>>;
pub type FnString = Arc<Mutex<dyn Fn(String) -> Result<(), String> + Send + Sync>>;
pub type FnStringArray = Arc<Mutex<dyn Fn(Vec<String>) -> Result<(), String> + Send + Sync>>;

pub type FnNoneMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnIntMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, i32) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnIntArrayMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, Vec<i32>) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnFloatMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, f32) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnFloatArrayMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, Vec<f32>) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnBoolMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, bool) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnBoolArrayMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, Vec<bool>) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnStringMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, String) -> Result<(), String> + Send + Sync + 'static>>);
pub type FnStringArrayMethod = (Arc<Mutex<dyn Any + Send + Sync>>, Arc<Mutex<dyn Fn(&mut dyn Any, Vec<String>) -> Result<(), String> + Send + Sync + 'static>>);

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

    FnNoneMethod(FnNoneMethod),
    FnIntMethod(FnIntMethod),
    FnIntArrayMethod(FnIntArrayMethod),
    FnFloatMethod(FnFloatMethod),
    FnFloatArrayMethod(FnFloatArrayMethod),
    FnBoolMethod(FnBoolMethod),
    FnBoolArrayMethod(FnBoolArrayMethod),
    FnStringMethod(FnStringMethod),
    FnStringArrayMethod(FnStringArrayMethod),
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

impl Default for SignalAndSlotHandler{
    fn default() -> Self {
        SignalAndSlotHandler {
            connections: vec![],
        }
    }
}

impl SignalAndSlotHandler {
    pub fn new() -> SignalAndSlotHandler {
        SignalAndSlotHandler::default()
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
                    let func_mtx = clone_all_function_none(func_mtx);
                    let handle = create_slot_function_none(func_mtx);
                    handle_vector.push(handle);
                }
                (FnInt(func_mtx), Int(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnIntArray(func_mtx), IntArray(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnFloat(func_mtx), Float(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnFloatArray(func_mtx), FloatArray(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnBool(func_mtx), Bool(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnBoolArray(func_mtx), BoolArray(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnString(func_mtx), String(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnStringArray(func_mtx), StringArray(data)) => {
                    let (func_mtx, data) = clone_all_function_arg(func_mtx, data);
                    let handle = create_slot_function_arg(func_mtx, data);
                    handle_vector.push(handle);
                }
                (FnNoneMethod((obj_mtx, func_mtx)), None) => {
                    let (func, mtx) = clone_all_method_none(func_mtx, obj_mtx);
                    let handle = create_slot_method_none(func, mtx);
                    handle_vector.push(handle);
                }
                (Slot::FnIntMethod((obj_mtx, func_mtx)), Int(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                (Slot::FnIntArrayMethod((obj_mtx, func_mtx)), IntArray(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                (Slot::FnFloatMethod((obj_mtx, func_mtx)), Float(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                (Slot::FnFloatArrayMethod((obj_mtx, func_mtx)), FloatArray(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                (Slot::FnBoolMethod((obj_mtx, func_mtx)), Bool(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                (Slot::FnBoolArrayMethod((obj_mtx, func_mtx)), BoolArray(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                (Slot::FnStringMethod((obj_mtx, func_mtx)), String(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                (Slot::FnStringArrayMethod((obj_mtx, func_mtx)), StringArray(data)) => {
                    let (func_mtx, obj_mtx, data) = clone_all_method_arg(func_mtx, obj_mtx, data);
                    let handle = create_slot_method_arg(func_mtx, obj_mtx, data);
                    handle_vector.push(handle);
                }
                _ => ()
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

//clone functions for slot receiving
#[allow(clippy::type_complexity)]
fn clone_all_function_none(func_mtx: Arc<Mutex<dyn Fn() -> Result<(), String> + Send + Sync>>)
    -> Arc<Mutex<dyn Fn() -> Result<(), String> + Send + Sync>>{
    Arc::clone(&func_mtx)
}

#[allow(clippy::type_complexity)]
fn clone_all_function_arg<D : Clone>(func_mtx: Arc<Mutex<dyn Fn(D) -> Result<(), String> + Send + Sync>>, data : D)
    -> (Arc<Mutex<dyn Fn(D) -> Result<(), String> + Send + Sync>>,
        D){
    (Arc::clone(&func_mtx),data)
}

#[allow(clippy::type_complexity)]
fn clone_all_method_none(func_mtx: Arc<Mutex<dyn Fn(&mut dyn Any) -> Result<(), String> + Send + Sync>>, obj_mtx : Arc<Mutex<dyn Any+Send+Sync>>)
    -> (Arc<Mutex<dyn Fn(&mut dyn Any) -> Result<(), String> + Send + Sync>>,
        Arc<Mutex<dyn Any+Send+Sync>>){
    (Arc::clone(&func_mtx), Arc::clone(&obj_mtx))
}

#[allow(clippy::type_complexity)]
fn clone_all_method_arg<D : Clone>(func_mtx: Arc<Mutex<dyn Fn(&mut dyn Any, D) -> Result<(), String> + Send + Sync>>, obj_mtx : Arc<Mutex<dyn Any+Send+Sync>>, data : D)
    -> (Arc<Mutex<dyn Fn(&mut dyn Any, D) -> Result<(), String> + Send + Sync>>,
        Arc<Mutex<dyn Any+Send+Sync>>,
        D){
    (Arc::clone(&func_mtx), Arc::clone(&obj_mtx), data)
}

//functions for thread spawning

#[allow(clippy::type_complexity)]
fn create_slot_function_none(func_mtx: Arc<Mutex<dyn Fn() -> Result<(), String> + Send + Sync>>) -> JoinHandle<()>{
    std::thread::spawn(move || {
        let func_guard = func_mtx.lock().unwrap();
        let func = func_guard.deref();

        match func() {
            Ok(()) => (),
            Err(e) => eprintln!("ERROR: {}", e),
        }
    })
}

#[allow(clippy::type_complexity)]
fn create_slot_function_arg<D: Send + 'static>(func_mtx: Arc<Mutex<dyn Fn(D) -> Result<(), String> + Send + Sync>>, data : D) -> JoinHandle<()>{
    std::thread::spawn(move || {
        let func_guard = func_mtx.lock().unwrap();
        let func = func_guard.deref();

        match func(data) {
            Ok(()) => (),
            Err(e) => eprintln!("ERROR: {}", e),
        }
    })
}

#[allow(clippy::type_complexity)]
fn create_slot_method_none(func_mtx: Arc<Mutex<dyn Fn(&mut dyn Any) -> Result<(), String> + Send + Sync>>, obj_mtx : Arc<Mutex<dyn Any+Send+Sync>>) -> JoinHandle<()>{
    std::thread::spawn(move || {
        let func_guard = func_mtx.lock().unwrap();
        let func = func_guard.deref();

        let mut obj_guard = obj_mtx.lock().unwrap();
        let obj = obj_guard.deref_mut();

        match func(obj) {
            Ok(()) => (),
            Err(e) => eprintln!("ERROR: {}", e),
        }
    })
}

#[allow(clippy::type_complexity)]
fn create_slot_method_arg<D: Send + 'static>(func_mtx: Arc<Mutex<dyn Fn(&mut dyn Any, D) -> Result<(), String> + Send + Sync>>, obj_mtx : Arc<Mutex<dyn Any+Send+Sync>>, data : D) -> JoinHandle<()>{
    std::thread::spawn(move || {
        let func_guard = func_mtx.lock().unwrap();
        let func = func_guard.deref();

        let mut obj_guard = obj_mtx.lock().unwrap();
        let obj = obj_guard.deref_mut();

        match func(obj, data) {
            Ok(()) => (),
            Err(e) => eprintln!("ERROR: {}", e),
        }
    })
}
