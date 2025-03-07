// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_relational_database_parameters::_update_relational_database_parameters_output::UpdateRelationalDatabaseParametersOutputBuilder;

pub use crate::operation::update_relational_database_parameters::_update_relational_database_parameters_input::UpdateRelationalDatabaseParametersInputBuilder;

/// Fluent builder constructing a request to `UpdateRelationalDatabaseParameters`.
///
/// <p>Allows the update of one or more parameters of a database in Amazon Lightsail.</p>
/// <p>Parameter updates don't cause outages; therefore, their application is not subject to the preferred maintenance window. However, there are two ways in which parameter updates are applied: <code>dynamic</code> or <code>pending-reboot</code>. Parameters marked with a <code>dynamic</code> apply type are applied immediately. Parameters marked with a <code>pending-reboot</code> apply type are applied only after the database is rebooted using the <code>reboot relational database</code> operation.</p>
/// <p>The <code>update relational database parameters</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRelationalDatabaseParametersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_relational_database_parameters::builders::UpdateRelationalDatabaseParametersInputBuilder,
}
impl UpdateRelationalDatabaseParametersFluentBuilder {
    /// Creates a new `UpdateRelationalDatabaseParameters`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParameters, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParametersError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParametersOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParametersError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParametersOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParametersError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParameters, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::update_relational_database_parameters::UpdateRelationalDatabaseParametersError>
    >{
        self.customize_middleware().await
    }
    /// <p>The name of your database for which to update parameters.</p>
    pub fn relational_database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.relational_database_name(input.into());
        self
    }
    /// <p>The name of your database for which to update parameters.</p>
    pub fn set_relational_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_relational_database_name(input);
        self
    }
    /// Appends an item to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>The database parameters to update.</p>
    pub fn parameters(mut self, input: crate::types::RelationalDatabaseParameter) -> Self {
        self.inner = self.inner.parameters(input);
        self
    }
    /// <p>The database parameters to update.</p>
    pub fn set_parameters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RelationalDatabaseParameter>>,
    ) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
}
