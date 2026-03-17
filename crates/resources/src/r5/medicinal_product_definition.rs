//! FHIR R5 MedicinalProductDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A product specific contact, person (in a role), or an organization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionContact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Allows the contact to be classified, for example QPPV, Pharmacovigilance Enquiry Information
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// A product specific contact, person (in a role), or an organization
    pub contact: Box<Reference>,
}

/// The product's name, including full name and possibly coded parts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The full product name
    #[serde(rename = "productName")]
    pub product_name: String,

    /// Type of product name, such as rINN, BAN, Proprietary, Non-Proprietary
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Coding words or phrases of the name
    pub part: Option<Vec<MedicinalProductDefinitionNamePart>>,

    /// Country and jurisdiction where the name applies
    pub usage: Option<Vec<MedicinalProductDefinitionNameUsage>>,
}

/// Coding words or phrases of the name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionNamePart {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A fragment of a product name
    pub part: String,

    /// Identifying type for this part of the name (e.g. strength part)
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,
}

/// Country and jurisdiction where the name applies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionNameUsage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Country code for where this name applies
    pub country: CodeableConcept,

    /// Jurisdiction code for where this name applies
    pub jurisdiction: Option<CodeableConcept>,

    /// Language code for this name
    pub language: CodeableConcept,
}

/// Reference to another product, e.g. for linking authorised to investigational product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionCrossReference {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to another product, e.g. for linking authorised to investigational product
    pub product: CodeableReference,

    /// The type of relationship, for instance branded to generic or virtual to actual product
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
}

/// A manufacturing or administrative process for the medicinal product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of manufacturing operation e.g. manufacturing itself, re-packaging
    #[serde(rename = "type")]
    pub r#type: Option<CodeableReference>,

    /// Date range of applicability
    #[serde(rename = "effectiveDate")]
    pub effective_date: Option<Period>,

    /// The organization responsible for the particular process, e.g. the manufacturer or importer
    pub organization: Option<Vec<Box<Reference>>>,

    /// Specifies whether this process is considered proprietary or confidential
    #[serde(rename = "confidentialityIndicator")]
    pub confidentiality_indicator: Option<CodeableConcept>,
}

/// Key product features such as "sugar free", "modified release"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A code expressing the type of characteristic
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// A value for the characteristic
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<String>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueInteger")]
    pub value_integer: Option<i32>,

    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
}

/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinition {
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

    /// Business identifier for this product. Could be an MPID
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Regulatory type, e.g. Investigational or Authorized
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// If this medicine applies to human or veterinary uses
    pub domain: Option<CodeableConcept>,

    /// A business identifier relating to a specific version of the product
    pub version: Option<String>,

    /// The status within the lifecycle of this product record
    pub status: Option<CodeableConcept>,

    /// The date at which the given status became applicable
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// General description of this product
    pub description: Option<String>,

    /// The dose form for a single part product, or combined form of a multiple part product
    #[serde(rename = "combinedPharmaceuticalDoseForm")]
    pub combined_pharmaceutical_dose_form: Option<CodeableConcept>,

    /// The path by which the product is taken into or makes contact with the body
    pub route: Option<Vec<CodeableConcept>>,

    /// Description of indication(s) for this product, used when structured indications are not required
    pub indication: Option<String>,

    /// The legal status of supply of the medicinal product as classified by the regulator
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,

    /// Whether the Medicinal Product is subject to additional monitoring for regulatory reasons
    #[serde(rename = "additionalMonitoringIndicator")]
    pub additional_monitoring_indicator: Option<CodeableConcept>,

    /// Whether the Medicinal Product is subject to special measures for regulatory reasons
    #[serde(rename = "specialMeasures")]
    pub special_measures: Option<Vec<CodeableConcept>>,

    /// If authorised for use in children
    #[serde(rename = "pediatricUseIndicator")]
    pub pediatric_use_indicator: Option<CodeableConcept>,

    /// Allows the product to be classified by various systems
    pub classification: Option<Vec<CodeableConcept>>,

    /// Marketing status of the medicinal product, in contrast to marketing authorization
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,

    /// Package type for the product
    #[serde(rename = "packagedMedicinalProduct")]
    pub packaged_medicinal_product: Option<Vec<CodeableConcept>>,

    /// Types of medicinal manufactured items and/or devices that this product consists of, such as tablets, capsule, or syringes
    #[serde(rename = "comprisedOf")]
    pub comprised_of: Option<Vec<Box<Reference>>>,

    /// The ingredients of this medicinal product - when not detailed in other resources
    pub ingredient: Option<Vec<CodeableConcept>>,

    /// Any component of the drug product which is not the chemical entity defined as the drug substance, or an excipient in the drug product
    pub impurity: Option<Vec<CodeableReference>>,

    /// Additional documentation about the medicinal product
    #[serde(rename = "attachedDocument")]
    pub attached_document: Option<Vec<Box<Reference>>>,

    /// A master file for the medicinal product (e.g. Pharmacovigilance System Master File)
    #[serde(rename = "masterFile")]
    pub master_file: Option<Vec<Box<Reference>>>,

    /// A product specific contact, person (in a role), or an organization
    pub contact: Option<Vec<MedicinalProductDefinitionContact>>,

    /// Clinical trials or studies that this product is involved in
    #[serde(rename = "clinicalTrial")]
    pub clinical_trial: Option<Vec<Box<Reference>>>,

    /// A code that this product is known by, within some formal terminology
    pub code: Option<Vec<Coding>>,

    /// The product's name, including full name and possibly coded parts
    pub name: Vec<MedicinalProductDefinitionName>,

    /// Reference to another product, e.g. for linking authorised to investigational product
    #[serde(rename = "crossReference")]
    pub cross_reference: Option<Vec<MedicinalProductDefinitionCrossReference>>,

    /// A manufacturing or administrative process for the medicinal product
    pub operation: Option<Vec<MedicinalProductDefinitionOperation>>,

    /// Key product features such as "sugar free", "modified release"
    pub characteristic: Option<Vec<MedicinalProductDefinitionCharacteristic>>,
}
