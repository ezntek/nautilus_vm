use crate::types::*;
use num::{Integer,Float,Num};

pub trait NlNumber<T: Num>: NlType<T> {
    fn add(&self, b: T) -> Self;
    fn sub(&self, b: T) -> Self;
    fn mul(&self, b: T) -> Self;
    fn div(&self, b: T) -> Self;
}

pub struct NlInteger<I: Integer>(I);

impl NlType<i8> for NlInteger<i8> {
    fn new(val: i8) -> Self {
        return NlInteger(val)
    }
}

impl Into<NlInteger<i8>> for i8 {
    fn into(self) -> NlInteger<i8> {
        NlInteger(self)
    }
}

impl NlNumber<i8> for NlInteger<i8> {
    fn add(&self, b: i8) -> NlInteger<i8> {
        if &self.0 + b > std::i8::MAX {
            NlInteger::<i16>(&self.0 + b)
        }
    }

    fn sub(&self, b: i8) -> NlInteger<i8> {
        NlInteger(())
    }

    fn mul(&self, b: i8) -> NlInteger<i8> {
        todo!()
    }

    fn div(&self, b: i8) -> NlInteger<i8> {
        todo!()
    }
} 

pub struct NlFloat<F: Float>(F);


