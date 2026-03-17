//! FHIR R4B MedicationKnowledge Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
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

/// Active or inactive ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Medication(s) or substance(s) contained in the medication
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,

    #[serde(rename = "itemReference")]
    pub item_reference: Box<Reference>,

    /// Active ingredient indicator
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,

    /// Quantity of ingredient present
    pub strength: Option<Ratio>,
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

    /// The category of the cost information
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The source or owner for the price information
    pub source: Option<String>,

    /// The price of the medication
    pub cost: Money,
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

/// Guidelines for administration of the medication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeAdministrationGuidelines {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Dosage for the medication for the specific guidelines
    pub dosage: Option<Vec<MedicationKnowledgeAdministrationGuidelinesDosage>>,

    /// Indication for use that apply to the specific administration guidelines
    #[serde(rename = "indicationCodeableConcept")]
    pub indication_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "indicationReference")]
    pub indication_reference: Option<Box<Reference>>,

    /// Characteristics of the patient that are relevant to the administration guidelines
    #[serde(rename = "patientCharacteristics")]
    pub patient_characteristics:
        Option<Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>>,
}

/// Dosage for the medication for the specific guidelines
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeAdministrationGuidelinesDosage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of dosage
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Dosage for the medication for the specific guidelines
    pub dosage: Vec<Dosage>,
}

/// Characteristics of the patient that are relevant to the administration guidelines
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Specific characteristic that is relevant to the administration guideline
    #[serde(rename = "characteristicCodeableConcept")]
    pub characteristic_codeable_concept: CodeableConcept,

    #[serde(rename = "characteristicQuantity")]
    pub characteristic_quantity: Quantity,

    /// The specific characteristic
    pub value: Option<Vec<String>>,
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

    /// A code that defines the specific type of packaging that the medication can be found in
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The number of product units the package would contain if fully loaded
    pub quantity: Option<Quantity>,
}

/// Specifies descriptive properties of the medicine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeDrugCharacteristic {
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
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueString")]
    pub value_string: Option<String>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueBase64Binary")]
    pub value_base64binary: Option<String>,
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
    pub schedule: Option<Vec<MedicationKnowledgeRegulatorySchedule>>,

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

/// Specifies the schedule of a medication in jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRegulatorySchedule {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Specifies the specific drug schedule
    pub schedule: CodeableConcept,
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

/// The time course of drug absorption, distribution, metabolism and excretion of a medication from the body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeKinetics {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The drug concentration measured at certain discrete points in time
    #[serde(rename = "areaUnderCurve")]
    pub area_under_curve: Option<Vec<Quantity>>,

    /// The median lethal dose of a drug
    #[serde(rename = "lethalDose50")]
    pub lethal_dose50: Option<Vec<Quantity>>,

    /// Time required for concentration in the body to decrease by half
    #[serde(rename = "halfLifePeriod")]
    pub half_life_period: Option<Duration>,
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

    /// Code that identifies this medication
    pub code: Option<CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: Option<String>,

    /// Manufacturer of the item
    pub manufacturer: Option<Box<Reference>>,

    /// powder | tablets | capsule +
    #[serde(rename = "doseForm")]
    pub dose_form: Option<CodeableConcept>,

    /// Amount of drug in package
    pub amount: Option<Quantity>,

    /// Additional names for a medication
    pub synonym: Option<Vec<String>>,

    /// Associated or related medication information
    #[serde(rename = "relatedMedicationKnowledge")]
    pub related_medication_knowledge: Option<Vec<MedicationKnowledgeRelatedMedicationKnowledge>>,

    /// A medication resource that is associated with this medication
    #[serde(rename = "associatedMedication")]
    pub associated_medication: Option<Vec<Box<Reference>>>,

    /// Category of the medication or product
    #[serde(rename = "productType")]
    pub product_type: Option<Vec<CodeableConcept>>,

    /// Associated documentation about the medication
    pub monograph: Option<Vec<MedicationKnowledgeMonograph>>,

    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationKnowledgeIngredient>>,

    /// The instructions for preparing the medication
    #[serde(rename = "preparationInstruction")]
    pub preparation_instruction: Option<String>,

    /// The intended or approved route of administration
    #[serde(rename = "intendedRoute")]
    pub intended_route: Option<Vec<CodeableConcept>>,

    /// The pricing of the medication
    pub cost: Option<Vec<MedicationKnowledgeCost>>,

    /// Program under which a medication is reviewed
    #[serde(rename = "monitoringProgram")]
    pub monitoring_program: Option<Vec<MedicationKnowledgeMonitoringProgram>>,

    /// Guidelines for administration of the medication
    #[serde(rename = "administrationGuidelines")]
    pub administration_guidelines: Option<Vec<MedicationKnowledgeAdministrationGuidelines>>,

    /// Categorization of the medication within a formulary or classification system
    #[serde(rename = "medicineClassification")]
    pub medicine_classification: Option<Vec<MedicationKnowledgeMedicineClassification>>,

    /// Details about packaged medications
    pub packaging: Option<MedicationKnowledgePackaging>,

    /// Specifies descriptive properties of the medicine
    #[serde(rename = "drugCharacteristic")]
    pub drug_characteristic: Option<Vec<MedicationKnowledgeDrugCharacteristic>>,

    /// Potential clinical issue with or between medication(s)
    pub contraindication: Option<Vec<Box<Reference>>>,

    /// Regulatory information about a medication
    pub regulatory: Option<Vec<MedicationKnowledgeRegulatory>>,

    /// The time course of drug absorption, distribution, metabolism and excretion of a medication from the body
    pub kinetics: Option<Vec<MedicationKnowledgeKinetics>>,
}
