//! FHIR R4 SubstanceSpecification Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Moiety, for structural modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationMoiety {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Role that the moiety is playing
    pub role: Option<CodeableConcept>,

    /// Identifier by which this moiety substance is known
    pub identifier: Option<Box<Identifier>>,

    /// Textual name for this moiety substance
    pub name: Option<String>,

    /// Stereochemistry type
    pub stereochemistry: Option<CodeableConcept>,

    /// Optical activity type
    #[serde(rename = "opticalActivity")]
    pub optical_activity: Option<CodeableConcept>,

    /// Molecular formula
    #[serde(rename = "molecularFormula")]
    pub molecular_formula: Option<String>,

    /// Quantitative value for this moiety
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,

    #[serde(rename = "amountString")]
    pub amount_string: Option<String>,
}

/// General specifications for this substance, including how it is related to other substances
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A category for this property, e.g. Physical, Chemical, Enzymatic
    pub category: Option<CodeableConcept>,

    /// Property type e.g. viscosity, pH, isoelectric point
    pub code: Option<CodeableConcept>,

    /// Parameters that were used in the measurement of a property (e.g. for viscosity: measured at 20C with a pH of 7.1)
    pub parameters: Option<String>,

    /// A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol)
    #[serde(rename = "definingSubstanceReference")]
    pub defining_substance_reference: Option<Box<Reference>>,

    #[serde(rename = "definingSubstanceCodeableConcept")]
    pub defining_substance_codeable_concept: Option<CodeableConcept>,

    /// Quantitative value for this property
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,

    #[serde(rename = "amountString")]
    pub amount_string: Option<String>,
}

/// Structural information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationStructure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Stereochemistry type
    pub stereochemistry: Option<CodeableConcept>,

    /// Optical activity type
    #[serde(rename = "opticalActivity")]
    pub optical_activity: Option<CodeableConcept>,

    /// Molecular formula
    #[serde(rename = "molecularFormula")]
    pub molecular_formula: Option<String>,

    /// Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot
    #[serde(rename = "molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety: Option<String>,

    /// Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio
    pub isotope: Option<Vec<SubstanceSpecificationStructureIsotope>>,

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids)
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,

    /// Molecular structural representation
    pub representation: Option<Vec<SubstanceSpecificationStructureRepresentation>>,
}

/// Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationStructureIsotope {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Substance identifier for each non-natural or radioisotope
    pub identifier: Option<Box<Identifier>>,

    /// Substance name for each non-natural or radioisotope
    pub name: Option<CodeableConcept>,

    /// The type of isotopic substitution present in a single substance
    pub substitution: Option<CodeableConcept>,

    /// Half life - for a non-natural nuclide
    #[serde(rename = "halfLife")]
    pub half_life: Option<Quantity>,

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids)
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
}

/// The molecular weight or weight range (for proteins, polymers or nucleic acids)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The method by which the molecular weight was determined
    pub method: Option<CodeableConcept>,

    /// Type of molecular weight such as exact, average (also known as. number average), weight average
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it w...
    pub amount: Option<Quantity>,
}

/// Molecular structural representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationStructureRepresentation {
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

    /// The structural representation as text string in a format e.g. InChI, SMILES, MOLFILE, CDX
    pub representation: Option<String>,

    /// An attached file with the structural representation
    pub attachment: Option<Attachment>,
}

/// Codes associated with the substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationCode {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specific code
    pub code: Option<CodeableConcept>,

    /// Status of the code assignment
    pub status: Option<CodeableConcept>,

    /// The date at which the code status is changed as part of the terminology maintenance
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// Any comment can be provided in this field, if necessary
    pub comment: Option<String>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,
}

/// Names applicable to this substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The actual name
    pub name: String,

    /// Name type
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The status of the name
    pub status: Option<CodeableConcept>,

    /// If this is the preferred name for this substance
    pub preferred: Option<bool>,

    /// Language of the name
    pub language: Option<Vec<CodeableConcept>>,

    /// The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive
    pub domain: Option<Vec<CodeableConcept>>,

    /// The jurisdiction where this name applies
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// A synonym of this name
    pub synonym: Option<Vec<SubstanceSpecificationName>>,

    /// A translation for this name
    pub translation: Option<Vec<SubstanceSpecificationName>>,

    /// Details of the official nature of this name
    pub official: Option<Vec<SubstanceSpecificationNameOfficial>>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,
}

/// Details of the official nature of this name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationNameOfficial {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Which authority uses this official name
    pub authority: Option<CodeableConcept>,

    /// The status of the official name
    pub status: Option<CodeableConcept>,

    /// Date of official name change
    pub date: Option<String>,
}

/// A link between this substance and another, with details of the relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecificationRelationship {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A pointer to another substance, as a resource or just a representational code
    #[serde(rename = "substanceReference")]
    pub substance_reference: Option<Box<Reference>>,

    #[serde(rename = "substanceCodeableConcept")]
    pub substance_codeable_concept: Option<CodeableConcept>,

    /// For example "salt to parent", "active moiety", "starting material"
    pub relationship: Option<CodeableConcept>,

    /// For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships
    #[serde(rename = "isDefining")]
    pub is_defining: Option<bool>,

    /// A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,

    #[serde(rename = "amountRange")]
    pub amount_range: Option<Range>,

    #[serde(rename = "amountRatio")]
    pub amount_ratio: Option<Ratio>,

    #[serde(rename = "amountString")]
    pub amount_string: Option<String>,

    /// For use when the numeric
    #[serde(rename = "amountRatioLowLimit")]
    pub amount_ratio_low_limit: Option<Ratio>,

    /// An operator for the amount, for example "average", "approximately", "less than"
    #[serde(rename = "amountType")]
    pub amount_type: Option<CodeableConcept>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,
}

/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecification {
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

    /// Identifier by which this substance is known
    pub identifier: Option<Box<Identifier>>,

    /// High level categorization, e.g. polymer or nucleic acid
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Status of substance within the catalogue e.g. approved
    pub status: Option<CodeableConcept>,

    /// If the substance applies to only human or veterinary use
    pub domain: Option<CodeableConcept>,

    /// Textual description of the substance
    pub description: Option<String>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,

    /// Textual comment about this record of a substance
    pub comment: Option<String>,

    /// Moiety, for structural modifications
    pub moiety: Option<Vec<SubstanceSpecificationMoiety>>,

    /// General specifications for this substance, including how it is related to other substances
    pub property: Option<Vec<SubstanceSpecificationProperty>>,

    /// General information detailing this substance
    #[serde(rename = "referenceInformation")]
    pub reference_information: Option<Box<Reference>>,

    /// Structural information
    pub structure: Option<SubstanceSpecificationStructure>,

    /// Codes associated with the substance
    pub code: Option<Vec<SubstanceSpecificationCode>>,

    /// Names applicable to this substance
    pub name: Option<Vec<SubstanceSpecificationName>>,

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids)
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>>,

    /// A link between this substance and another, with details of the relationship
    pub relationship: Option<Vec<SubstanceSpecificationRelationship>>,

    /// Data items specific to nucleic acids
    #[serde(rename = "nucleicAcid")]
    pub nucleic_acid: Option<Box<Reference>>,

    /// Data items specific to polymers
    pub polymer: Option<Box<Reference>>,

    /// Data items specific to proteins
    pub protein: Option<Box<Reference>>,

    /// Material or taxonomic/anatomical source for the substance
    #[serde(rename = "sourceMaterial")]
    pub source_material: Option<Box<Reference>>,
}
