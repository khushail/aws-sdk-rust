// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_log_groups::_describe_log_groups_output::DescribeLogGroupsOutputBuilder;

pub use crate::operation::describe_log_groups::_describe_log_groups_input::DescribeLogGroupsInputBuilder;

/// Fluent builder constructing a request to `DescribeLogGroups`.
///
/// <p>Lists the specified log groups. You can list all your log groups or filter the results by prefix. The results are ASCII-sorted by log group name.</p>
/// <p>CloudWatch Logs doesn’t support IAM policies that control access to the <code>DescribeLogGroups</code> action by using the <code>aws:ResourceTag/<i>key-name</i> </code> condition key. Other CloudWatch Logs actions do support the use of the <code>aws:ResourceTag/<i>key-name</i> </code> condition key to control access. For more information about using tags to control access, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html">Controlling access to Amazon Web Services resources using tags</a>.</p>
/// <p>If you are using CloudWatch cross-account observability, you can use this operation in a monitoring account and view data from the linked source accounts. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html">CloudWatch cross-account observability</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeLogGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_log_groups::builders::DescribeLogGroupsInputBuilder,
}
impl DescribeLogGroupsFluentBuilder {
    /// Creates a new `DescribeLogGroups`.
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
            crate::operation::describe_log_groups::DescribeLogGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_log_groups::DescribeLogGroupsError,
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
        crate::operation::describe_log_groups::DescribeLogGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_log_groups::DescribeLogGroupsError,
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
        crate::operation::describe_log_groups::DescribeLogGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_log_groups::DescribeLogGroupsError,
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
            crate::operation::describe_log_groups::DescribeLogGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_log_groups::DescribeLogGroupsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_log_groups::paginator::DescribeLogGroupsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_log_groups::paginator::DescribeLogGroupsPaginator {
        crate::operation::describe_log_groups::paginator::DescribeLogGroupsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `accountIdentifiers`.
    ///
    /// To override the contents of this collection use [`set_account_identifiers`](Self::set_account_identifiers).
    ///
    /// <p>When <code>includeLinkedAccounts</code> is set to <code>True</code>, use this parameter to specify the list of accounts to search. You can specify as many as 20 account IDs in the array. </p>
    pub fn account_identifiers(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.account_identifiers(input.into());
        self
    }
    /// <p>When <code>includeLinkedAccounts</code> is set to <code>True</code>, use this parameter to specify the list of accounts to search. You can specify as many as 20 account IDs in the array. </p>
    pub fn set_account_identifiers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_account_identifiers(input);
        self
    }
    /// <p>The prefix to match.</p> <note>
    /// <p> <code>logGroupNamePrefix</code> and <code>logGroupNamePattern</code> are mutually exclusive. Only one of these parameters can be passed. </p>
    /// </note>
    pub fn log_group_name_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.log_group_name_prefix(input.into());
        self
    }
    /// <p>The prefix to match.</p> <note>
    /// <p> <code>logGroupNamePrefix</code> and <code>logGroupNamePattern</code> are mutually exclusive. Only one of these parameters can be passed. </p>
    /// </note>
    pub fn set_log_group_name_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_log_group_name_prefix(input);
        self
    }
    /// <p>If you specify a string for this parameter, the operation returns only log groups that have names that match the string based on a case-sensitive substring search. For example, if you specify <code>Foo</code>, log groups named <code>FooBar</code>, <code>aws/Foo</code>, and <code>GroupFoo</code> would match, but <code>foo</code>, <code>F/o/o</code> and <code>Froo</code> would not match.</p> <note>
    /// <p> <code>logGroupNamePattern</code> and <code>logGroupNamePrefix</code> are mutually exclusive. Only one of these parameters can be passed. </p>
    /// </note>
    pub fn log_group_name_pattern(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.log_group_name_pattern(input.into());
        self
    }
    /// <p>If you specify a string for this parameter, the operation returns only log groups that have names that match the string based on a case-sensitive substring search. For example, if you specify <code>Foo</code>, log groups named <code>FooBar</code>, <code>aws/Foo</code>, and <code>GroupFoo</code> would match, but <code>foo</code>, <code>F/o/o</code> and <code>Froo</code> would not match.</p> <note>
    /// <p> <code>logGroupNamePattern</code> and <code>logGroupNamePrefix</code> are mutually exclusive. Only one of these parameters can be passed. </p>
    /// </note>
    pub fn set_log_group_name_pattern(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_log_group_name_pattern(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>If you are using a monitoring account, set this to <code>True</code> to have the operation return log groups in the accounts listed in <code>accountIdentifiers</code>.</p>
    /// <p>If this parameter is set to <code>true</code> and <code>accountIdentifiers</code> contains a null value, the operation returns all log groups in the monitoring account and all log groups in all source accounts that are linked to the monitoring account. </p> <note>
    /// <p> If you specify <code>includeLinkedAccounts</code> in your request, then <code>metricFilterCount</code>, <code>retentionInDays</code>, and <code>storedBytes</code> are not included in the response. </p>
    /// </note>
    pub fn include_linked_accounts(mut self, input: bool) -> Self {
        self.inner = self.inner.include_linked_accounts(input);
        self
    }
    /// <p>If you are using a monitoring account, set this to <code>True</code> to have the operation return log groups in the accounts listed in <code>accountIdentifiers</code>.</p>
    /// <p>If this parameter is set to <code>true</code> and <code>accountIdentifiers</code> contains a null value, the operation returns all log groups in the monitoring account and all log groups in all source accounts that are linked to the monitoring account. </p> <note>
    /// <p> If you specify <code>includeLinkedAccounts</code> in your request, then <code>metricFilterCount</code>, <code>retentionInDays</code>, and <code>storedBytes</code> are not included in the response. </p>
    /// </note>
    pub fn set_include_linked_accounts(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_linked_accounts(input);
        self
    }
}
