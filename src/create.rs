use std::sync::Arc;
use std::sync::Mutex;

use super::Slot;


pub fn none_slot(func: Box<dyn Fn() -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnNone(Arc::new(Mutex::new(func)))
}

pub fn int_slot(func: Box<dyn Fn(i32) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnInt(Arc::new(Mutex::new(func)))
}

pub fn int_array_slot(func: Box<dyn Fn(Vec<i32>) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnIntArray(Arc::new(Mutex::new(func)))
}

pub fn float_slot(func: Box<dyn Fn(f32) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnFloat(Arc::new(Mutex::new(func)))
}

pub fn float_array_slot(func: Box<dyn Fn(Vec<f32>) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnFloatArray(Arc::new(Mutex::new(func)))
}

pub fn bool_slot(func: Box<dyn Fn(bool) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnBool(Arc::new(Mutex::new(func)))
}

pub fn bool_array_slot(func: Box<dyn Fn(Vec<bool>) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnBoolArray(Arc::new(Mutex::new(func)))
}

pub fn string_slot(func: Box<dyn Fn(String) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnString(Arc::new(Mutex::new(func)))
}

pub fn string_array_slot(func: Box<dyn Fn(Vec<String>) -> Result<(), String> + Send + Sync + 'static>) -> Slot {
    Slot::FnStringArray(Arc::new(Mutex::new(func)))
}