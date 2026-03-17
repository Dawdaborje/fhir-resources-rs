//! FHIR R5 CapabilityStatement Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Software that is covered by this capability statement
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementSoftware {
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

    /// Date this version was released
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
}

/// If this describes a specific instance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementImplementation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Describes this specific instance
    pub description: String,

    /// Base URL for the installation
    pub url: Option<String>,

    /// Organization that manages the data
    pub custodian: Option<Box<Reference>>,
}

/// If the endpoint is a RESTful one
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRest {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// client | server
    pub mode: String,

    /// General description of implementation
    pub documentation: Option<String>,

    /// Information about security of implementation
    pub security: Option<CapabilityStatementRestSecurity>,

    /// Resource served on the REST interface
    pub resource: Option<Vec<CapabilityStatementRestResource>>,

    /// What operations are supported?
    pub interaction: Option<Vec<CapabilityStatementRestInteraction>>,

    /// Search parameters for searching all resources
    #[serde(rename = "searchParam")]
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>>,

    /// Definition of a system level operation
    pub operation: Option<Vec<CapabilityStatementRestResourceOperation>>,

    /// Compartments served/used by system
    pub compartment: Option<Vec<String>>,
}

/// Information about security of implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestSecurity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Adds CORS Headers (http://enable-cors.org/)
    pub cors: Option<bool>,

    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    pub service: Option<Vec<CodeableConcept>>,

    /// General description of how security works
    pub description: Option<String>,
}

/// Resource served on the REST interface
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A resource type that is supported
    #[serde(rename = "type")]
    pub r#type: String,

    /// System-wide profile
    pub profile: Option<String>,

    /// Use-case specific profiles
    #[serde(rename = "supportedProfile")]
    pub supported_profile: Option<Vec<String>>,

    /// Additional information about the use of the resource type
    pub documentation: Option<String>,

    /// What operations are supported?
    pub interaction: Option<Vec<CapabilityStatementRestResourceInteraction>>,

    /// no-version | versioned | versioned-update
    pub versioning: Option<String>,

    /// Whether vRead can return past versions
    #[serde(rename = "readHistory")]
    pub read_history: Option<bool>,

    /// If update can commit to a new identity
    #[serde(rename = "updateCreate")]
    pub update_create: Option<bool>,

    /// If allows/uses conditional create
    #[serde(rename = "conditionalCreate")]
    pub conditional_create: Option<bool>,

    /// not-supported | modified-since | not-match | full-support
    #[serde(rename = "conditionalRead")]
    pub conditional_read: Option<String>,

    /// If allows/uses conditional update
    #[serde(rename = "conditionalUpdate")]
    pub conditional_update: Option<bool>,

    /// If allows/uses conditional patch
    #[serde(rename = "conditionalPatch")]
    pub conditional_patch: Option<bool>,

    /// not-supported | single | multiple - how conditional delete is supported
    #[serde(rename = "conditionalDelete")]
    pub conditional_delete: Option<String>,

    /// literal | logical | resolves | enforced | local
    #[serde(rename = "referencePolicy")]
    pub reference_policy: Option<Vec<String>>,

    /// _include values supported by the server
    #[serde(rename = "searchInclude")]
    pub search_include: Option<Vec<String>>,

    /// _revinclude values supported by the server
    #[serde(rename = "searchRevInclude")]
    pub search_rev_include: Option<Vec<String>>,

    /// Search parameters supported by implementation
    #[serde(rename = "searchParam")]
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>>,

    /// Definition of a resource operation
    pub operation: Option<Vec<CapabilityStatementRestResourceOperation>>,
}

/// What operations are supported?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// read | vread | update | patch | delete | history-instance | history-type | create | search-type
    pub code: String,

    /// Anything special about operation behavior
    pub documentation: Option<String>,
}

/// Search parameters supported by implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceSearchParam {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name for parameter in search url
    pub name: String,

    /// Source of definition for parameter
    pub definition: Option<String>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    #[serde(rename = "type")]
    pub r#type: String,

    /// Server-specific usage
    pub documentation: Option<String>,
}

/// Definition of a resource operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceOperation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name by which the operation/query is invoked
    pub name: String,

    /// The defined operation/query
    pub definition: String,

    /// Specific details about operation behavior
    pub documentation: Option<String>,
}

/// What operations are supported?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// transaction | batch | search-system | history-system
    pub code: String,

    /// Anything special about operation behavior
    pub documentation: Option<String>,
}

/// If messaging is supported
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessaging {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Where messages should be sent
    pub endpoint: Option<Vec<CapabilityStatementMessagingEndpoint>>,

    /// Reliable Message Cache Length (min)
    #[serde(rename = "reliableCache")]
    pub reliable_cache: Option<u32>,

    /// Messaging interface behavior details
    pub documentation: Option<String>,

    /// Messages supported by this system
    #[serde(rename = "supportedMessage")]
    pub supported_message: Option<Vec<CapabilityStatementMessagingSupportedMessage>>,
}

/// Where messages should be sent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingEndpoint {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// http | ftp | mllp +
    pub protocol: Coding,

    /// Network address or identifier of the end-point
    pub address: String,
}

/// Messages supported by this system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingSupportedMessage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// sender | receiver
    pub mode: String,

    /// Message supported by this system
    pub definition: String,
}

/// Document definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementDocument {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// producer | consumer
    pub mode: String,

    /// Description of document support
    pub documentation: Option<String>,

    /// Constraint on the resources used in the document
    pub profile: String,
}

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server or Client for a particular version of FHIR that may be used as a statement of actual server functionality or a st...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement {
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

    /// Canonical identifier for this capability statement, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the CapabilityStatement (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the capability statement
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// Name for this capability statement (computer friendly)
    pub name: Option<String>,

    /// Name for this capability statement (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: String,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the capability statement
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for capability statement (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this capability statement is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// instance | capability | requirements
    pub kind: String,

    /// Canonical URL of another capability statement this implements
    pub instantiates: Option<Vec<String>>,

    /// Canonical URL of another capability statement this adds to
    pub imports: Option<Vec<String>>,

    /// Software that is covered by this capability statement
    pub software: Option<CapabilityStatementSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<CapabilityStatementImplementation>,

    /// FHIR Version the system supports
    #[serde(rename = "fhirVersion")]
    pub fhir_version: String,

    /// formats supported (xml | json | ttl | mime type)
    pub format: Vec<String>,

    /// Patch formats supported
    #[serde(rename = "patchFormat")]
    pub patch_format: Option<Vec<String>>,

    /// Languages supported
    #[serde(rename = "acceptLanguage")]
    pub accept_language: Option<Vec<String>>,

    /// Implementation guides supported
    #[serde(rename = "implementationGuide")]
    pub implementation_guide: Option<Vec<String>>,

    /// If the endpoint is a RESTful one
    pub rest: Option<Vec<CapabilityStatementRest>>,

    /// If messaging is supported
    pub messaging: Option<Vec<CapabilityStatementMessaging>>,

    /// Document definition
    pub document: Option<Vec<CapabilityStatementDocument>>,
}
