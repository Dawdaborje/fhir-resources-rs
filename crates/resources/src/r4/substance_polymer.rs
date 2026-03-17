//! FHIR R4 SubstancePolymer Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerMonomerSet {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    #[serde(rename = "ratioType")]
    pub ratio_type: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "startingMaterial")]
    pub starting_material: Option<Vec<SubstancePolymerMonomerSetStartingMaterial>>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    pub material: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "isDefining")]
    pub is_defining: Option<bool>,

    /// Todo
    pub amount: Option<SubstanceAmount>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeat {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    #[serde(rename = "numberOfUnits")]
    pub number_of_units: Option<i32>,

    /// Todo
    #[serde(rename = "averageMolecularFormula")]
    pub average_molecular_formula: Option<String>,

    /// Todo
    #[serde(rename = "repeatUnitAmountType")]
    pub repeat_unit_amount_type: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "repeatUnit")]
    pub repeat_unit: Option<Vec<SubstancePolymerRepeatRepeatUnit>>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeatRepeatUnit {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    #[serde(rename = "orientationOfPolymerisation")]
    pub orientation_of_polymerisation: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "repeatUnit")]
    pub repeat_unit: Option<String>,

    /// Todo
    pub amount: Option<SubstanceAmount>,

    /// Todo
    #[serde(rename = "degreeOfPolymerisation")]
    pub degree_of_polymerisation:
        Option<Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>>,

    /// Todo
    #[serde(rename = "structuralRepresentation")]
    pub structural_representation:
        Option<Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    pub degree: Option<CodeableConcept>,

    /// Todo
    pub amount: Option<SubstanceAmount>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Todo
    pub representation: Option<String>,

    /// Todo
    pub attachment: Option<Attachment>,
}

/// Todo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymer {
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

    /// Todo
    pub class: Option<CodeableConcept>,

    /// Todo
    pub geometry: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "copolymerConnectivity")]
    pub copolymer_connectivity: Option<Vec<CodeableConcept>>,

    /// Todo
    pub modification: Option<Vec<String>>,

    /// Todo
    #[serde(rename = "monomerSet")]
    pub monomer_set: Option<Vec<SubstancePolymerMonomerSet>>,

    /// Todo
    pub repeat: Option<Vec<SubstancePolymerRepeat>>,
}
