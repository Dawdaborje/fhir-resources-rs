mod account;
mod patient;

use patient::Patient;
use serde_json::Value;
use std::io::Error;
// in-crate result type
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum FhirVersion {
    R4,
    R4B,
    R5,
    R6,
}

#[derive(Debug)]
pub enum FhirResource {
    PatientResource,
}

struct Resource {}

pub fn create_resource(resource: FhirResource, version: FhirVersion, data: Value) {
    match resource {
        FhirResource::PatientResource => Patient::new(data),
    }
}
