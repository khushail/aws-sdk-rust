// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_access_point::_delete_access_point_output::DeleteAccessPointOutputBuilder;

pub use crate::operation::delete_access_point::_delete_access_point_input::DeleteAccessPointInputBuilder;

/// Fluent builder constructing a request to `DeleteAccessPoint`.
///
/// <p>Deletes the specified access point.</p>
/// <p>All Amazon S3 on Outposts REST API requests for this action require an additional parameter of <code>x-amz-outpost-id</code> to be passed with the request. In addition, you must use an S3 on Outposts endpoint hostname prefix instead of <code>s3-control</code>. For an example of the request syntax for Amazon S3 on Outposts that uses the S3 on Outposts endpoint hostname prefix and the <code>x-amz-outpost-id</code> derived by using the access point ARN, see the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_DeleteAccessPoint.html#API_control_DeleteAccessPoint_Examples">Examples</a> section.</p>
/// <p>The following actions are related to <code>DeleteAccessPoint</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_CreateAccessPoint.html">CreateAccessPoint</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_GetAccessPoint.html">GetAccessPoint</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_ListAccessPoints.html">ListAccessPoints</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteAccessPointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_access_point::builders::DeleteAccessPointInputBuilder,
}
impl DeleteAccessPointFluentBuilder {
    /// Creates a new `DeleteAccessPoint`.
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
            crate::operation::delete_access_point::DeleteAccessPoint,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_access_point::DeleteAccessPointError,
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
        crate::operation::delete_access_point::DeleteAccessPointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_access_point::DeleteAccessPointError,
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
        crate::operation::delete_access_point::DeleteAccessPointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_access_point::DeleteAccessPointError,
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
            crate::operation::delete_access_point::DeleteAccessPoint,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_access_point::DeleteAccessPointError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Web Services account ID for the account that owns the specified access point.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the account that owns the specified access point.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The name of the access point you want to delete.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /accesspoint/
    /// <my-accesspoint-name></my-accesspoint-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the access point you want to delete.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /accesspoint/
    /// <my-accesspoint-name></my-accesspoint-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
}
