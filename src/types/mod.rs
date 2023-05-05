pub mod number_types;
pub mod general_types;

pub trait NlType<T> {
    fn new(val: T) -> Self where Self: Sized;
    fn get(&self) -> T;
    fn set(&mut self, val: T);    
}
