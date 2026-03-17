//! FHIR R4 TerminologyCapabilities Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Software that is covered by this terminology capability statement
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesSoftware {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A name the software is known by
    pub name: String,

    /// Version covered by this statement
    pub version: Option<String>,
}

/// If this describes a specific instance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesImplementation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Describes this specific instance
    pub description: String,

    /// Base URL for the implementation
    pub url: Option<String>,
}

/// A code system supported by the server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// URI for the Code System
    pub uri: Option<String>,

    /// Version of Code System supported
    pub version: Option<Vec<TerminologyCapabilitiesCodeSystemVersion>>,

    /// Whether subsumption is supported
    pub subsumption: Option<bool>,
}

/// Version of Code System supported
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystemVersion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Version identifier for this version
    pub code: Option<String>,

    /// If this is the default version for this code system
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,

    /// If compositional grammar is supported
    pub compositional: Option<bool>,

    /// Language Displays supported
    pub language: Option<Vec<String>>,

    /// Filter Properties supported
    pub filter: Option<Vec<TerminologyCapabilitiesCodeSystemVersionFilter>>,

    /// Properties supported for $lookup
    pub property: Option<Vec<String>>,
}

/// Filter Properties supported
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystemVersionFilter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code of the property supported
    pub code: String,

    /// Operations supported for the property
    pub op: Vec<String>,
}

/// Information about the [ValueSet/$expand](valueset-operation-expand.html) operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesExpansion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether the server can return nested value sets
    pub hierarchical: Option<bool>,

    /// Whether the server supports paging on expansion
    pub paging: Option<bool>,

    /// Allow request for incomplete expansions?
    pub incomplete: Option<bool>,

    /// Supported expansion parameter
    pub parameter: Option<Vec<TerminologyCapabilitiesExpansionParameter>>,

    /// Documentation about text searching works
    #[serde(rename = "textFilter")]
    pub text_filter: Option<String>,
}

/// Supported expansion parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesExpansionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Expansion Parameter name
    pub name: String,

    /// Description of support for parameter
    pub documentation: Option<String>,
}

/// Information about the [ValueSet/$validate-code](valueset-operation-validate-code.html) operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesValidateCode {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether translations are validated
    pub translations: bool,
}

/// Information about the [ConceptMap/$translate](conceptmap-operation-translate.html) operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesTranslation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether the client must identify the map
    #[serde(rename = "needsMap")]
    pub needs_map: bool,
}

/// Information about the [ConceptMap/$closure](conceptmap-operation-closure.html) operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesClosure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// If cross-system closure is supported
    pub translation: Option<bool>,
}

/// A TerminologyCapabilities resource documents a set of capabilities (behaviors) of a FHIR Terminology Server that may be used as a statement of actual server functionality or a statement of required...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilities {
    /// The type of resource
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Logical id of this artifact
    pub id: Option<String>,

    /// Metadata about the resource
    pub meta: Option<Meta>,

    /// A set of rules under which this content was created
    #[serde(rename = "implicitRules")]
    pub implicit_rules: Option<String>,

    /// Language of the resource content
    pub language: Option<String>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Canonical identifier for this terminology capabilities, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Business version of the terminology capabilities
    pub version: Option<String>,

    /// Name for this terminology capabilities (computer friendly)
    pub name: Option<String>,

    /// Name for this terminology capabilities (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: String,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the terminology capabilities
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for terminology capabilities (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this terminology capabilities is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// instance | capability | requirements
    pub kind: String,

    /// Software that is covered by this terminology capability statement
    pub software: Option<TerminologyCapabilitiesSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<TerminologyCapabilitiesImplementation>,

    /// Whether lockedDate is supported
    #[serde(rename = "lockedDate")]
    pub locked_date: Option<bool>,

    /// A code system supported by the server
    #[serde(rename = "codeSystem")]
    pub code_system: Option<Vec<TerminologyCapabilitiesCodeSystem>>,

    /// Information about the [ValueSet/$expand](valueset-operation-expand.html) operation
    pub expansion: Option<TerminologyCapabilitiesExpansion>,

    /// explicit | all
    #[serde(rename = "codeSearch")]
    pub code_search: Option<String>,

    /// Information about the [ValueSet/$validate-code](valueset-operation-validate-code.html) operation
    #[serde(rename = "validateCode")]
    pub validate_code: Option<TerminologyCapabilitiesValidateCode>,

    /// Information about the [ConceptMap/$translate](conceptmap-operation-translate.html) operation
    pub translation: Option<TerminologyCapabilitiesTranslation>,

    /// Information about the [ConceptMap/$closure](conceptmap-operation-closure.html) operation
    pub closure: Option<TerminologyCapabilitiesClosure>,
}
