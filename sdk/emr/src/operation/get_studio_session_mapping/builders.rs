// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_studio_session_mapping::_get_studio_session_mapping_output::GetStudioSessionMappingOutputBuilder;

pub use crate::operation::get_studio_session_mapping::_get_studio_session_mapping_input::GetStudioSessionMappingInputBuilder;

/// Fluent builder constructing a request to `GetStudioSessionMapping`.
///
/// <p>Fetches mapping details for the specified Amazon EMR Studio and identity (user or group).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetStudioSessionMappingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::get_studio_session_mapping::builders::GetStudioSessionMappingInputBuilder,
}
impl GetStudioSessionMappingFluentBuilder {
    /// Creates a new `GetStudioSessionMapping`.
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
            crate::operation::get_studio_session_mapping::GetStudioSessionMapping,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_studio_session_mapping::GetStudioSessionMappingError,
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
        crate::operation::get_studio_session_mapping::GetStudioSessionMappingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_studio_session_mapping::GetStudioSessionMappingError,
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
        crate::operation::get_studio_session_mapping::GetStudioSessionMappingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_studio_session_mapping::GetStudioSessionMappingError,
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
            crate::operation::get_studio_session_mapping::GetStudioSessionMapping,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_studio_session_mapping::GetStudioSessionMappingError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the Amazon EMR Studio.</p>
    pub fn studio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.studio_id(input.into());
        self
    }
    /// <p>The ID of the Amazon EMR Studio.</p>
    pub fn set_studio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_studio_id(input);
        self
    }
    /// <p>The globally unique identifier (GUID) of the user or group. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_User.html#singlesignon-Type-User-UserId">UserId</a> and <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_Group.html#singlesignon-Type-Group-GroupId">GroupId</a> in the <i>IAM Identity Center Identity Store API Reference</i>. Either <code>IdentityName</code> or <code>IdentityId</code> must be specified.</p>
    pub fn identity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_id(input.into());
        self
    }
    /// <p>The globally unique identifier (GUID) of the user or group. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_User.html#singlesignon-Type-User-UserId">UserId</a> and <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_Group.html#singlesignon-Type-Group-GroupId">GroupId</a> in the <i>IAM Identity Center Identity Store API Reference</i>. Either <code>IdentityName</code> or <code>IdentityId</code> must be specified.</p>
    pub fn set_identity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_id(input);
        self
    }
    /// <p>The name of the user or group to fetch. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_User.html#singlesignon-Type-User-UserName">UserName</a> and <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_Group.html#singlesignon-Type-Group-DisplayName">DisplayName</a> in the <i>IAM Identity Center Identity Store API Reference</i>. Either <code>IdentityName</code> or <code>IdentityId</code> must be specified.</p>
    pub fn identity_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.identity_name(input.into());
        self
    }
    /// <p>The name of the user or group to fetch. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_User.html#singlesignon-Type-User-UserName">UserName</a> and <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_Group.html#singlesignon-Type-Group-DisplayName">DisplayName</a> in the <i>IAM Identity Center Identity Store API Reference</i>. Either <code>IdentityName</code> or <code>IdentityId</code> must be specified.</p>
    pub fn set_identity_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_identity_name(input);
        self
    }
    /// <p>Specifies whether the identity to fetch is a user or a group.</p>
    pub fn identity_type(mut self, input: crate::types::IdentityType) -> Self {
        self.inner = self.inner.identity_type(input);
        self
    }
    /// <p>Specifies whether the identity to fetch is a user or a group.</p>
    pub fn set_identity_type(
        mut self,
        input: ::std::option::Option<crate::types::IdentityType>,
    ) -> Self {
        self.inner = self.inner.set_identity_type(input);
        self
    }
}
