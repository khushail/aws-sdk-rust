// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_assessment_report::_create_assessment_report_output::CreateAssessmentReportOutputBuilder;

pub use crate::operation::create_assessment_report::_create_assessment_report_input::CreateAssessmentReportInputBuilder;

/// Fluent builder constructing a request to `CreateAssessmentReport`.
///
/// <p> Creates an assessment report for the specified assessment. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAssessmentReportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_assessment_report::builders::CreateAssessmentReportInputBuilder,
}
impl CreateAssessmentReportFluentBuilder {
    /// Creates a new `CreateAssessmentReport`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_assessment_report::CreateAssessmentReport,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_assessment_report::CreateAssessmentReportError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_assessment_report::CreateAssessmentReportOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_assessment_report::CreateAssessmentReportError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_assessment_report::CreateAssessmentReportOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_assessment_report::CreateAssessmentReportError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_assessment_report::CreateAssessmentReport,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_assessment_report::CreateAssessmentReportError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p> The name of the new assessment report. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p> The name of the new assessment report. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p> The description of the assessment report. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p> The description of the assessment report. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p> The identifier for the assessment. </p>
    pub fn assessment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.assessment_id(input.into());
        self
    }
    /// <p> The identifier for the assessment. </p>
    pub fn set_assessment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_assessment_id(input);
        self
    }
    /// <p>A SQL statement that represents an evidence finder query.</p>
    /// <p>Provide this parameter when you want to generate an assessment report from the results of an evidence finder search query. When you use this parameter, Audit Manager generates a one-time report using only the evidence from the query output. This report does not include any assessment evidence that was manually <a href="https://docs.aws.amazon.com/audit-manager/latest/userguide/generate-assessment-report.html#generate-assessment-report-include-evidence">added to a report using the console</a>, or <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_BatchAssociateAssessmentReportEvidence.html">associated with a report using the API</a>. </p>
    /// <p>To use this parameter, the <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_EvidenceFinderEnablement.html#auditmanager-Type-EvidenceFinderEnablement-enablementStatus">enablementStatus</a> of evidence finder must be <code>ENABLED</code>. </p>
    /// <p> For examples and help resolving <code>queryStatement</code> validation exceptions, see <a href="https://docs.aws.amazon.com/audit-manager/latest/userguide/evidence-finder-issues.html#querystatement-exceptions">Troubleshooting evidence finder issues</a> in the <i>Audit Manager User Guide.</i> </p>
    pub fn query_statement(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.query_statement(input.into());
        self
    }
    /// <p>A SQL statement that represents an evidence finder query.</p>
    /// <p>Provide this parameter when you want to generate an assessment report from the results of an evidence finder search query. When you use this parameter, Audit Manager generates a one-time report using only the evidence from the query output. This report does not include any assessment evidence that was manually <a href="https://docs.aws.amazon.com/audit-manager/latest/userguide/generate-assessment-report.html#generate-assessment-report-include-evidence">added to a report using the console</a>, or <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_BatchAssociateAssessmentReportEvidence.html">associated with a report using the API</a>. </p>
    /// <p>To use this parameter, the <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_EvidenceFinderEnablement.html#auditmanager-Type-EvidenceFinderEnablement-enablementStatus">enablementStatus</a> of evidence finder must be <code>ENABLED</code>. </p>
    /// <p> For examples and help resolving <code>queryStatement</code> validation exceptions, see <a href="https://docs.aws.amazon.com/audit-manager/latest/userguide/evidence-finder-issues.html#querystatement-exceptions">Troubleshooting evidence finder issues</a> in the <i>Audit Manager User Guide.</i> </p>
    pub fn set_query_statement(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_query_statement(input);
        self
    }
}
