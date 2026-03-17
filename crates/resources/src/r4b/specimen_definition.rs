//! FHIR R4B SpecimenDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
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

    /// Specimen requirements
    pub requirement: Option<String>,

    /// Specimen retention time
    #[serde(rename = "retentionTime")]
    pub retention_time: Option<Duration>,

    /// Rejection criterion
    #[serde(rename = "rejectionCriterion")]
    pub rejection_criterion: Option<Vec<CodeableConcept>>,

    /// Specimen handling before testing
    pub handling: Option<Vec<SpecimenDefinitionTypeTestedHandling>>,
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

    /// Container material
    pub material: Option<CodeableConcept>,

    /// Kind of container associated with the kind of specimen
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Color of container cap
    pub cap: Option<CodeableConcept>,

    /// Container description
    pub description: Option<String>,

    /// Container capacity
    pub capacity: Option<Quantity>,

    /// Minimum volume
    #[serde(rename = "minimumVolumeQuantity")]
    pub minimum_volume_quantity: Option<Quantity>,

    #[serde(rename = "minimumVolumeString")]
    pub minimum_volume_string: Option<String>,

    /// Additive associated with container
    pub additive: Option<Vec<SpecimenDefinitionTypeTestedContainerAdditive>>,

    /// Specimen container preparation
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
    #[serde(rename = "additiveCodeableConcept")]
    pub additive_codeable_concept: CodeableConcept,

    #[serde(rename = "additiveReference")]
    pub additive_reference: Box<Reference>,
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

    /// Temperature qualifier
    #[serde(rename = "temperatureQualifier")]
    pub temperature_qualifier: Option<CodeableConcept>,

    /// Temperature range
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

    /// Business identifier of a kind of specimen
    pub identifier: Option<Box<Identifier>>,

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
