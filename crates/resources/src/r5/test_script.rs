//! FHIR R5 TestScript Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// An abstract server representing a client or sender in a message exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptOrigin {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The index of the abstract origin server starting at 1
    pub index: i32,

    /// FHIR-Client | FHIR-SDC-FormFiller
    pub profile: Coding,

    /// The url path of the origin server
    pub url: Option<String>,
}

/// An abstract server representing a destination or receiver in a message exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptDestination {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The index of the abstract destination server starting at 1
    pub index: i32,

    /// FHIR-Server | FHIR-SDC-FormManager | FHIR-SDC-FormReceiver | FHIR-SDC-FormProcessor
    pub profile: Coding,

    /// The url path of the destination server
    pub url: Option<String>,
}

/// Required capability that is assumed to function correctly on the FHIR server being tested
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptMetadata {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Links to the FHIR specification
    pub link: Option<Vec<TestScriptMetadataLink>>,

    /// Capabilities that are assumed to function correctly on the FHIR server being tested
    pub capability: Vec<TestScriptMetadataCapability>,
}

/// Links to the FHIR specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptMetadataLink {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// URL to the specification
    pub url: String,

    /// Short description
    pub description: Option<String>,
}

/// Capabilities that are assumed to function correctly on the FHIR server being tested
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptMetadataCapability {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Are the capabilities required?
    pub required: bool,

    /// Are the capabilities validated?
    pub validated: bool,

    /// The expected capabilities of the server
    pub description: Option<String>,

    /// Which origin server these requirements apply to
    pub origin: Option<Vec<i32>>,

    /// Which server these requirements apply to
    pub destination: Option<i32>,

    /// Links to the FHIR specification
    pub link: Option<Vec<String>>,

    /// Required Capability Statement
    pub capabilities: String,
}

/// Indication of the artifact(s) that are tested by this test case
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptScope {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specific conformance artifact being tested
    pub artifact: String,

    /// required | optional | strict
    pub conformance: Option<CodeableConcept>,

    /// unit | integration | production
    pub phase: Option<CodeableConcept>,
}

/// Fixture in the test script - by reference (uri)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptFixture {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether or not to implicitly create the fixture during setup
    pub autocreate: bool,

    /// Whether or not to implicitly delete the fixture during teardown
    pub autodelete: bool,

    /// Reference of the resource
    pub resource: Option<Box<Reference>>,
}

/// Placeholder for evaluated elements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptVariable {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Descriptive name for this variable
    pub name: String,

    /// Default, hard-coded, or user-defined value for this variable
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,

    /// Natural language description of the variable
    pub description: Option<String>,

    /// The FHIRPath expression against the fixture body
    pub expression: Option<String>,

    /// HTTP header field name for source
    #[serde(rename = "headerField")]
    pub header_field: Option<String>,

    /// Hint help text for default value to enter
    pub hint: Option<String>,

    /// XPath or JSONPath against the fixture body
    pub path: Option<String>,

    /// Fixture Id of source expression or headerField within this variable
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
}

/// A series of required setup operations before tests are executed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A setup operation or assert to perform
    pub action: Vec<TestScriptSetupAction>,
}

/// A setup operation or assert to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// The setup operation to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The operation code type that will be executed
    #[serde(rename = "type")]
    pub r#type: Option<Coding>,

    /// Resource type
    pub resource: Option<String>,

    /// Tracking/logging operation label
    pub label: Option<String>,

    /// Tracking/reporting operation description
    pub description: Option<String>,

    /// Mime type to accept in the payload of the response, with charset etc
    pub accept: Option<String>,

    /// Mime type of the request payload contents, with charset etc
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    /// Server responding to the request
    pub destination: Option<i32>,

    /// Whether or not to send the request url in encoded format
    #[serde(rename = "encodeRequestUrl")]
    pub encode_request_url: bool,

    /// delete | get | options | patch | post | put | head
    pub method: Option<String>,

    /// Server initiating the request
    pub origin: Option<i32>,

    /// Explicitly defined path parameters
    pub params: Option<String>,

    /// Each operation can have one or more header elements
    #[serde(rename = "requestHeader")]
    pub request_header: Option<Vec<TestScriptSetupActionOperationRequestHeader>>,

    /// Fixture Id of mapped request
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,

    /// Fixture Id of mapped response
    #[serde(rename = "responseId")]
    pub response_id: Option<String>,

    /// Fixture Id of body for PUT and POST requests
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,

    /// Id of fixture used for extracting the [id], [type], and [vid] for GET requests
    #[serde(rename = "targetId")]
    pub target_id: Option<String>,

    /// Request URL
    pub url: Option<String>,
}

/// Each operation can have one or more header elements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionOperationRequestHeader {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// HTTP header field name
    pub field: String,

    /// HTTP headerfield value
    pub value: String,
}

