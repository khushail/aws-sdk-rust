// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_assessment_report_evidence_folder::_disassociate_assessment_report_evidence_folder_output::DisassociateAssessmentReportEvidenceFolderOutputBuilder;

pub use crate::operation::disassociate_assessment_report_evidence_folder::_disassociate_assessment_report_evidence_folder_input::DisassociateAssessmentReportEvidenceFolderInputBuilder;

/// Fluent builder constructing a request to `DisassociateAssessmentReportEvidenceFolder`.
///
/// <p> Disassociates an evidence folder from the specified assessment report in Audit Manager. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateAssessmentReportEvidenceFolderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::disassociate_assessment_report_evidence_folder::builders::DisassociateAssessmentReportEvidenceFolderInputBuilder,
}
impl DisassociateAssessmentReportEvidenceFolderFluentBuilder {
    /// Creates a new `DisassociateAssessmentReportEvidenceFolder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolder, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolderError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolderOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolderError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolderOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolderError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolder, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::disassociate_assessment_report_evidence_folder::DisassociateAssessmentReportEvidenceFolderError>
    >{
        self.customize_middleware().await
    }
    /// <p> The unique identifier for the assessment. </p>
    pub fn assessment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.assessment_id(input.into());
        self
    }
    /// <p> The unique identifier for the assessment. </p>
    pub fn set_assessment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_assessment_id(input);
        self
    }
    /// <p> The unique identifier for the folder that the evidence is stored in. </p>
    pub fn evidence_folder_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.evidence_folder_id(input.into());
        self
    }
    /// <p> The unique identifier for the folder that the evidence is stored in. </p>
    pub fn set_evidence_folder_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_evidence_folder_id(input);
        self
    }
}
