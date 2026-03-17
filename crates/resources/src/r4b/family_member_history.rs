//! FHIR R4B FamilyMemberHistory Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Condition that the related person had
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistoryCondition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Condition suffered by relation
    pub code: CodeableConcept,

    /// deceased | permanent disability | etc.
    pub outcome: Option<CodeableConcept>,

    /// Whether the condition contributed to the cause of death
    #[serde(rename = "contributedToDeath")]
    pub contributed_to_death: Option<bool>,

    /// When condition first manifested
    #[serde(rename = "onsetAge")]
    pub onset_age: Option<Age>,

    #[serde(rename = "onsetRange")]
    pub onset_range: Option<Range>,

    #[serde(rename = "onsetPeriod")]
    pub onset_period: Option<Period>,

    #[serde(rename = "onsetString")]
    pub onset_string: Option<String>,

    /// Extra information about condition
    pub note: Option<Vec<Annotation>>,
}

/// Significant health conditions for a person related to the patient relevant in the context of care for the patient.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistory {
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

    /// External Id(s) for this record
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// partial | completed | entered-in-error | health-unknown
    pub status: String,

    /// subject-unknown | withheld | unable-to-obtain | deferred
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,

    /// Patient history is about
    pub patient: Box<Reference>,

    /// When history was recorded or last updated
    pub date: Option<String>,

    /// The family member described
    pub name: Option<String>,

    /// Relationship to the subject
    pub relationship: CodeableConcept,

    /// male | female | other | unknown
    pub sex: Option<CodeableConcept>,

    /// (approximate) date of birth
    #[serde(rename = "bornPeriod")]
    pub born_period: Option<Period>,

    #[serde(rename = "bornDate")]
    pub born_date: Option<String>,

    #[serde(rename = "bornString")]
    pub born_string: Option<String>,

    /// (approximate) age
    #[serde(rename = "ageAge")]
    pub age_age: Option<Age>,

    #[serde(rename = "ageRange")]
    pub age_range: Option<Range>,

    #[serde(rename = "ageString")]
    pub age_string: Option<String>,

    /// Age is estimated?
    #[serde(rename = "estimatedAge")]
    pub estimated_age: Option<bool>,

    /// Dead? How old/when?
    #[serde(rename = "deceasedBoolean")]
    pub deceased_boolean: Option<bool>,

    #[serde(rename = "deceasedAge")]
    pub deceased_age: Option<Age>,

    #[serde(rename = "deceasedRange")]
    pub deceased_range: Option<Range>,

    #[serde(rename = "deceasedDate")]
    pub deceased_date: Option<String>,

    #[serde(rename = "deceasedString")]
    pub deceased_string: Option<String>,

    /// Why was family member history performed?
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why was family member history performed?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// General note about related person
    pub note: Option<Vec<Annotation>>,

    /// Condition that the related person had
    pub condition: Option<Vec<FamilyMemberHistoryCondition>>,
}
