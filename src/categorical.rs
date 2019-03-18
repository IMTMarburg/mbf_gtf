use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use pyo3::prelude::*;
use pyo3::{
    //ToPyObject,
    IntoPyObject};


pub struct Categorical {
    pub values: Vec<u32> ,
    pub cats: HashMap<String, u32>,
}


impl Categorical {
    pub fn new() -> Categorical {
        let xs = Vec::new();
        let hm = HashMap::new();
        Categorical { values: xs, cats: hm}
    }

    pub fn push(&mut self, value: &str) ->() {
        let next = self.cats.len() as u32;
        let no = match self.cats.entry(value.to_string()) {
            Vacant(entry) =>  entry.insert(next),
            Occupied(entry) => entry.into_mut(),
        };
        self.values.push(*no);
        
    }
}
/*
impl ToPyObject for Categorical {
    fn to_object(&self, py: Python) -> PyObject {
        self.values.to_object(py)
    }
}
*/
impl IntoPyObject for Categorical {
    fn into_object(self, py: Python) -> PyObject {
        (
            self.values.into_object(py),
            self.cats.into_object(py)
        ).into_object(py)
    }
}
