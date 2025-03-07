// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_capacity_reservations::_describe_capacity_reservations_output::DescribeCapacityReservationsOutputBuilder;

pub use crate::operation::describe_capacity_reservations::_describe_capacity_reservations_input::DescribeCapacityReservationsInputBuilder;

/// Fluent builder constructing a request to `DescribeCapacityReservations`.
///
/// <p>Describes one or more of your Capacity Reservations. The results describe only the Capacity Reservations in the Amazon Web Services Region that you're currently using.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeCapacityReservationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_capacity_reservations::builders::DescribeCapacityReservationsInputBuilder,
}
impl DescribeCapacityReservationsFluentBuilder {
    /// Creates a new `DescribeCapacityReservations`.
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
            crate::operation::describe_capacity_reservations::DescribeCapacityReservations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_capacity_reservations::DescribeCapacityReservationsError,
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
        crate::operation::describe_capacity_reservations::DescribeCapacityReservationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_capacity_reservations::DescribeCapacityReservationsError,
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
        crate::operation::describe_capacity_reservations::DescribeCapacityReservationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_capacity_reservations::DescribeCapacityReservationsError,
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
            crate::operation::describe_capacity_reservations::DescribeCapacityReservations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_capacity_reservations::DescribeCapacityReservationsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_capacity_reservations::paginator::DescribeCapacityReservationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_capacity_reservations::paginator::DescribeCapacityReservationsPaginator{
        crate::operation::describe_capacity_reservations::paginator::DescribeCapacityReservationsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `CapacityReservationIds`.
    ///
    /// To override the contents of this collection use [`set_capacity_reservation_ids`](Self::set_capacity_reservation_ids).
    ///
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn capacity_reservation_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.capacity_reservation_ids(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn set_capacity_reservation_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_capacity_reservation_ids(input);
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>instance-type</code> - The type of instance for which the Capacity Reservation reserves capacity.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the Capacity Reservation.</p> </li>
    /// <li> <p> <code>instance-platform</code> - The type of operating system for which the Capacity Reservation reserves capacity.</p> </li>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone of the Capacity Reservation.</p> </li>
    /// <li> <p> <code>tenancy</code> - Indicates the tenancy of the Capacity Reservation. A Capacity Reservation can have one of the following tenancy settings:</p>
    /// <ul>
    /// <li> <p> <code>default</code> - The Capacity Reservation is created on hardware that is shared with other Amazon Web Services accounts.</p> </li>
    /// <li> <p> <code>dedicated</code> - The Capacity Reservation is created on single-tenant hardware that is dedicated to a single Amazon Web Services account.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost on which the Capacity Reservation was created.</p> </li>
    /// <li> <p> <code>state</code> - The current state of the Capacity Reservation. A Capacity Reservation can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>active</code>- The Capacity Reservation is active and the capacity is available for your use.</p> </li>
    /// <li> <p> <code>expired</code> - The Capacity Reservation expired automatically at the date and time specified in your request. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>cancelled</code> - The Capacity Reservation was cancelled. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>pending</code> - The Capacity Reservation request was successful but the capacity provisioning is still pending.</p> </li>
    /// <li> <p> <code>failed</code> - The Capacity Reservation request has failed. A request might fail due to invalid request parameters, capacity constraints, or instance limit constraints. Failed requests are retained for 60 minutes.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>start-date</code> - The date and time at which the Capacity Reservation was started.</p> </li>
    /// <li> <p> <code>end-date</code> - The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to expired when it reaches its end date and time.</p> </li>
    /// <li> <p> <code>end-date-type</code> - Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>instance-match-criteria</code> - Indicates the type of instance launches that the Capacity Reservation accepts. The options include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The Capacity Reservation accepts all instances that have matching attributes (instance type, platform, and Availability Zone). Instances that have matching attributes launch into the Capacity Reservation automatically without specifying any additional parameters.</p> </li>
    /// <li> <p> <code>targeted</code> - The Capacity Reservation only accepts instances that have matching attributes (instance type, platform, and Availability Zone), and explicitly target the Capacity Reservation. This ensures that only permitted instances can use the reserved capacity.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>placement-group-arn</code> - The ARN of the cluster placement group in which the Capacity Reservation was created.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>instance-type</code> - The type of instance for which the Capacity Reservation reserves capacity.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the Capacity Reservation.</p> </li>
    /// <li> <p> <code>instance-platform</code> - The type of operating system for which the Capacity Reservation reserves capacity.</p> </li>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone of the Capacity Reservation.</p> </li>
    /// <li> <p> <code>tenancy</code> - Indicates the tenancy of the Capacity Reservation. A Capacity Reservation can have one of the following tenancy settings:</p>
    /// <ul>
    /// <li> <p> <code>default</code> - The Capacity Reservation is created on hardware that is shared with other Amazon Web Services accounts.</p> </li>
    /// <li> <p> <code>dedicated</code> - The Capacity Reservation is created on single-tenant hardware that is dedicated to a single Amazon Web Services account.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost on which the Capacity Reservation was created.</p> </li>
    /// <li> <p> <code>state</code> - The current state of the Capacity Reservation. A Capacity Reservation can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>active</code>- The Capacity Reservation is active and the capacity is available for your use.</p> </li>
    /// <li> <p> <code>expired</code> - The Capacity Reservation expired automatically at the date and time specified in your request. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>cancelled</code> - The Capacity Reservation was cancelled. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>pending</code> - The Capacity Reservation request was successful but the capacity provisioning is still pending.</p> </li>
    /// <li> <p> <code>failed</code> - The Capacity Reservation request has failed. A request might fail due to invalid request parameters, capacity constraints, or instance limit constraints. Failed requests are retained for 60 minutes.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>start-date</code> - The date and time at which the Capacity Reservation was started.</p> </li>
    /// <li> <p> <code>end-date</code> - The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to expired when it reaches its end date and time.</p> </li>
    /// <li> <p> <code>end-date-type</code> - Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>instance-match-criteria</code> - Indicates the type of instance launches that the Capacity Reservation accepts. The options include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The Capacity Reservation accepts all instances that have matching attributes (instance type, platform, and Availability Zone). Instances that have matching attributes launch into the Capacity Reservation automatically without specifying any additional parameters.</p> </li>
    /// <li> <p> <code>targeted</code> - The Capacity Reservation only accepts instances that have matching attributes (instance type, platform, and Availability Zone), and explicitly target the Capacity Reservation. This ensures that only permitted instances can use the reserved capacity.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>placement-group-arn</code> - The ARN of the cluster placement group in which the Capacity Reservation was created.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
