//! FHIR R5 SubstanceDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Moiety, for structural modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionMoiety {
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

    /// Molecular formula for this moiety (e.g. with the Hill system)
    #[serde(rename = "molecularFormula")]
    pub molecular_formula: Option<String>,

    /// Quantitative value for this moiety
    pub amount: Option<serde_json::Value>,

    /// The measurement type of the quantitative value
    #[serde(rename = "measurementType")]
    pub measurement_type: Option<CodeableConcept>,
}

/// General specifications for this substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionCharacterization {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The method used to find the characterization e.g. HPLC
    pub technique: Option<CodeableConcept>,

    /// Describes the nature of the chemical entity and explains, for instance, whether this is a base or a salt form
    pub form: Option<CodeableConcept>,

    /// The description or justification in support of the interpretation of the data file
    pub description: Option<String>,

    /// The data produced by the analytical instrument or a pictorial representation of that data. Examples: a JCAMP, JDX, or ADX file, or a chromatogram or spectrum analysis
    pub file: Option<Vec<Attachment>>,
}

/// General specifications for this substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A code expressing the type of property
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// A value for the property
    pub value: Option<serde_json::Value>,
}

/// The average mass of a molecule of a compound
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionMolecularWeight {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The method by which the weight was determined
    pub method: Option<CodeableConcept>,

    /// Type of molecular weight e.g. exact, average, weight average
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Used to capture quantitative values for a variety of elements
    pub amount: Quantity,
}

/// Structural information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionStructure {
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

    /// An expression which states the number and type of atoms present in a molecule of a substance
    #[serde(rename = "molecularFormula")]
    pub molecular_formula: Option<String>,

    /// Specified per moiety according to the Hill system
    #[serde(rename = "molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety: Option<String>,

    /// The molecular weight or weight range
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<SubstanceDefinitionMolecularWeight>,

    /// The method used to find the structure e.g. X-ray, NMR
    pub technique: Option<Vec<CodeableConcept>>,

    /// Source of information for the structure
    #[serde(rename = "sourceDocument")]
    pub source_document: Option<Vec<Box<Reference>>>,

    /// A depiction of the structure of the substance
    pub representation: Option<Vec<SubstanceDefinitionStructureRepresentation>>,
}

/// A depiction of the structure of the substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionStructureRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kind of structural representation (e.g. full, partial)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The structural representation as a text string in a standard format
    pub representation: Option<String>,

    /// The format of the representation e.g. InChI, SMILES, MOLFILE (note: not the physical file format)
    pub format: Option<CodeableConcept>,

    /// An attachment with the structural representation e.g. a structure graphic or AnIML file
    pub document: Option<Box<Reference>>,
}

/// Codes associated with the substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionCode {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specific code
    pub code: Option<CodeableConcept>,

    /// Status of the code assignment, for example 'provisional', 'approved'
    pub status: Option<CodeableConcept>,

    /// The date at which the code status was changed
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// Any comment can be provided in this field
    pub note: Option<Vec<Annotation>>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,
}

/// Names applicable to this substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The actual name
    pub name: String,

    /// Name type e.g. 'systematic', 'scientific, 'brand'
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The status of the name e.g. 'current', 'proposed'
    pub status: Option<CodeableConcept>,

    /// If this is the preferred name for this substance
    pub preferred: Option<bool>,

    /// Human language that the name is written in
    pub language: Option<Vec<CodeableConcept>>,

    /// The use context of this name e.g. as an active ingredient or as a food colour additive
    pub domain: Option<Vec<CodeableConcept>>,

    /// The jurisdiction where this name applies
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// A synonym of this particular name, by which the substance is also known
    pub synonym: Option<Vec<SubstanceDefinitionName>>,

    /// A translation for this name into another human language
    pub translation: Option<Vec<SubstanceDefinitionName>>,

    /// Details of the official nature of this name
    pub official: Option<Vec<SubstanceDefinitionNameOfficial>>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,
}

