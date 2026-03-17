//! FHIR R5 MedicationKnowledge Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Associated or related medication information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Category of medicationKnowledge
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Associated documentation about the associated medication knowledge
    pub reference: Vec<Box<Reference>>,
}

/// Associated documentation about the medication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeMonograph {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The category of medication document
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Associated documentation about the medication
    pub source: Option<Box<Reference>>,
}

/// The pricing of the medication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeCost {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The date range for which the cost is effective
    #[serde(rename = "effectiveDate")]
    pub effective_date: Option<Vec<Period>>,

    /// The category of the cost information
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The source or owner for the price information
    pub source: Option<String>,

    /// The price or category of the cost of the medication
    pub cost: serde_json::Value,
}

/// Program under which a medication is reviewed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeMonitoringProgram {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of program under which the medication is monitored
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Name of the reviewing program
    pub name: Option<String>,
}

/// Guidelines or protocols for administration of the medication for an indication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Indication for use that applies to the specific administration guideline
    pub indication: Option<Vec<CodeableReference>>,

    /// Guidelines for dosage of the medication
    #[serde(rename = "dosingGuideline")]
    pub dosing_guideline: Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuideline>>,
}

/// Guidelines for dosage of the medication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Intention of the treatment
    #[serde(rename = "treatmentIntent")]
    pub treatment_intent: Option<CodeableConcept>,

    /// Dosage for the medication for the specific guidelines
    pub dosage: Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage>>,

    /// Type of treatment the guideline applies to
    #[serde(rename = "administrationTreatment")]
    pub administration_treatment: Option<CodeableConcept>,

    /// Characteristics of the patient that are relevant to the administration guidelines
    #[serde(rename = "patientCharacteristic")]
    pub patient_characteristic:
        Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic>>,
}

/// Dosage for the medication for the specific guidelines
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Category of dosage for a medication
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Dosage for the medication for the specific guidelines
    pub dosage: Vec<Dosage>,
}

/// Characteristics of the patient that are relevant to the administration guidelines
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Categorization of specific characteristic that is relevant to the administration guideline
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The specific characteristic
    pub value: Option<serde_json::Value>,
}

/// Categorization of the medication within a formulary or classification system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeMedicineClassification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of category for the medication (for example, therapeutic classification, therapeutic sub-classification)
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The source of the classification
    pub source: Option<serde_json::Value>,

    /// Specific category assigned to the medication
    pub classification: Option<Vec<CodeableConcept>>,
}

/// Details about packaged medications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgePackaging {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Cost of the packaged medication
    pub cost: Option<Vec<MedicationKnowledgeCost>>,

    /// The packaged medication that is being priced
    #[serde(rename = "packagedProduct")]
    pub packaged_product: Option<Box<Reference>>,
}

/// How the medication should be stored
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeStorageGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to additional information
    pub reference: Option<String>,

    /// Additional storage notes
    pub note: Option<Vec<Annotation>>,

    /// Duration remains stable
    #[serde(rename = "stabilityDuration")]
    pub stability_duration: Option<Duration>,

    /// Setting or value of environment for adequate storage
    #[serde(rename = "environmentalSetting")]
    pub environmental_setting: Option<Vec<MedicationKnowledgeStorageGuidelineEnvironmentalSetting>>,
}

/// Setting or value of environment for adequate storage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeStorageGuidelineEnvironmentalSetting {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Categorization of the setting
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Value of the setting
    pub value: serde_json::Value,
}

/// Regulatory information about a medication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRegulatory {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Specifies the authority of the regulation
    #[serde(rename = "regulatoryAuthority")]
    pub regulatory_authority: Box<Reference>,

    /// Specifies if changes are allowed when dispensing a medication from a regulatory perspective
    pub substitution: Option<Vec<MedicationKnowledgeRegulatorySubstitution>>,

    /// Specifies the schedule of a medication in jurisdiction
    pub schedule: Option<Vec<CodeableConcept>>,

    /// The maximum number of units of the medication that can be dispensed in a period
    #[serde(rename = "maxDispense")]
    pub max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
}

/// Specifies if changes are allowed when dispensing a medication from a regulatory perspective
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRegulatorySubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Specifies the type of substitution allowed
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Specifies if regulation allows for changes in the medication when dispensing
    pub allowed: bool,
}

