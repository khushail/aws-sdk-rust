// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_domain_entry::_delete_domain_entry_output::DeleteDomainEntryOutputBuilder;

pub use crate::operation::delete_domain_entry::_delete_domain_entry_input::DeleteDomainEntryInputBuilder;

/// Fluent builder constructing a request to `DeleteDomainEntry`.
///
/// <p>Deletes a specific domain entry.</p>
/// <p>The <code>delete domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDomainEntryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_domain_entry::builders::DeleteDomainEntryInputBuilder,
}
impl DeleteDomainEntryFluentBuilder {
    /// Creates a new `DeleteDomainEntry`.
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
            crate::operation::delete_domain_entry::DeleteDomainEntry,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_domain_entry::DeleteDomainEntryError,
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
        crate::operation::delete_domain_entry::DeleteDomainEntryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_domain_entry::DeleteDomainEntryError,
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
        crate::operation::delete_domain_entry::DeleteDomainEntryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_domain_entry::DeleteDomainEntryError,
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
            crate::operation::delete_domain_entry::DeleteDomainEntry,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_domain_entry::DeleteDomainEntryError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the domain entry to delete.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain entry to delete.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>An array of key-value pairs containing information about your domain entries.</p>
    pub fn domain_entry(mut self, input: crate::types::DomainEntry) -> Self {
        self.inner = self.inner.domain_entry(input);
        self
    }
    /// <p>An array of key-value pairs containing information about your domain entries.</p>
    pub fn set_domain_entry(
        mut self,
        input: ::std::option::Option<crate::types::DomainEntry>,
    ) -> Self {
        self.inner = self.inner.set_domain_entry(input);
        self
    }
}
