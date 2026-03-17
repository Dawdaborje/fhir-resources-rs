//! FHIR R4 MolecularSequence Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// A sequence used as reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceReferenceSeq {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Chromosome containing genetic finding
    pub chromosome: Option<CodeableConcept>,

    /// The Genome Build used for reference, following GRCh build versions e.g. 'GRCh 37'
    #[serde(rename = "genomeBuild")]
    pub genome_build: Option<String>,

    /// sense | antisense
    pub orientation: Option<String>,

    /// Reference identifier
    #[serde(rename = "referenceSeqId")]
    pub reference_seq_id: Option<CodeableConcept>,

    /// A pointer to another MolecularSequence entity as reference sequence
    #[serde(rename = "referenceSeqPointer")]
    pub reference_seq_pointer: Option<Box<Reference>>,

    /// A string to represent reference sequence
    #[serde(rename = "referenceSeqString")]
    pub reference_seq_string: Option<String>,

    /// watson | crick
    pub strand: Option<String>,

    /// Start position of the window on the reference sequence
    #[serde(rename = "windowStart")]
    pub window_start: Option<i32>,

    /// End position of the window on the reference sequence
    #[serde(rename = "windowEnd")]
    pub window_end: Option<i32>,
}

/// Variant in sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceVariant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Start position of the variant on the reference sequence
    pub start: Option<i32>,

    /// End position of the variant on the reference sequence
    pub end: Option<i32>,

    /// Allele that was observed
    #[serde(rename = "observedAllele")]
    pub observed_allele: Option<String>,

    /// Allele in the reference sequence
    #[serde(rename = "referenceAllele")]
    pub reference_allele: Option<String>,

    /// Extended CIGAR string for aligning the sequence with reference bases
    pub cigar: Option<String>,

    /// Pointer to observed variant information
    #[serde(rename = "variantPointer")]
    pub variant_pointer: Option<Box<Reference>>,
}

/// An set of value as quality of sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceQuality {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// indel | snp | unknown
    #[serde(rename = "type")]
    pub r#type: String,

    /// Standard sequence for comparison
    #[serde(rename = "standardSequence")]
    pub standard_sequence: Option<CodeableConcept>,

    /// Start position of the sequence
    pub start: Option<i32>,

    /// End position of the sequence
    pub end: Option<i32>,

    /// Quality score for the comparison
    pub score: Option<Quantity>,

    /// Method to get quality
    pub method: Option<CodeableConcept>,

    /// True positives from the perspective of the truth data
    #[serde(rename = "truthTP")]
    pub truth_t_p: Option<f64>,

    /// True positives from the perspective of the query data
    #[serde(rename = "queryTP")]
    pub query_t_p: Option<f64>,

    /// False negatives
    #[serde(rename = "truthFN")]
    pub truth_f_n: Option<f64>,

    /// False positives
    #[serde(rename = "queryFP")]
    pub query_f_p: Option<f64>,

    /// False positives where the non-REF alleles in the Truth and Query Call Sets match
    #[serde(rename = "gtFP")]
    pub gt_f_p: Option<f64>,

    /// Precision of comparison
    pub precision: Option<f64>,

    /// Recall of comparison
    pub recall: Option<f64>,

    /// F-score
    #[serde(rename = "fScore")]
    pub f_score: Option<f64>,

    /// Receiver Operator Characteristic (ROC) Curve
    pub roc: Option<MolecularSequenceQualityRoc>,
}

/// Receiver Operator Characteristic (ROC) Curve
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceQualityRoc {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Genotype quality score
    pub score: Option<Vec<i32>>,

    /// Roc score true positive numbers
    #[serde(rename = "numTP")]
    pub num_t_p: Option<Vec<i32>>,

    /// Roc score false positive numbers
    #[serde(rename = "numFP")]
    pub num_f_p: Option<Vec<i32>>,

    /// Roc score false negative numbers
    #[serde(rename = "numFN")]
    pub num_f_n: Option<Vec<i32>>,

    /// Precision of the GQ score
    pub precision: Option<Vec<f64>>,

    /// Sensitivity of the GQ score
    pub sensitivity: Option<Vec<f64>>,

    /// FScore of the GQ score
    #[serde(rename = "fMeasure")]
    pub f_measure: Option<Vec<f64>>,
}

/// External repository which contains detailed report related with observedSeq in this resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRepository {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// directlink | openapi | login | oauth | other
    #[serde(rename = "type")]
    pub r#type: String,

    /// URI of the repository
    pub url: Option<String>,

    /// Repository's name
    pub name: Option<String>,

    /// Id of the dataset that used to call for dataset in repository
    #[serde(rename = "datasetId")]
    pub dataset_id: Option<String>,

    /// Id of the variantset that used to call for variantset in repository
    #[serde(rename = "variantsetId")]
    pub variantset_id: Option<String>,

    /// Id of the read
    #[serde(rename = "readsetId")]
    pub readset_id: Option<String>,
}

/// Structural variant
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceStructureVariant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Structural variant change type
    #[serde(rename = "variantType")]
    pub variant_type: Option<CodeableConcept>,

    /// Does the structural variant have base pair resolution breakpoints?
    pub exact: Option<bool>,

    /// Structural variant length
    pub length: Option<i32>,

    /// Structural variant outer
    pub outer: Option<MolecularSequenceStructureVariantOuter>,

    /// Structural variant inner
    pub inner: Option<MolecularSequenceStructureVariantInner>,
}

/// Structural variant outer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceStructureVariantOuter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Structural variant outer start
    pub start: Option<i32>,

    /// Structural variant outer end
    pub end: Option<i32>,
}

/// Structural variant inner
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceStructureVariantInner {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Structural variant inner start
    pub start: Option<i32>,

    /// Structural variant inner end
    pub end: Option<i32>,
}

/// Raw data describing a biological sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence {
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

    /// Unique ID for this particular sequence. This is a FHIR-defined id
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// aa | dna | rna
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Base number of coordinate system (0 for 0-based numbering or coordinates, inclusive start, exclusive end, 1 for 1-based numbering, inclusive start, inclusive end)
    #[serde(rename = "coordinateSystem")]
    pub coordinate_system: i32,

    /// Who and/or what this is about
    pub patient: Option<Box<Reference>>,

    /// Specimen used for sequencing
    pub specimen: Option<Box<Reference>>,

    /// The method for sequencing
    pub device: Option<Box<Reference>>,

    /// Who should be responsible for test result
    pub performer: Option<Box<Reference>>,

    /// The number of copies of the sequence of interest. (RNASeq)
    pub quantity: Option<Quantity>,

    /// A sequence used as reference
    #[serde(rename = "referenceSeq")]
    pub reference_seq: Option<MolecularSequenceReferenceSeq>,

    /// Variant in sequence
    pub variant: Option<Vec<MolecularSequenceVariant>>,

    /// Sequence that was observed
    #[serde(rename = "observedSeq")]
    pub observed_seq: Option<String>,

    /// An set of value as quality of sequence
    pub quality: Option<Vec<MolecularSequenceQuality>>,

    /// Average number of reads representing a given nucleotide in the reconstructed sequence
    #[serde(rename = "readCoverage")]
    pub read_coverage: Option<i32>,

    /// External repository which contains detailed report related with observedSeq in this resource
    pub repository: Option<Vec<MolecularSequenceRepository>>,

    /// Pointer to next atomic sequence
    pub pointer: Option<Vec<Box<Reference>>>,

    /// Structural variant
    #[serde(rename = "structureVariant")]
    pub structure_variant: Option<Vec<MolecularSequenceStructureVariant>>,
}
