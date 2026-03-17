//! FHIR R4 MedicinalProduct Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// The product's name, including full name and possibly coded parts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductName {
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

    /// Coding words or phrases of the name
    #[serde(rename = "namePart")]
    pub name_part: Option<Vec<MedicinalProductNameNamePart>>,

    /// Country where the name applies
    #[serde(rename = "countryLanguage")]
    pub country_language: Option<Vec<MedicinalProductNameCountryLanguage>>,
}

/// Coding words or phrases of the name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductNameNamePart {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A fragment of a product name
    pub part: String,

    /// Idenifying type for this part of the name (e.g. strength part)
    #[serde(rename = "type")]
    pub r#type: Coding,
}

/// Country where the name applies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductNameCountryLanguage {
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

/// An operation applied to the product, for manufacturing or adminsitrative purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductManufacturingBusinessOperation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of manufacturing operation
    #[serde(rename = "operationType")]
    pub operation_type: Option<CodeableConcept>,

    /// Regulatory authorization reference number
    #[serde(rename = "authorisationReferenceNumber")]
    pub authorisation_reference_number: Option<Box<Identifier>>,

    /// Regulatory authorization date
    #[serde(rename = "effectiveDate")]
    pub effective_date: Option<String>,

    /// To indicate if this proces is commercially confidential
    #[serde(rename = "confidentialityIndicator")]
    pub confidentiality_indicator: Option<CodeableConcept>,

    /// The manufacturer or establishment associated with the process
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// A regulator which oversees the operation
    pub regulator: Option<Box<Reference>>,
}

/// Indicates if the medicinal product has an orphan designation for the treatment of a rare disease
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductSpecialDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifier for the designation, or procedure number
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The type of special designation, e.g. orphan drug, minor use
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The intended use of the product, e.g. prevention, treatment
    #[serde(rename = "intendedUse")]
    pub intended_use: Option<CodeableConcept>,

    /// Condition for which the medicinal use applies
    pub indication: Option<serde_json::Value>,

    /// For example granted, pending, expired or withdrawn
    pub status: Option<CodeableConcept>,

    /// Date when the designation was granted
    pub date: Option<String>,

    /// Animal species for which this applies
    pub species: Option<CodeableConcept>,
}

/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProduct {
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
    pub domain: Option<Coding>,

    /// The dose form for a single part product, or combined form of a multiple part product
    #[serde(rename = "combinedPharmaceuticalDoseForm")]
    pub combined_pharmaceutical_dose_form: Option<CodeableConcept>,

    /// The legal status of supply of the medicinal product as classified by the regulator
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,

    /// Whether the Medicinal Product is subject to additional monitoring for regulatory reasons
    #[serde(rename = "additionalMonitoringIndicator")]
    pub additional_monitoring_indicator: Option<CodeableConcept>,

    /// Whether the Medicinal Product is subject to special measures for regulatory reasons
    #[serde(rename = "specialMeasures")]
    pub special_measures: Option<Vec<String>>,

    /// If authorised for use in children
    #[serde(rename = "paediatricUseIndicator")]
    pub paediatric_use_indicator: Option<CodeableConcept>,

    /// Allows the product to be classified by various systems
    #[serde(rename = "productClassification")]
    pub product_classification: Option<Vec<CodeableConcept>>,

    /// Marketing status of the medicinal product, in contrast to marketing authorizaton
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,

    /// Pharmaceutical aspects of product
    #[serde(rename = "pharmaceuticalProduct")]
    pub pharmaceutical_product: Option<Vec<Box<Reference>>>,

    /// Package representation for the product
    #[serde(rename = "packagedMedicinalProduct")]
    pub packaged_medicinal_product: Option<Vec<Box<Reference>>>,

    /// Supporting documentation, typically for regulatory submission
    #[serde(rename = "attachedDocument")]
    pub attached_document: Option<Vec<Box<Reference>>>,

    /// A master file for to the medicinal product (e.g. Pharmacovigilance System Master File)
    #[serde(rename = "masterFile")]
    pub master_file: Option<Vec<Box<Reference>>>,

    /// A product specific contact, person (in a role), or an organization
    pub contact: Option<Vec<Box<Reference>>>,

    /// Clinical trials or studies that this product is involved in
    #[serde(rename = "clinicalTrial")]
    pub clinical_trial: Option<Vec<Box<Reference>>>,

    /// The product's name, including full name and possibly coded parts
    pub name: Vec<MedicinalProductName>,

    /// Reference to another product, e.g. for linking authorised to investigational product
    #[serde(rename = "crossReference")]
    pub cross_reference: Option<Vec<Box<Identifier>>>,

    /// An operation applied to the product, for manufacturing or adminsitrative purpose
    #[serde(rename = "manufacturingBusinessOperation")]
    pub manufacturing_business_operation:
        Option<Vec<MedicinalProductManufacturingBusinessOperation>>,

    /// Indicates if the medicinal product has an orphan designation for the treatment of a rare disease
    #[serde(rename = "specialDesignation")]
    pub special_designation: Option<Vec<MedicinalProductSpecialDesignation>>,
}