/// The assertion to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionAssert {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Tracking/logging assertion label
    pub label: Option<String>,

    /// Tracking/reporting assertion description
    pub description: Option<String>,

    /// response | request
    pub direction: Option<String>,

    /// Id of the source fixture to be evaluated
    #[serde(rename = "compareToSourceId")]
    pub compare_to_source_id: Option<String>,

    /// The FHIRPath expression to evaluate against the source fixture
    #[serde(rename = "compareToSourceExpression")]
    pub compare_to_source_expression: Option<String>,

    /// XPath or JSONPath expression to evaluate against the source fixture
    #[serde(rename = "compareToSourcePath")]
    pub compare_to_source_path: Option<String>,

    /// Mime type to compare against the 'Content-Type' header
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    /// fail | pass | skip | stop
    #[serde(rename = "defaultManualCompletion")]
    pub default_manual_completion: Option<String>,

    /// The FHIRPath expression to be evaluated
    pub expression: Option<String>,

    /// HTTP header field name
    #[serde(rename = "headerField")]
    pub header_field: Option<String>,

    /// Fixture Id of minimum content resource
    #[serde(rename = "minimumId")]
    pub minimum_id: Option<String>,

    /// Perform validation on navigation links?
    #[serde(rename = "navigationLinks")]
    pub navigation_links: Option<bool>,

    /// equals | notEquals | in | notIn | greaterThan | lessThan | empty | notEmpty | contains | notContains | eval | manualEval
    pub operator: Option<String>,

    /// XPath or JSONPath expression
    pub path: Option<String>,

    /// delete | get | options | patch | post | put | head
    #[serde(rename = "requestMethod")]
    pub request_method: Option<String>,

    /// Request URL comparison value
    #[serde(rename = "requestURL")]
    pub request_u_r_l: Option<String>,

    /// Resource type
    pub resource: Option<String>,

    /// continue | switchingProtocols | okay | created | accepted | nonAuthoritativeInformation | noContent | resetContent | partialContent | multipleChoices | movedPermanently | found | seeOther | notModi...
    pub response: Option<String>,

    /// HTTP response code to test
    #[serde(rename = "responseCode")]
    pub response_code: Option<String>,

    /// Fixture Id of source expression or headerField
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,

    /// If this assert fails, will the current test execution stop?
    #[serde(rename = "stopTestOnFail")]
    pub stop_test_on_fail: bool,

    /// Profile Id of validation profile reference
    #[serde(rename = "validateProfileId")]
    pub validate_profile_id: Option<String>,

    /// The value to compare to
    pub value: Option<String>,

    /// Will this assert produce a warning only on error?
    #[serde(rename = "warningOnly")]
    pub warning_only: bool,

    /// Links or references to the testing requirements
    pub requirement: Option<Vec<TestScriptSetupActionAssertRequirement>>,
}

/// Links or references to the testing requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionAssertRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Link or reference to the testing requirement
    pub link: Option<serde_json::Value>,
}

/// A test in this script
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTest {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Tracking/logging name of this test
    pub name: Option<String>,

    /// Tracking/reporting short description of the test
    pub description: Option<String>,

    /// A test operation or assert to perform
    pub action: Vec<TestScriptTestAction>,
}

/// A test operation or assert to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTestAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The setup assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// A series of required clean up steps
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTeardown {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// One or more teardown operations to perform
    pub action: Vec<TestScriptTeardownAction>,
}

/// One or more teardown operations to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTeardownAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The teardown operation to perform
    pub operation: TestScriptSetupActionOperation,
}

/// A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScript {
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

    /// Canonical identifier for this test script, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the test script
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the test script
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this test script (computer friendly)
    pub name: String,

    /// Name for this test script (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the test script
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for test script (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this test script is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// An abstract server representing a client or sender in a message exchange
    pub origin: Option<Vec<TestScriptOrigin>>,

    /// An abstract server representing a destination or receiver in a message exchange
    pub destination: Option<Vec<TestScriptDestination>>,

    /// Required capability that is assumed to function correctly on the FHIR server being tested
    pub metadata: Option<TestScriptMetadata>,

    /// Indication of the artifact(s) that are tested by this test case
    pub scope: Option<Vec<TestScriptScope>>,

    /// Fixture in the test script - by reference (uri)
    pub fixture: Option<Vec<TestScriptFixture>>,

    /// Reference of the validation profile
    pub profile: Option<Vec<String>>,

    /// Placeholder for evaluated elements
    pub variable: Option<Vec<TestScriptVariable>>,

    /// A series of required setup operations before tests are executed
    pub setup: Option<TestScriptSetup>,

    /// A test in this script
    pub test: Option<Vec<TestScriptTest>>,

    /// A series of required clean up steps
    pub teardown: Option<TestScriptTeardown>,
}
