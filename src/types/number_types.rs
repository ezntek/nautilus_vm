use crate::types::*;
use num::{Integer,Float,Num};

pub enum NlInt { 
    I8(NlInteger<i8>),
    I16(NlInteger<i16>),
    I32(NlInteger<i32>),
    I64(NlInteger<i64>),
}

pub trait NlNumberTrait<T: Num>: NlType<T> {
    fn add(&self, b: T) -> Self;
    fn sub(&self, b: T) -> Self;
    fn mul(&self, b: T) -> Self;
    fn div(&self, b: T) -> Self;
}

pub struct NlInteger<I: Integer>(I);

impl NlType<i32> for NlInteger<i32> {
    fn new(val: i32) -> Self {
        NlInteger(val)
    }

    fn get(&self) -> i32 {
        todo!()
    }

    fn set(&mut self, val: i32) {
        todo!()
    }
}

impl Into<NlInteger<i32>> for i32 {
    fn into(self) -> NlInteger<i32> {
        NlInteger(self)
    }
}

impl NlNumberTrait<i32> for NlInteger<i32> {
    fn add(&self, b: i32) -> NlInteger<i32> {
        NlInteger(&self.0 + b)     
    }

    fn sub(&self, b: i32) -> NlInteger<i32> {
        NlInteger(&self.0 - b)     
    }

    fn mul(&self, b: i32) -> NlInteger<i32> {
        NlInteger(&self.0 * b)
    }

    fn div(&self, b: i32) -> NlInteger<i32> {
        NlInteger(self.0.div_floor(&b))
    }
}

pub struct NlFloat<F: Float>(F);

impl NlType<f64> for NlFloat<f64> {
    fn new(val: f64) -> Self {
        NlFloat(val)
    }

    fn get(&self) -> f64 {
        todo!()
    }

    fn set(&mut self, val: f64) {
        todo!()
    }
}

impl Into<NlFloat<f64>> for f64 {
    fn into(self) -> NlFloat<f64> {
        NlFloat(self)
    }
}

impl NlNumberTrait<f64> for NlFloat<f64> {
    fn add(&self, b: f64) -> Self {
        NlFloat(&self.0 + b)
    }
    fn sub(&self, b: f64) -> Self {
        NlFloat(&self.0 - b)
    }
    fn mul(&self, b: f64) -> Self {
        NlFloat(&self.0 * b)
    }
    fn div(&self, b: f64) -> Self {
        NlFloat(&self.0 / b)
    }
}