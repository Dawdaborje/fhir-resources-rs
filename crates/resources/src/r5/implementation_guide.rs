//! FHIR R5 ImplementationGuide Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Another Implementation guide this depends on
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDependsOn {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identity of the IG that this depends on
    pub uri: String,

    /// NPM Package name for IG this depends on
    #[serde(rename = "packageId")]
    pub package_id: Option<String>,

    /// Version of the IG
    pub version: Option<String>,

    /// Why dependency exists
    pub reason: Option<String>,
}

/// Profiles that apply globally
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideGlobal {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type this profile applies to
    #[serde(rename = "type")]
    pub r#type: String,

    /// Profile that all resources must conform to
    pub profile: String,
}

/// Information needed to build the IG
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Grouping used to present related resources in the IG
    pub grouping: Option<Vec<ImplementationGuideDefinitionGrouping>>,

    /// Resource in the implementation guide
    pub resource: Option<Vec<ImplementationGuideDefinitionResource>>,

    /// Page/Section in the Guide
    pub page: Option<ImplementationGuideDefinitionPage>,

    /// Defines how IG is built by tools
    pub parameter: Option<Vec<ImplementationGuideDefinitionParameter>>,

    /// A template for building resources
    pub template: Option<Vec<ImplementationGuideDefinitionTemplate>>,
}

/// Grouping used to present related resources in the IG
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionGrouping {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Descriptive name for the package
    pub name: String,

    /// Human readable text describing the package
    pub description: Option<String>,
}

/// Resource in the implementation guide
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionResource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Location of the resource
    pub reference: Box<Reference>,

    /// Versions this applies to (if different to IG)
    #[serde(rename = "fhirVersion")]
    pub fhir_version: Option<Vec<String>>,

    /// Human readable name for the resource
    pub name: Option<String>,

    /// Reason why included in guide
    pub description: Option<String>,

    /// Is this an example
    #[serde(rename = "isExample")]
    pub is_example: Option<bool>,

    /// Profile(s) this is an example of
    pub profile: Option<Vec<String>>,

    /// Grouping this is part of
    #[serde(rename = "groupingId")]
    pub grouping_id: Option<String>,
}

/// Page/Section in the Guide
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionPage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Source for page
    pub source: Option<serde_json::Value>,

    /// Name of the page when published
    pub name: String,

    /// Short title shown for navigational assistance
    pub title: String,

    /// html | markdown | xml | generated
    pub generation: String,

    /// Nested Pages / Sections
    pub page: Option<Vec<ImplementationGuideDefinitionPage>>,
}

/// Defines how IG is built by tools
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code that identifies parameter
    pub code: Coding,

    /// Value for named type
    pub value: String,
}

/// A template for building resources
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of template specified
    pub code: String,

    /// The source location for the template
    pub source: String,

    /// The scope in which the template applies
    pub scope: Option<String>,
}

/// Information about an assembled IG
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifest {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Location of rendered implementation guide
    pub rendering: Option<String>,

    /// Resource in the implementation guide
    pub resource: Vec<ImplementationGuideManifestResource>,

    /// HTML page within the parent IG
    pub page: Option<Vec<ImplementationGuideManifestPage>>,

    /// Image within the IG
    pub image: Option<Vec<String>>,

    /// Additional linkable file in IG
    pub other: Option<Vec<String>>,
}

/// Resource in the implementation guide
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifestResource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Location of the resource
    pub reference: Box<Reference>,

    /// Is this an example
    #[serde(rename = "isExample")]
    pub is_example: Option<bool>,

    /// Profile(s) this is an example of
    pub profile: Option<Vec<String>>,

    /// Relative path for page in IG
    #[serde(rename = "relativePath")]
    pub relative_path: Option<String>,
}

/// HTML page within the parent IG
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifestPage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// HTML page name
    pub name: String,

    /// Title of the page, for references
    pub title: Option<String>,

    /// Anchor available on the page
    pub anchor: Option<Vec<String>>,
}

/// A set of rules of how a particular interoperability or standards problem is solved - typically through the use of FHIR resources. This resource is used to gather all the parts of an implementation ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuide {
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

    /// Canonical identifier for this implementation guide, represented as a URI (globally unique)
    pub url: String,

    /// Additional identifier for the implementation guide (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the implementation guide
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this implementation guide (computer friendly)
    pub name: String,

    /// Name for this implementation guide (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the implementation guide
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for implementation guide (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this implementation guide is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// NPM Package name for IG
    #[serde(rename = "packageId")]
    pub package_id: String,

    /// SPDX license code for this IG (or not-open-source)
    pub license: Option<String>,

    /// FHIR Version(s) this Implementation Guide targets
    #[serde(rename = "fhirVersion")]
    pub fhir_version: Vec<String>,

    /// Another Implementation guide this depends on
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<Vec<ImplementationGuideDependsOn>>,

    /// Profiles that apply globally
    pub global: Option<Vec<ImplementationGuideGlobal>>,

    /// Information needed to build the IG
    pub definition: Option<ImplementationGuideDefinition>,

    /// Information about an assembled IG
    pub manifest: Option<ImplementationGuideManifest>,
}
