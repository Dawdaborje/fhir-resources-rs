//! FHIR R4 SubstanceNucleicAcid Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunit {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Index of linear sequences of nucleic acids in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated a...
    pub subunit: Option<i32>,

    /// Actual nucleotide sequence notation from 5' to 3' end using standard single letter codes. In addition to the base sequence, sugar and type of phosphate or non-phosphate linkage should also be captured
    pub sequence: Option<String>,

    /// The length of the sequence shall be captured
    pub length: Option<i32>,

    /// (TBC)
    #[serde(rename = "sequenceAttachment")]
    pub sequence_attachment: Option<Attachment>,

    /// The nucleotide present at the 5’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter...
    #[serde(rename = "fivePrime")]
    pub five_prime: Option<CodeableConcept>,

    /// The nucleotide present at the 3’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter...
    #[serde(rename = "threePrime")]
    pub three_prime: Option<CodeableConcept>,

    /// The linkages between sugar residues will also be captured
    pub linkage: Option<Vec<SubstanceNucleicAcidSubunitLinkage>>,

    /// 5.3.6.8.1 Sugar ID (Mandatory)
    pub sugar: Option<Vec<SubstanceNucleicAcidSubunitSugar>>,
}

/// The linkages between sugar residues will also be captured
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunitLinkage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The entity that links the sugar residues together should also be captured for nearly all naturally occurring nucleic acid the linkage is a phosphate group. For many synthetic oligonucleotides phosp...
    pub connectivity: Option<String>,

    /// Each linkage will be registered as a fragment and have an ID
    pub identifier: Option<Box<Identifier>>,

    /// Each linkage will be registered as a fragment and have at least one name. A single name shall be assigned to each linkage
    pub name: Option<String>,

    /// Residues shall be captured as described in 5.3.6.8.3
    #[serde(rename = "residueSite")]
    pub residue_site: Option<String>,
}

/// 5.3.6.8.1 Sugar ID (Mandatory)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunitSugar {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The Substance ID of the sugar or sugar-like component that make up the nucleotide
    pub identifier: Option<Box<Identifier>>,

    /// The name of the sugar or sugar-like component that make up the nucleotide
    pub name: Option<String>,

    /// The residues that contain a given sugar will be captured. The order of given residues will be captured in the 5‘-3‘direction consistent with the base sequences listed above
    #[serde(rename = "residueSite")]
    pub residue_site: Option<String>,
}

/// Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcid {
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

    /// The type of the sequence shall be specified based on a controlled vocabulary
    #[serde(rename = "sequenceType")]
    pub sequence_type: Option<CodeableConcept>,

    /// The number of linear sequences of nucleotides linked through phosphodiester bonds shall be described. Subunits would be strands of nucleic acids that are tightly associated typically through Watson...
    #[serde(rename = "numberOfSubunits")]
    pub number_of_subunits: Option<i32>,

    /// The area of hybridisation shall be described if applicable for double stranded RNA or DNA. The number associated with the subunit followed by the number associated to the residue shall be specified...
    #[serde(rename = "areaOfHybridisation")]
    pub area_of_hybridisation: Option<String>,

    /// (TBC)
    #[serde(rename = "oligoNucleotideType")]
    pub oligo_nucleotide_type: Option<CodeableConcept>,

    /// Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times
    pub subunit: Option<Vec<SubstanceNucleicAcidSubunit>>,
}
