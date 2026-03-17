//! FHIR R4 MedicinalProductIndication Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Information about the use of the medicinal product in relation to other therapies described as part of the indication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIndicationOtherTherapy {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of relationship between the medicinal product indication or contraindication and another therapy
    #[serde(rename = "therapyRelationshipType")]
    pub therapy_relationship_type: CodeableConcept,

    /// Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication
    pub medication: serde_json::Value,
}

/// Indication for the Medicinal Product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIndication {
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

    /// The medication for which this is an indication
    pub subject: Option<Vec<Box<Reference>>>,

    /// The disease, symptom or procedure that is the indication for treatment
    #[serde(rename = "diseaseSymptomProcedure")]
    pub disease_symptom_procedure: Option<CodeableConcept>,

    /// The status of the disease or symptom for which the indication applies
    #[serde(rename = "diseaseStatus")]
    pub disease_status: Option<CodeableConcept>,

    /// Comorbidity (concurrent condition) or co-infection as part of the indication
    pub comorbidity: Option<Vec<CodeableConcept>>,

    /// The intended effect, aim or strategy to be achieved by the indication
    #[serde(rename = "intendedEffect")]
    pub intended_effect: Option<CodeableConcept>,

    /// Timing or duration information as part of the indication
    pub duration: Option<Quantity>,

    /// Information about the use of the medicinal product in relation to other therapies described as part of the indication
    #[serde(rename = "otherTherapy")]
    pub other_therapy: Option<Vec<MedicinalProductIndicationOtherTherapy>>,

    /// Describe the undesirable effects of the medicinal product
    #[serde(rename = "undesirableEffect")]
    pub undesirable_effect: Option<Vec<Box<Reference>>>,

    /// The population group to which this applies
    pub population: Option<Vec<Population>>,
}
