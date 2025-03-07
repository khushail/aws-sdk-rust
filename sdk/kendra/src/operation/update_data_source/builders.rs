// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_data_source::_update_data_source_output::UpdateDataSourceOutputBuilder;

pub use crate::operation::update_data_source::_update_data_source_input::UpdateDataSourceInputBuilder;

/// Fluent builder constructing a request to `UpdateDataSource`.
///
/// <p>Updates an existing Amazon Kendra data source connector.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDataSourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_data_source::builders::UpdateDataSourceInputBuilder,
}
impl UpdateDataSourceFluentBuilder {
    /// Creates a new `UpdateDataSource`.
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
            crate::operation::update_data_source::UpdateDataSource,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_data_source::UpdateDataSourceError,
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
        crate::operation::update_data_source::UpdateDataSourceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_data_source::UpdateDataSourceError,
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
        crate::operation::update_data_source::UpdateDataSourceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_data_source::UpdateDataSourceError,
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
            crate::operation::update_data_source::UpdateDataSource,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_data_source::UpdateDataSourceError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier of the data source connector you want to update.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier of the data source connector you want to update.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>A new name for the data source connector.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A new name for the data source connector.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The identifier of the index used with the data source connector.</p>
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_id(input.into());
        self
    }
    /// <p>The identifier of the index used with the data source connector.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_id(input);
        self
    }
    /// <p>Configuration information you want to update for the data source connector.</p>
    pub fn configuration(mut self, input: crate::types::DataSourceConfiguration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>Configuration information you want to update for the data source connector.</p>
    pub fn set_configuration(
        mut self,
        input: ::std::option::Option<crate::types::DataSourceConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your data source. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn vpc_configuration(mut self, input: crate::types::DataSourceVpcConfiguration) -> Self {
        self.inner = self.inner.vpc_configuration(input);
        self
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your data source. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn set_vpc_configuration(
        mut self,
        input: ::std::option::Option<crate::types::DataSourceVpcConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_vpc_configuration(input);
        self
    }
    /// <p>A new description for the data source connector.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A new description for the data source connector.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The sync schedule you want to update for the data source connector.</p>
    pub fn schedule(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schedule(input.into());
        self
    }
    /// <p>The sync schedule you want to update for the data source connector.</p>
    pub fn set_schedule(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schedule(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a role with permission to access the data source and required resources. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM roles for Amazon Kendra</a>.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a role with permission to access the data source and required resources. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM roles for Amazon Kendra</a>.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The code for a language you want to update for the data source connector. This allows you to support a language for all documents when updating the data source. English is supported by default. For more information on supported languages, including their codes, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html">Adding documents in languages other than English</a>.</p>
    pub fn language_code(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.language_code(input.into());
        self
    }
    /// <p>The code for a language you want to update for the data source connector. This allows you to support a language for all documents when updating the data source. English is supported by default. For more information on supported languages, including their codes, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html">Adding documents in languages other than English</a>.</p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_language_code(input);
        self
    }
    /// <p>Configuration information you want to update for altering document metadata and content during the document ingestion process.</p>
    /// <p>For more information on how to create, modify and delete document metadata, or make other content alterations when you ingest documents into Amazon Kendra, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html">Customizing document metadata during the ingestion process</a>.</p>
    pub fn custom_document_enrichment_configuration(
        mut self,
        input: crate::types::CustomDocumentEnrichmentConfiguration,
    ) -> Self {
        self.inner = self.inner.custom_document_enrichment_configuration(input);
        self
    }
    /// <p>Configuration information you want to update for altering document metadata and content during the document ingestion process.</p>
    /// <p>For more information on how to create, modify and delete document metadata, or make other content alterations when you ingest documents into Amazon Kendra, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html">Customizing document metadata during the ingestion process</a>.</p>
    pub fn set_custom_document_enrichment_configuration(
        mut self,
        input: ::std::option::Option<crate::types::CustomDocumentEnrichmentConfiguration>,
    ) -> Self {
        self.inner = self
            .inner
            .set_custom_document_enrichment_configuration(input);
        self
    }
}
