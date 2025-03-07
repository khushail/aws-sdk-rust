// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_identity_pool_usage::_describe_identity_pool_usage_output::DescribeIdentityPoolUsageOutputBuilder;

pub use crate::operation::describe_identity_pool_usage::_describe_identity_pool_usage_input::DescribeIdentityPoolUsageInputBuilder;

/// Fluent builder constructing a request to `DescribeIdentityPoolUsage`.
///
/// <p>Gets usage details (for example, data storage) about a particular identity pool.</p>
/// <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p> <examples>
/// <example>
/// <name>
/// DescribeIdentityPoolUsage
/// </name>
/// <description>
/// The following examples have been edited for readability.
/// </description>
/// <request>
/// POST / HTTP/1.1 CONTENT-TYPE: application/json X-AMZN-REQUESTID: 8dc0e749-c8cd-48bd-8520-da6be00d528b X-AMZ-TARGET: com.amazonaws.cognito.sync.model.AWSCognitoSyncService.DescribeIdentityPoolUsage HOST: cognito-sync.us-east-1.amazonaws.com:443 X-AMZ-DATE: 20141111T205737Z AUTHORIZATION: AWS4-HMAC-SHA256 Credential=
/// <credential>
/// , SignedHeaders=content-type;host;x-amz-date;x-amz-target;x-amzn-requestid, Signature=
/// <signature>
/// { "Operation": "com.amazonaws.cognito.sync.model#DescribeIdentityPoolUsage", "Service": "com.amazonaws.cognito.sync.model#AWSCognitoSyncService", "Input": { "IdentityPoolId": "IDENTITY_POOL_ID" } }
/// </signature>
/// </credential>
/// </request>
/// <response>
/// 1.1 200 OK x-amzn-requestid: 8dc0e749-c8cd-48bd-8520-da6be00d528b content-type: application/json content-length: 271 date: Tue, 11 Nov 2014 20:57:37 GMT { "Output": { "__type": "com.amazonaws.cognito.sync.model#DescribeIdentityPoolUsageResponse", "IdentityPoolUsage": { "DataStorage": 0, "IdentityPoolId": "IDENTITY_POOL_ID", "LastModifiedDate": 1.413231134115E9, "SyncSessionsCount": null } }, "Version": "1.0" }
/// </response>
/// </example>
/// </examples>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeIdentityPoolUsageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_identity_pool_usage::builders::DescribeIdentityPoolUsageInputBuilder,
}
impl DescribeIdentityPoolUsageFluentBuilder {
    /// Creates a new `DescribeIdentityPoolUsage`.
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
            crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsageError,
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
        crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsageError,
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
        crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsageError,
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
            crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_identity_pool_usage::DescribeIdentityPoolUsageError,
        >,
    > {
        self.customize_middleware().await
    }
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn identity_pool_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.identity_pool_id(input.into());
        self
    }
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn set_identity_pool_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_identity_pool_id(input);
        self
    }
}
