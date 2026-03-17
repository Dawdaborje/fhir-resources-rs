//! FHIR R5 InventoryReport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// An inventory listing section (grouped by any of the attributes)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryReportInventoryListing {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Location of the inventory items
    pub location: Option<Box<Reference>>,

    /// The status of the items that are being reported
    #[serde(rename = "itemStatus")]
    pub item_status: Option<CodeableConcept>,

    /// The date and time when the items were counted
    #[serde(rename = "countingDateTime")]
    pub counting_date_time: Option<String>,

    /// The item or items in this listing
    pub item: Option<Vec<InventoryReportInventoryListingItem>>,
}

/// The item or items in this listing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryReportInventoryListingItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The inventory category or classification of the items being reported
    pub category: Option<CodeableConcept>,

    /// The quantity of the item or items being reported
    pub quantity: Quantity,

    /// The code or reference to the item type
    pub item: CodeableReference,
}

/// A report of inventory or stock items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryReport {
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

    /// Business identifier for the report
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// draft | requested | active | entered-in-error
    pub status: String,

    /// snapshot | difference
    #[serde(rename = "countType")]
    pub count_type: String,

    /// addition | subtraction
    #[serde(rename = "operationType")]
    pub operation_type: Option<CodeableConcept>,

    /// The reason for this count - regular count, ad-hoc count, new arrivals, etc
    #[serde(rename = "operationTypeReason")]
    pub operation_type_reason: Option<CodeableConcept>,

    /// When the report has been submitted
    #[serde(rename = "reportedDateTime")]
    pub reported_date_time: String,

    /// Who submits the report
    pub reporter: Option<Box<Reference>>,

    /// The period the report refers to
    #[serde(rename = "reportingPeriod")]
    pub reporting_period: Option<Period>,

    /// An inventory listing section (grouped by any of the attributes)
    #[serde(rename = "inventoryListing")]
    pub inventory_listing: Option<Vec<InventoryReportInventoryListing>>,

    /// A note associated with the InventoryReport
    pub note: Option<Vec<Annotation>>,
}
