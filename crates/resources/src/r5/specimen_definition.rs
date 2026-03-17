//! FHIR R5 SpecimenDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Specimen in container intended for testing by lab
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTested {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Primary or secondary specimen
    #[serde(rename = "isDerived")]
    pub is_derived: Option<bool>,

    /// Type of intended specimen
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// preferred | alternate
    pub preference: String,

    /// The specimen's container
    pub container: Option<SpecimenDefinitionTypeTestedContainer>,

    /// Requirements for specimen delivery and special handling
    pub requirement: Option<String>,

    /// The usual time for retaining this kind of specimen
    #[serde(rename = "retentionTime")]
    pub retention_time: Option<Duration>,

    /// Specimen for single use only
    #[serde(rename = "singleUse")]
    pub single_use: Option<bool>,

    /// Criterion specified for specimen rejection
    #[serde(rename = "rejectionCriterion")]
    pub rejection_criterion: Option<Vec<CodeableConcept>>,

    /// Specimen handling before testing
    pub handling: Option<Vec<SpecimenDefinitionTypeTestedHandling>>,

    /// Where the specimen will be tested
    #[serde(rename = "testingDestination")]
    pub testing_destination: Option<Vec<CodeableConcept>>,
}

/// The specimen's container
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTestedContainer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The material type used for the container
    pub material: Option<CodeableConcept>,

    /// Kind of container associated with the kind of specimen
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Color of container cap
    pub cap: Option<CodeableConcept>,

    /// The description of the kind of container
    pub description: Option<String>,

    /// The capacity of this kind of container
    pub capacity: Option<Quantity>,

    /// Minimum volume
    #[serde(rename = "minimumVolume")]
    pub minimum_volume: Option<serde_json::Value>,

    /// Additive associated with container
    pub additive: Option<Vec<SpecimenDefinitionTypeTestedContainerAdditive>>,

    /// Special processing applied to the container for this specimen type
    pub preparation: Option<String>,
}

/// Additive associated with container
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Additive associated with container
    pub additive: serde_json::Value,
}

/// Specimen handling before testing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTestedHandling {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Qualifies the interval of temperature
    #[serde(rename = "temperatureQualifier")]
    pub temperature_qualifier: Option<CodeableConcept>,

    /// Temperature range for these handling instructions
    #[serde(rename = "temperatureRange")]
    pub temperature_range: Option<Range>,

    /// Maximum preservation time
    #[serde(rename = "maxDuration")]
    pub max_duration: Option<Duration>,

    /// Preservation instruction
    pub instruction: Option<String>,
}

/// A kind of specimen with associated set of requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinition {
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

    /// Logical canonical URL to reference this SpecimenDefinition (globally unique)
    pub url: Option<String>,

    /// Business identifier
    pub identifier: Option<Box<Identifier>>,

    /// Business version of the SpecimenDefinition
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this {{title}} (computer friendly)
    pub name: Option<String>,

    /// Name for this SpecimenDefinition (Human friendly)
    pub title: Option<String>,

    /// Based on FHIR definition of another SpecimenDefinition
    #[serde(rename = "derivedFromCanonical")]
    pub derived_from_canonical: Option<Vec<String>>,

    /// Based on external definition
    #[serde(rename = "derivedFromUri")]
    pub derived_from_uri: Option<Vec<String>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// If this SpecimenDefinition is not for real usage
    pub experimental: Option<bool>,

    /// Type of subject for specimen collection
    pub subject: Option<serde_json::Value>,

    /// Date status first applied
    pub date: Option<String>,

    /// The name of the individual or organization that published the SpecimenDefinition
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the SpecimenDefinition
    pub description: Option<String>,

    /// Content intends to support these contexts
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for this SpecimenDefinition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this SpecimenDefinition is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When SpecimenDefinition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// The date on which the asset content was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// The effective date range for the SpecimenDefinition
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// Kind of material to collect
    #[serde(rename = "typeCollected")]
    pub type_collected: Option<CodeableConcept>,

    /// Patient preparation for collection
    #[serde(rename = "patientPreparation")]
    pub patient_preparation: Option<Vec<CodeableConcept>>,

    /// Time aspect for collection
    #[serde(rename = "timeAspect")]
    pub time_aspect: Option<String>,

    /// Specimen collection procedure
    pub collection: Option<Vec<CodeableConcept>>,

    /// Specimen in container intended for testing by lab
    #[serde(rename = "typeTested")]
    pub type_tested: Option<Vec<SpecimenDefinitionTypeTested>>,
}
