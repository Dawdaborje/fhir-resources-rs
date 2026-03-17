//! FHIR R5 FamilyMemberHistory Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Who or what participated in the activities related to the family member history and how they were involved
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistoryParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of involvement
    pub function: Option<CodeableConcept>,

    /// Who or what participated in the activities related to the family member history
    pub actor: Box<Reference>,
}

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

    /// deceased | permanent disability | etc
    pub outcome: Option<CodeableConcept>,

    /// Whether the condition contributed to the cause of death
    #[serde(rename = "contributedToDeath")]
    pub contributed_to_death: Option<bool>,

    /// When condition first manifested
    pub onset: Option<serde_json::Value>,

    /// Extra information about condition
    pub note: Option<Vec<Annotation>>,
}

/// Procedures that the related person had
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistoryProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Procedures performed on the related person
    pub code: CodeableConcept,

    /// What happened following the procedure
    pub outcome: Option<CodeableConcept>,

    /// Whether the procedure contributed to the cause of death
    #[serde(rename = "contributedToDeath")]
    pub contributed_to_death: Option<bool>,

    /// When the procedure was performed
    pub performed: Option<serde_json::Value>,

    /// Extra information about the procedure
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

    /// Who or what participated in the activities related to the family member history and how they were involved
    pub participant: Option<Vec<FamilyMemberHistoryParticipant>>,

    /// The family member described
    pub name: Option<String>,

    /// Relationship to the subject
    pub relationship: CodeableConcept,

    /// male | female | other | unknown
    pub sex: Option<CodeableConcept>,

    /// (approximate) date of birth
    pub born: Option<serde_json::Value>,

    /// (approximate) age
    pub age: Option<serde_json::Value>,

    /// Age is estimated?
    #[serde(rename = "estimatedAge")]
    pub estimated_age: Option<bool>,

    /// Dead? How old/when?
    pub deceased: Option<serde_json::Value>,

    /// Why was family member history performed?
    pub reason: Option<Vec<CodeableReference>>,

    /// General note about related person
    pub note: Option<Vec<Annotation>>,

    /// Condition that the related person had
    pub condition: Option<Vec<FamilyMemberHistoryCondition>>,

    /// Procedures that the related person had
    pub procedure: Option<Vec<FamilyMemberHistoryProcedure>>,
}
