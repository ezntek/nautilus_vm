pub mod number_types;

pub trait NlType<T> {
    fn new(val: T) -> Self;
}

pub trait NlObject<T, U: NlType<T>> {
    fn get(&self) -> T;
    fn set(&mut self, val: T);
}

pub trait NlStringTrait<S: AsRef<str>>: NlType< S> {
    
}

pub struct NlBool(bool);

impl NlType<bool> for NlBool {
    fn new(val: bool) -> NlBool {
        NlBool(val)
    }
}

pub struct NlString(String);

impl NlType<String> for NlString {
    fn new(val: String) -> NlString {
        NlString(val)
    }     
}

impl NlString {
    pub fn capitalize(&self) -> Self {
        NlString(self.0.to_uppercase())
    }
    pub fn lowercase(&self) -> Self {
        NlString(self.0.to_lowercase())
    }
    pub fn is_ascii(&self) -> NlBool {
        NlBool(self.0.is_ascii())
    }
}
