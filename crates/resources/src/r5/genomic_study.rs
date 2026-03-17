//! FHIR R5 GenomicStudy Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Genomic Analysis Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysis {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifiers for the analysis event
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Type of the methods used in the analysis (e.g., FISH, Karyotyping, MSI)
    #[serde(rename = "methodType")]
    pub method_type: Option<Vec<CodeableConcept>>,

    /// Type of the genomic changes studied in the analysis (e.g., DNA, RNA, or AA change)
    #[serde(rename = "changeType")]
    pub change_type: Option<Vec<CodeableConcept>>,

    /// Genome build that is used in this analysis
    #[serde(rename = "genomeBuild")]
    pub genome_build: Option<CodeableConcept>,

    /// The defined protocol that describes the analysis
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<String>,

    /// The URL pointing to an externally maintained protocol that describes the analysis
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<String>,

    /// Name of the analysis event (human friendly)
    pub title: Option<String>,

    /// What the genomic analysis is about, when it is not about the subject of record
    pub focus: Option<Vec<Box<Reference>>>,

    /// The specimen used in the analysis event
    pub specimen: Option<Vec<Box<Reference>>>,

    /// The date of the analysis event
    pub date: Option<String>,

    /// Any notes capture with the analysis event
    pub note: Option<Vec<Annotation>>,

    /// The protocol that was performed for the analysis event
    #[serde(rename = "protocolPerformed")]
    pub protocol_performed: Option<Box<Reference>>,

    /// The genomic regions to be studied in the analysis (BED file)
    #[serde(rename = "regionsStudied")]
    pub regions_studied: Option<Vec<Box<Reference>>>,

    /// Genomic regions actually called in the analysis event (BED file)
    #[serde(rename = "regionsCalled")]
    pub regions_called: Option<Vec<Box<Reference>>>,

    /// Inputs for the analysis event
    pub input: Option<Vec<GenomicStudyAnalysisInput>>,

    /// Outputs for the analysis event
    pub output: Option<Vec<GenomicStudyAnalysisOutput>>,

    /// Performer for the analysis event
    pub performer: Option<Vec<GenomicStudyAnalysisPerformer>>,

    /// Devices used for the analysis (e.g., instruments, software), with settings and parameters
    pub device: Option<Vec<GenomicStudyAnalysisDevice>>,
}

/// Inputs for the analysis event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisInput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// File containing input data
    pub file: Option<Box<Reference>>,

    /// Type of input data (e.g., BAM, CRAM, or FASTA)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The analysis event or other GenomicStudy that generated this input file
    #[serde(rename = "generatedByIdentifier")]
    pub generated_by_identifier: Option<Box<Identifier>>,

    #[serde(rename = "generatedByReference")]
    pub generated_by_reference: Option<Box<Reference>>,
}

/// Outputs for the analysis event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisOutput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// File containing output data
    pub file: Option<Box<Reference>>,

    /// Type of output data (e.g., VCF, MAF, or BAM)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
}

/// Performer for the analysis event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The organization, healthcare professional, or others who participated in performing this analysis
    pub actor: Option<Box<Reference>>,

    /// Role of the actor for this analysis
    pub role: Option<CodeableConcept>,
}

/// Devices used for the analysis (e.g., instruments, software), with settings and parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisDevice {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Device used for the analysis
    pub device: Option<Box<Reference>>,

    /// Specific function for the device used for the analysis
    pub function: Option<CodeableConcept>,
}

/// A set of analyses performed to analyze and generate genomic data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudy {
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

    /// Identifiers for this genomic study
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// registered | available | cancelled | entered-in-error | unknown
    pub status: String,

    /// The type of the study (e.g., Familial variant segregation, Functional variation detection, or Gene expression profiling)
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// The primary subject of the genomic study
    pub subject: Box<Reference>,

    /// The healthcare event with which this genomics study is associated
    pub encounter: Option<Box<Reference>>,

    /// When the genomic study was started
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,

    /// Event resources that the genomic study is based on
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Healthcare professional who requested or referred the genomic study
    pub referrer: Option<Box<Reference>>,

    /// Healthcare professionals who interpreted the genomic study
    pub interpreter: Option<Vec<Box<Reference>>>,

    /// Why the genomic study was performed
    pub reason: Option<Vec<CodeableReference>>,

    /// The defined protocol that describes the study
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<String>,

    /// The URL pointing to an externally maintained protocol that describes the study
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<String>,

    /// Comments related to the genomic study
    pub note: Option<Vec<Annotation>>,

    /// Description of the genomic study
    pub description: Option<String>,

    /// Genomic Analysis Event
    pub analysis: Option<Vec<GenomicStudyAnalysis>>,
}
