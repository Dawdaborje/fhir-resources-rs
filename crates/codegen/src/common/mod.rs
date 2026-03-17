//! Common utilities shared across FHIR version generators
//!
//! This module contains reusable components for code generation that are
//! version-agnostic, including type mapping, code generation, and file writing.

pub mod codegen;
pub mod type_mapper;
pub mod writer;
