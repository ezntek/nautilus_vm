use crate::types::*;

pub trait NlStringTrait<S: AsRef<str>>: NlType<S> {}

pub struct NlBool(bool);

impl NlType<bool> for NlBool {
    fn new(val: bool) -> NlBool {
        NlBool(val)
    }

    fn get(&self) -> bool {
        todo!()
    }

    fn set(&mut self, val: bool) {
        todo!()
    }
}

pub struct NlString(String);

impl NlType<String> for NlString {
    fn new(val: String) -> NlString {
        NlString(val)
    }

    fn get(&self) -> String {
        todo!()
    }

    fn set(&mut self, val: String) {
        todo!()
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

