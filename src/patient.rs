use crate::FhirVersion;
use serde_json::Value;

pub struct Patient {
    id: String,
    name: String,
}

impl Patient {
    pub fn new(data: Value, version: FhirVersion) {
        println!("{:?}, {:?}", data, version)
    }

    pub fn build_r4_patient(data: Value) {
        println!("This is r4 patient")
    }
}
