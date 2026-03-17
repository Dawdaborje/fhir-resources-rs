//! FHIR R5 SubstancePolymer Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

    /// Captures the type of ratio to the entire polymer, e.g. Monomer/Polymer ratio, SRU/Polymer Ratio
    #[serde(rename = "ratioType")]
    pub ratio_type: Option<CodeableConcept>,

    /// The starting materials - monomer(s) used in the synthesis of the polymer
    #[serde(rename = "startingMaterial")]
    pub starting_material: Option<Vec<SubstancePolymerMonomerSetStartingMaterial>>,
}

/// The starting materials - monomer(s) used in the synthesis of the polymer
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

    /// The type of substance for this starting material
    pub code: Option<CodeableConcept>,

    /// Substance high level category, e.g. chemical substance
    pub category: Option<CodeableConcept>,

    /// Used to specify whether the attribute described is a defining element for the unique identification of the polymer
    #[serde(rename = "isDefining")]
    pub is_defining: Option<bool>,

    /// A percentage
    pub amount: Option<Quantity>,
}

/// Specifies and quantifies the repeated units and their configuration
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

    /// A representation of an (average) molecular formula from a polymer
    #[serde(rename = "averageMolecularFormula")]
    pub average_molecular_formula: Option<String>,

    /// How the quantitative amount of Structural Repeat Units is captured (e.g. Exact, Numeric, Average)
    #[serde(rename = "repeatUnitAmountType")]
    pub repeat_unit_amount_type: Option<CodeableConcept>,

    /// An SRU - Structural Repeat Unit
    #[serde(rename = "repeatUnit")]
    pub repeat_unit: Option<Vec<SubstancePolymerRepeatRepeatUnit>>,
}

/// An SRU - Structural Repeat Unit
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

    /// Structural repeat units are essential elements for defining polymers
    pub unit: Option<String>,

    /// The orientation of the polymerisation, e.g. head-tail, head-head, random
    pub orientation: Option<CodeableConcept>,

    /// Number of repeats of this unit
    pub amount: Option<i32>,

    /// Applies to homopolymer and block co-polymers where the degree of polymerisation within a block can be described
    #[serde(rename = "degreeOfPolymerisation")]
    pub degree_of_polymerisation:
        Option<Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>>,

    /// A graphical structure for this SRU
    #[serde(rename = "structuralRepresentation")]
    pub structural_representation:
        Option<Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>>,
}

/// Applies to homopolymer and block co-polymers where the degree of polymerisation within a block can be described
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

    /// The type of the degree of polymerisation shall be described, e.g. SRU/Polymer Ratio
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// An average amount of polymerisation
    pub average: Option<i32>,

    /// A low expected limit of the amount
    pub low: Option<i32>,

    /// A high expected limit of the amount
    pub high: Option<i32>,
}

/// A graphical structure for this SRU
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

    /// The type of structure (e.g. Full, Partial, Representative)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The structural representation as text string in a standard format e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF
    pub representation: Option<String>,

    /// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF
    pub format: Option<CodeableConcept>,

    /// An attached file with the structural representation
    pub attachment: Option<Attachment>,
}

/// Properties of a substance specific to it being a polymer.
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

    /// A business idenfier for this polymer, but typically this is handled by a SubstanceDefinition identifier
    pub identifier: Option<Box<Identifier>>,

    /// Overall type of the polymer
    pub class: Option<CodeableConcept>,

    /// Polymer geometry, e.g. linear, branched, cross-linked, network or dendritic
    pub geometry: Option<CodeableConcept>,

    /// Descrtibes the copolymer sequence type (polymer connectivity)
    #[serde(rename = "copolymerConnectivity")]
    pub copolymer_connectivity: Option<Vec<CodeableConcept>>,

    /// Todo - this is intended to connect to a repeating full modification structure, also used by Protein and Nucleic Acid . String is just a placeholder
    pub modification: Option<String>,

    /// Todo
    #[serde(rename = "monomerSet")]
    pub monomer_set: Option<Vec<SubstancePolymerMonomerSet>>,

    /// Specifies and quantifies the repeated units and their configuration
    pub repeat: Option<Vec<SubstancePolymerRepeat>>,
}
