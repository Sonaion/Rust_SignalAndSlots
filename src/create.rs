use std::sync::Arc;
use std::sync::Mutex;

use super::Slot;
use std::any::Any;

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

pub fn none_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnNoneMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn int_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, i32) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnIntMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: i32| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn int_array_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, Vec<i32>) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnIntArrayMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: Vec<i32>| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn float_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, f32) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnFloatMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: f32| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn float_array_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, Vec<f32>) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnFloatArrayMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: Vec<f32>| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn bool_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, bool) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnBoolMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: bool| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn bool_array_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, Vec<bool>) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnBoolArrayMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: Vec<bool>| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn string_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, String) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnStringMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: String| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}

pub fn string_array_method_slot<T: Any + Send + Sync>(func: Box<dyn Fn(&mut T, Vec<String>) -> Result<(), String> + Send + Sync + 'static>, obj: Arc<Mutex<T>>) -> Slot {
    Slot::FnStringArrayMethod((obj, Arc::new(Mutex::new(move |arg: &mut dyn Any, data: Vec<String>| {
        return match arg.downcast_mut::<T>() {
            Some(value) => func(value, data),
            None => Err(String::from("Couldn't cast for Method use")),
        };
    }))))
}
