//! FHIR R4B TestReport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// A participant in the test execution, either the execution engine, a client, or a server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// test-engine | client | server
    #[serde(rename = "type")]
    pub r#type: String,

    /// The uri of the participant. An absolute URL is preferred
    pub uri: String,

    /// The display name of the participant
    pub display: Option<String>,
}

/// The results of the series of required setup operations before the tests were executed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A setup operation or assert that was executed
    pub action: Vec<TestReportSetupAction>,
}

/// A setup operation or assert that was executed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetupAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The operation to perform
    pub operation: Option<TestReportSetupActionOperation>,

    /// The assertion to perform
    pub assert: Option<TestReportSetupActionAssert>,
}

/// The operation to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetupActionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// pass | skip | fail | warning | error
    pub result: String,

    /// A message associated with the result
    pub message: Option<String>,

    /// A link to further details on the result
    pub detail: Option<String>,
}

/// The assertion to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetupActionAssert {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// pass | skip | fail | warning | error
    pub result: String,

    /// A message associated with the result
    pub message: Option<String>,

    /// A link to further details on the result
    pub detail: Option<String>,
}

/// A test executed from the test script
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTest {
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

    /// A test operation or assert that was performed
    pub action: Vec<TestReportTestAction>,
}

/// A test operation or assert that was performed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTestAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The operation performed
    pub operation: Option<TestReportSetupActionOperation>,

    /// The assertion performed
    pub assert: Option<TestReportSetupActionAssert>,
}

/// The results of running the series of required clean up steps
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTeardown {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// One or more teardown operations performed
    pub action: Vec<TestReportTeardownAction>,
}

/// One or more teardown operations performed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTeardownAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The teardown operation performed
    pub operation: TestReportSetupActionOperation,
}

/// A summary of information based on the results of executing a TestScript.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReport {
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

    /// External identifier
    pub identifier: Option<Box<Identifier>>,

    /// Informal name of the executed TestScript
    pub name: Option<String>,

    /// completed | in-progress | waiting | stopped | entered-in-error
    pub status: String,

    /// Reference to the version-specific TestScript that was executed to produce this TestReport
    #[serde(rename = "testScript")]
    pub test_script: Box<Reference>,

    /// pass | fail | pending
    pub result: String,

    /// The final score (percentage of tests passed) resulting from the execution of the TestScript
    pub score: Option<f64>,

    /// Name of the tester producing this report (Organization or individual)
    pub tester: Option<String>,

    /// When the TestScript was executed and this TestReport was generated
    pub issued: Option<String>,

    /// A participant in the test execution, either the execution engine, a client, or a server
    pub participant: Option<Vec<TestReportParticipant>>,

    /// The results of the series of required setup operations before the tests were executed
    pub setup: Option<TestReportSetup>,

    /// A test executed from the test script
    pub test: Option<Vec<TestReportTest>>,

    /// The results of running the series of required clean up steps
    pub teardown: Option<TestReportTeardown>,
}
