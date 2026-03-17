//! FHIR R5 SubstanceProtein Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// This subclause refers to the description of each subunit constituting the SubstanceProtein. A subunit is a linear sequence of amino acids linked through peptide bonds. The Subunit information shall...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceProteinSubunit {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Index of primary sequences of amino acids linked through peptide bonds in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical ...
    pub subunit: Option<i32>,

    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowerca...
    pub sequence: Option<String>,

    /// Length of linear sequences of amino acids contained in the subunit
    pub length: Option<i32>,

    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowerca...
    #[serde(rename = "sequenceAttachment")]
    pub sequence_attachment: Option<Attachment>,

    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    #[serde(rename = "nTerminalModificationId")]
    pub n_terminal_modification_id: Option<Box<Identifier>>,

    /// The name of the fragment modified at the N-terminal of the SubstanceProtein shall be specified
    #[serde(rename = "nTerminalModification")]
    pub n_terminal_modification: Option<String>,

    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    #[serde(rename = "cTerminalModificationId")]
    pub c_terminal_modification_id: Option<Box<Identifier>>,

    /// The modification at the C-terminal shall be specified
    #[serde(rename = "cTerminalModification")]
    pub c_terminal_modification: Option<String>,
}

/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence, or a combination of subunits that are either covalently linked or have a defined invariant stoichiometric relationshi...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceProtein {
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

    /// The SubstanceProtein descriptive elements will only be used when a complete or partial amino acid sequence is available or derivable from a nucleic acid sequence
    #[serde(rename = "sequenceType")]
    pub sequence_type: Option<CodeableConcept>,

    /// Number of linear sequences of amino acids linked through peptide bonds. The number of subunits constituting the SubstanceProtein shall be described. It is possible that the number of subunits can b...
    #[serde(rename = "numberOfSubunits")]
    pub number_of_subunits: Option<i32>,

    /// The disulphide bond between two cysteine residues either on the same subunit or on two different subunits shall be described. The position of the disulfide bonds in the SubstanceProtein shall be li...
    #[serde(rename = "disulfideLinkage")]
    pub disulfide_linkage: Option<Vec<String>>,

    /// This subclause refers to the description of each subunit constituting the SubstanceProtein. A subunit is a linear sequence of amino acids linked through peptide bonds. The Subunit information shall...
    pub subunit: Option<Vec<SubstanceProteinSubunit>>,
}
