use rand::prelude::*;
use std::marker::PhantomData;

pub struct Fact<T> {
    _marker: PhantomData<*const T>,
}

impl<T > Fact<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn fact(&self) -> &'static str {
        match self._marker {
            PhantomData::<i32> => {
                let facts = [
                    "i32 is a signed 32-bit integer",
                    "i32 can represent values from -2^31 to 2^31 - 1",
                    "i32 is a common choice for representing whole numbers",
                ];
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..facts.len());
                &facts[index]
            }
            PhantomData::<f64> => {
                let facts = [
                    "f64 is a 64-bit floating-point number",
                    "f64 provides about 15-17 decimal digits of precision",
                    "f64 is the most common choice for representing real numbers",
                ];
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..facts.len());
                &facts[index]
            }
            // ... 
            _ => panic!("Unsupported type"),
        }
    }
}