/// The maximum number of units of the medication that can be dispensed in a period
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The maximum number of units of the medication that can be dispensed
    pub quantity: Quantity,

    /// The period that applies to the maximum number of units
    pub period: Option<Duration>,
}

/// Minimal definition information about the medication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeDefinitional {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Definitional resources that provide more information about this medication
    pub definition: Option<Vec<Box<Reference>>>,

    /// powder | tablets | capsule +
    #[serde(rename = "doseForm")]
    pub dose_form: Option<CodeableConcept>,

    /// The intended or approved route of administration
    #[serde(rename = "intendedRoute")]
    pub intended_route: Option<Vec<CodeableConcept>>,

    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationKnowledgeDefinitionalIngredient>>,

    /// Specifies descriptive properties of the medicine
    #[serde(rename = "drugCharacteristic")]
    pub drug_characteristic: Option<Vec<MedicationKnowledgeDefinitionalDrugCharacteristic>>,
}

/// Active or inactive ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeDefinitionalIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Substances contained in the medication
    pub item: CodeableReference,

    /// A code that defines the type of ingredient, active, base, etc
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Quantity of ingredient present
    pub strength: Option<serde_json::Value>,
}

/// Specifies descriptive properties of the medicine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeDefinitionalDrugCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code specifying the type of characteristic of medication
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Description of the characteristic
    pub value: Option<serde_json::Value>,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledge {
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

    /// Business identifier for this medication
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Code that identifies this medication
    pub code: Option<CodeableConcept>,

    /// active | entered-in-error | inactive
    pub status: Option<String>,

    /// Creator or owner of the knowledge or information about the medication
    pub author: Option<Box<Reference>>,

    /// Codes that identify the different jurisdictions for which the information of this resource was created
    #[serde(rename = "intendedJurisdiction")]
    pub intended_jurisdiction: Option<Vec<CodeableConcept>>,

    /// A name associated with the medication being described
    pub name: Option<Vec<String>>,

    /// Associated or related medication information
    #[serde(rename = "relatedMedicationKnowledge")]
    pub related_medication_knowledge: Option<Vec<MedicationKnowledgeRelatedMedicationKnowledge>>,

    /// The set of medication resources that are associated with this medication
    #[serde(rename = "associatedMedication")]
    pub associated_medication: Option<Vec<Box<Reference>>>,

    /// Category of the medication or product
    #[serde(rename = "productType")]
    pub product_type: Option<Vec<CodeableConcept>>,

    /// Associated documentation about the medication
    pub monograph: Option<Vec<MedicationKnowledgeMonograph>>,

    /// The instructions for preparing the medication
    #[serde(rename = "preparationInstruction")]
    pub preparation_instruction: Option<String>,

    /// The pricing of the medication
    pub cost: Option<Vec<MedicationKnowledgeCost>>,

    /// Program under which a medication is reviewed
    #[serde(rename = "monitoringProgram")]
    pub monitoring_program: Option<Vec<MedicationKnowledgeMonitoringProgram>>,

    /// Guidelines or protocols for administration of the medication for an indication
    #[serde(rename = "indicationGuideline")]
    pub indication_guideline: Option<Vec<MedicationKnowledgeIndicationGuideline>>,

    /// Categorization of the medication within a formulary or classification system
    #[serde(rename = "medicineClassification")]
    pub medicine_classification: Option<Vec<MedicationKnowledgeMedicineClassification>>,

    /// Details about packaged medications
    pub packaging: Option<Vec<MedicationKnowledgePackaging>>,

    /// Potential clinical issue with or between medication(s)
    #[serde(rename = "clinicalUseIssue")]
    pub clinical_use_issue: Option<Vec<Box<Reference>>>,

    /// How the medication should be stored
    #[serde(rename = "storageGuideline")]
    pub storage_guideline: Option<Vec<MedicationKnowledgeStorageGuideline>>,

    /// Regulatory information about a medication
    pub regulatory: Option<Vec<MedicationKnowledgeRegulatory>>,

    /// Minimal definition information about the medication
    pub definitional: Option<MedicationKnowledgeDefinitional>,
}
