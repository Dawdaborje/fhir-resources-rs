//! FHIR R5 MolecularSequence Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A sequence defined relative to another sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelative {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Ways of identifying nucleotides or amino acids within a sequence
    #[serde(rename = "coordinateSystem")]
    pub coordinate_system: CodeableConcept,

    /// Indicates the order in which the sequence should be considered when putting multiple 'relative' elements together
    #[serde(rename = "ordinalPosition")]
    pub ordinal_position: Option<i32>,

    /// Indicates the nucleotide range in the composed sequence when multiple 'relative' elements are used together
    #[serde(rename = "sequenceRange")]
    pub sequence_range: Option<Range>,

    /// A sequence used as starting sequence
    #[serde(rename = "startingSequence")]
    pub starting_sequence: Option<MolecularSequenceRelativeStartingSequence>,

    /// Changes in sequence from the starting sequence
    pub edit: Option<Vec<MolecularSequenceRelativeEdit>>,
}

/// A sequence used as starting sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelativeStartingSequence {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The genome assembly used for starting sequence, e.g. GRCh38
    #[serde(rename = "genomeAssembly")]
    pub genome_assembly: Option<CodeableConcept>,

    /// Chromosome Identifier
    pub chromosome: Option<CodeableConcept>,

    /// The reference sequence that represents the starting sequence
    #[serde(rename = "sequenceCodeableConcept")]
    pub sequence_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "sequenceString")]
    pub sequence_string: Option<String>,

    #[serde(rename = "sequenceReference")]
    pub sequence_reference: Option<Box<Reference>>,

    /// Start position of the window on the starting sequence
    #[serde(rename = "windowStart")]
    pub window_start: Option<i32>,

    /// End position of the window on the starting sequence
    #[serde(rename = "windowEnd")]
    pub window_end: Option<i32>,

    /// sense | antisense
    pub orientation: Option<String>,

    /// watson | crick
    pub strand: Option<String>,
}

/// Changes in sequence from the starting sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelativeEdit {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Start position of the edit on the starting sequence
    pub start: Option<i32>,

    /// End position of the edit on the starting sequence
    pub end: Option<i32>,

    /// Allele that was observed
    #[serde(rename = "replacementSequence")]
    pub replacement_sequence: Option<String>,

    /// Allele in the starting sequence
    #[serde(rename = "replacedSequence")]
    pub replaced_sequence: Option<String>,
}

/// Representation of a molecular sequence.
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

    /// Unique ID for this particular sequence
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// aa | dna | rna
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Subject this sequence is associated too
    pub subject: Option<Box<Reference>>,

    /// What the molecular sequence is about, when it is not about the subject of record
    pub focus: Option<Vec<Box<Reference>>>,

    /// Specimen used for sequencing
    pub specimen: Option<Box<Reference>>,

    /// The method for sequencing
    pub device: Option<Box<Reference>>,

    /// Who should be responsible for test result
    pub performer: Option<Box<Reference>>,

    /// Sequence that was observed
    pub literal: Option<String>,

    /// Embedded file or a link (URL) which contains content to represent the sequence
    pub formatted: Option<Vec<Attachment>>,

    /// A sequence defined relative to another sequence
    pub relative: Option<Vec<MolecularSequenceRelative>>,
}
