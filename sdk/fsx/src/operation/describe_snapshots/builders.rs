// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_snapshots::_describe_snapshots_output::DescribeSnapshotsOutputBuilder;

pub use crate::operation::describe_snapshots::_describe_snapshots_input::DescribeSnapshotsInputBuilder;

/// Fluent builder constructing a request to `DescribeSnapshots`.
///
/// <p>Returns the description of specific Amazon FSx for OpenZFS snapshots, if a <code>SnapshotIds</code> value is provided. Otherwise, this operation returns all snapshots owned by your Amazon Web Services account in the Amazon Web Services Region of the endpoint that you're calling.</p>
/// <p>When retrieving all snapshots, you can optionally specify the <code>MaxResults</code> parameter to limit the number of snapshots in a response. If more backups remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response. </p>
/// <p>Use this operation in an iterative process to retrieve a list of your snapshots. <code>DescribeSnapshots</code> is called first without a <code>NextToken</code> value. Then the operation continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code> value.</p>
/// <p>When using this operation, keep the following in mind:</p>
/// <ul>
/// <li> <p>The operation might return fewer than the <code>MaxResults</code> value of snapshot descriptions while still including a <code>NextToken</code> value.</p> </li>
/// <li> <p>The order of snapshots returned in the response of one <code>DescribeSnapshots</code> call and the order of backups returned across the responses of a multi-call iteration is unspecified. </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeSnapshotsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_snapshots::builders::DescribeSnapshotsInputBuilder,
}
impl DescribeSnapshotsFluentBuilder {
    /// Creates a new `DescribeSnapshots`.
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
            crate::operation::describe_snapshots::DescribeSnapshots,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_snapshots::DescribeSnapshotsError,
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
        crate::operation::describe_snapshots::DescribeSnapshotsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_snapshots::DescribeSnapshotsError,
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
        crate::operation::describe_snapshots::DescribeSnapshotsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_snapshots::DescribeSnapshotsError,
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
            crate::operation::describe_snapshots::DescribeSnapshots,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_snapshots::DescribeSnapshotsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_snapshots::paginator::DescribeSnapshotsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_snapshots::paginator::DescribeSnapshotsPaginator {
        crate::operation::describe_snapshots::paginator::DescribeSnapshotsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `SnapshotIds`.
    ///
    /// To override the contents of this collection use [`set_snapshot_ids`](Self::set_snapshot_ids).
    ///
    /// <p>The IDs of the snapshots that you want to retrieve. This parameter value overrides any filters. If any IDs aren't found, a <code>SnapshotNotFound</code> error occurs.</p>
    pub fn snapshot_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_ids(input.into());
        self
    }
    /// <p>The IDs of the snapshots that you want to retrieve. This parameter value overrides any filters. If any IDs aren't found, a <code>SnapshotNotFound</code> error occurs.</p>
    pub fn set_snapshot_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_snapshot_ids(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters structure. The supported names are <code>file-system-id</code> or <code>volume-id</code>.</p>
    pub fn filters(mut self, input: crate::types::SnapshotFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters structure. The supported names are <code>file-system-id</code> or <code>volume-id</code>.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SnapshotFilter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of resources to return in the response. This value must be an integer greater than zero.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of resources to return in the response. This value must be an integer greater than zero.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>(Optional) Opaque pagination token returned from a previous operation (String). If present, this token indicates from what point you can continue processing the request, where the previous <code>NextToken</code> value left off.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>(Optional) Opaque pagination token returned from a previous operation (String). If present, this token indicates from what point you can continue processing the request, where the previous <code>NextToken</code> value left off.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
