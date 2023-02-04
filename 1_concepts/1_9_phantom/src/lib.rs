use rand::prelude::*;
use std::marker::PhantomData;

pub struct Fact<T> {
    _marker: PhantomData<T>,
}

impl<T> Fact<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

}

impl Fact<i32> {
    pub fn fact(&self) -> Option<&str>{
        let facts = [
            "i32 is a signed 32-bit integer",
            "i32 can represent values from -2^31 to 2^31 - 1",
            "i32 is a common choice for representing whole numbers",
        ];
        let mut rng = rand::thread_rng();
        facts.choose(&mut rng).copied()
    }
}

impl Fact<f64> {
    pub fn fact(&self) -> Option<&str> {
        let facts = [
                    "f64 is a 64-bit floating-point number",
                    "f64 provides about 15-17 decimal digits of precision",
                    "f64 is the most common choice for representing real numbers",
                ];
        let mut rng = rand::thread_rng();
        facts.choose(&mut rng).copied()
    }
}

impl Fact<&'static str> {
    pub fn fact(&self) -> Option<&str>{
        let facts = [
                    "&str is a string slice type",
                    "&str is immutable",
                    "&str is often used for text data",
                ];
        let mut rng = rand::thread_rng();
        facts.choose(&mut rng).copied()
    }
}

impl<T> Fact<Vec<T>> {
    pub fn fact(&self) -> Option<&str> {
        let facts = [
                    "Vec<T> is a dynamic array type",
                    "Vec<T> grows as elements are added",
                    "Vec<T> is often used for collections of values",
                ];
        let mut rng = rand::thread_rng();
        facts.choose(&mut rng).copied()
    }
}

impl Fact<u32> {
    pub fn fact(&self) -> Option<&str> {
        let facts = [
                    "u32 is an unsigned 32-bit integer",
                    "u32 can represent values from 0 to 2^32 - 1",
                    "u32 is often used for representing non-negative whole numbers",
                ];
        let mut rng = rand::thread_rng();
        facts.choose(&mut rng).copied()
    }
}