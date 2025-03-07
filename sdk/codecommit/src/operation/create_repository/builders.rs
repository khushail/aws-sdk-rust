// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_repository::_create_repository_output::CreateRepositoryOutputBuilder;

pub use crate::operation::create_repository::_create_repository_input::CreateRepositoryInputBuilder;

/// Fluent builder constructing a request to `CreateRepository`.
///
/// <p>Creates a new, empty repository.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRepositoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_repository::builders::CreateRepositoryInputBuilder,
}
impl CreateRepositoryFluentBuilder {
    /// Creates a new `CreateRepository`.
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
            crate::operation::create_repository::CreateRepository,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_repository::CreateRepositoryError,
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
        crate::operation::create_repository::CreateRepositoryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_repository::CreateRepositoryError,
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
        crate::operation::create_repository::CreateRepositoryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_repository::CreateRepositoryError,
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
            crate::operation::create_repository::CreateRepository,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_repository::CreateRepositoryError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the new repository to be created.</p> <note>
    /// <p>The repository name must be unique across the calling AWS account. Repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. For more information about the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the <i>AWS CodeCommit User Guide</i>. The suffix .git is prohibited.</p>
    /// </note>
    pub fn repository_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.repository_name(input.into());
        self
    }
    /// <p>The name of the new repository to be created.</p> <note>
    /// <p>The repository name must be unique across the calling AWS account. Repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. For more information about the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the <i>AWS CodeCommit User Guide</i>. The suffix .git is prohibited.</p>
    /// </note>
    pub fn set_repository_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_repository_name(input);
        self
    }
    /// <p>A comment or description about the new repository.</p> <note>
    /// <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p>
    /// </note>
    pub fn repository_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.repository_description(input.into());
        self
    }
    /// <p>A comment or description about the new repository.</p> <note>
    /// <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p>
    /// </note>
    pub fn set_repository_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_repository_description(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>One or more tag key-value pairs to use when tagging this repository.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>One or more tag key-value pairs to use when tagging this repository.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
