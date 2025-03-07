// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::tag_open_id_connect_provider::_tag_open_id_connect_provider_output::TagOpenIdConnectProviderOutputBuilder;

pub use crate::operation::tag_open_id_connect_provider::_tag_open_id_connect_provider_input::TagOpenIdConnectProviderInputBuilder;

/// Fluent builder constructing a request to `TagOpenIDConnectProvider`.
///
/// <p>Adds one or more tags to an OpenID Connect (OIDC)-compatible identity provider. For more information about these providers, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_oidc.html">About web identity federation</a>. If a tag with the same key name already exists, then that tag is overwritten with the new value.</p>
/// <p>A tag consists of a key name and an associated value. By assigning tags to your resources, you can do the following:</p>
/// <ul>
/// <li> <p> <b>Administrative grouping and discovery</b> - Attach tags to resources to aid in organization and search. For example, you could search for all resources with the key name <i>Project</i> and the value <i>MyImportantProject</i>. Or search for all resources with the key name <i>Cost Center</i> and the value <i>41200</i>. </p> </li>
/// <li> <p> <b>Access control</b> - Include tags in IAM identity-based and resource-based policies. You can use tags to restrict access to only an OIDC provider that has a specified tag attached. For examples of policies that show how to use tags to control access, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html">Control access using IAM tags</a> in the <i>IAM User Guide</i>.</p> </li>
/// </ul> <note>
/// <ul>
/// <li> <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request fails and the resource is not created. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p> </li>
/// <li> <p>Amazon Web Services always interprets the tag <code>Value</code> as a single string. If you need to store an array, you can store comma-separated values in the string. However, you must interpret the value in your code.</p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TagOpenIDConnectProviderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::tag_open_id_connect_provider::builders::TagOpenIdConnectProviderInputBuilder,
}
impl TagOpenIDConnectProviderFluentBuilder {
    /// Creates a new `TagOpenIDConnectProvider`.
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
            crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProvider,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
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
        crate::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
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
        crate::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
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
            crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProvider,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN of the OIDC identity provider in IAM to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn open_id_connect_provider_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.open_id_connect_provider_arn(input.into());
        self
    }
    /// <p>The ARN of the OIDC identity provider in IAM to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_open_id_connect_provider_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_open_id_connect_provider_arn(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags that you want to attach to the OIDC identity provider in IAM. Each tag consists of a key name and an associated value.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The list of tags that you want to attach to the OIDC identity provider in IAM. Each tag consists of a key name and an associated value.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