/// Details of the official nature of this name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionNameOfficial {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Which authority uses this official name
    pub authority: Option<CodeableConcept>,

    /// The status of the official name, for example 'draft', 'active'
    pub status: Option<CodeableConcept>,

    /// Date of official name change
    pub date: Option<String>,
}

/// A link between this substance and another
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionRelationship {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A pointer to another substance, as a resource or a representational code
    #[serde(rename = "substanceDefinition")]
    pub substance_definition: Option<serde_json::Value>,

    /// For example "salt to parent", "active moiety"
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible relationships
    #[serde(rename = "isDefining")]
    pub is_defining: Option<bool>,

    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other
    pub amount: Option<serde_json::Value>,

    /// For use when the numeric has an uncertain range
    #[serde(rename = "ratioHighLimitAmount")]
    pub ratio_high_limit_amount: Option<Ratio>,

    /// An operator for the amount, for example "average", "approximately", "less than"
    pub comparator: Option<CodeableConcept>,

    /// Supporting literature
    pub source: Option<Vec<Box<Reference>>>,
}

/// Material or taxonomic/anatomical source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionSourceMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Classification of the origin of the raw material. e.g. cat hair is an Animal source type
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The genus of an organism e.g. the Latin epithet of the plant/animal scientific name
    pub genus: Option<CodeableConcept>,

    /// The species of an organism e.g. the Latin epithet of the species of the plant/animal
    pub species: Option<CodeableConcept>,

    /// An anatomical origin of the source material within an organism
    pub part: Option<CodeableConcept>,

    /// The country or countries where the material is harvested
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<Vec<CodeableConcept>>,
}

/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinition {
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
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// A business level version identifier of the substance
    pub version: Option<String>,

    /// Status of substance within the catalogue e.g. active, retired
    pub status: Option<CodeableConcept>,

    /// A categorization, high level e.g. polymer or nucleic acid, or food, chemical, biological, or lower e.g. polymer linear or branch chain, or type of impurity
    pub classification: Option<Vec<CodeableConcept>>,

    /// If the substance applies to human or veterinary use
    pub domain: Option<CodeableConcept>,

    /// The quality standard, established benchmark, to which substance complies (e.g. USP/NF, BP)
    pub grade: Option<Vec<CodeableConcept>>,

    /// Textual description of the substance
    pub description: Option<String>,

    /// Supporting literature
    #[serde(rename = "informationSource")]
    pub information_source: Option<Vec<Box<Reference>>>,

    /// Textual comment about the substance's catalogue or registry record
    pub note: Option<Vec<Annotation>>,

    /// The entity that creates, makes, produces or fabricates the substance
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// An entity that is the source for the substance. It may be different from the manufacturer
    pub supplier: Option<Vec<Box<Reference>>>,

    /// Moiety, for structural modifications
    pub moiety: Option<Vec<SubstanceDefinitionMoiety>>,

    /// General specifications for this substance
    pub characterization: Option<Vec<SubstanceDefinitionCharacterization>>,

    /// General specifications for this substance
    pub property: Option<Vec<SubstanceDefinitionProperty>>,

    /// General information detailing this substance
    #[serde(rename = "referenceInformation")]
    pub reference_information: Option<Box<Reference>>,

    /// The average mass of a molecule of a compound
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<Vec<SubstanceDefinitionMolecularWeight>>,

    /// Structural information
    pub structure: Option<SubstanceDefinitionStructure>,

    /// Codes associated with the substance
    pub code: Option<Vec<SubstanceDefinitionCode>>,

    /// Names applicable to this substance
    pub name: Option<Vec<SubstanceDefinitionName>>,

    /// A link between this substance and another
    pub relationship: Option<Vec<SubstanceDefinitionRelationship>>,

    /// Data items specific to nucleic acids
    #[serde(rename = "nucleicAcid")]
    pub nucleic_acid: Option<Box<Reference>>,

    /// Data items specific to polymers
    pub polymer: Option<Box<Reference>>,

    /// Data items specific to proteins
    pub protein: Option<Box<Reference>>,

    /// Material or taxonomic/anatomical source
    #[serde(rename = "sourceMaterial")]
    pub source_material: Option<SubstanceDefinitionSourceMaterial>,
}
