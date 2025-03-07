// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_capacity_reservation_fleets::_cancel_capacity_reservation_fleets_output::CancelCapacityReservationFleetsOutputBuilder;

pub use crate::operation::cancel_capacity_reservation_fleets::_cancel_capacity_reservation_fleets_input::CancelCapacityReservationFleetsInputBuilder;

/// Fluent builder constructing a request to `CancelCapacityReservationFleets`.
///
/// <p>Cancels one or more Capacity Reservation Fleets. When you cancel a Capacity Reservation Fleet, the following happens:</p>
/// <ul>
/// <li> <p>The Capacity Reservation Fleet's status changes to <code>cancelled</code>.</p> </li>
/// <li> <p>The individual Capacity Reservations in the Fleet are cancelled. Instances running in the Capacity Reservations at the time of cancelling the Fleet continue to run in shared capacity.</p> </li>
/// <li> <p>The Fleet stops creating new Capacity Reservations.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelCapacityReservationFleetsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::cancel_capacity_reservation_fleets::builders::CancelCapacityReservationFleetsInputBuilder,
}
impl CancelCapacityReservationFleetsFluentBuilder {
    /// Creates a new `CancelCapacityReservationFleets`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleets, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsOutput, ::aws_smithy_http::result::SdkError<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsOutput, ::aws_smithy_http::result::SdkError<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleets, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsError>
    >{
        self.customize_middleware().await
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
    /// Appends an item to `CapacityReservationFleetIds`.
    ///
    /// To override the contents of this collection use [`set_capacity_reservation_fleet_ids`](Self::set_capacity_reservation_fleet_ids).
    ///
    /// <p>The IDs of the Capacity Reservation Fleets to cancel.</p>
    pub fn capacity_reservation_fleet_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.capacity_reservation_fleet_ids(input.into());
        self
    }
    /// <p>The IDs of the Capacity Reservation Fleets to cancel.</p>
    pub fn set_capacity_reservation_fleet_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_capacity_reservation_fleet_ids(input);
        self
    }
}
