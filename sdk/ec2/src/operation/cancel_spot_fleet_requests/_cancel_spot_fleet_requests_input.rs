// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for CancelSpotFleetRequests.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelSpotFleetRequestsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The IDs of the Spot Fleet requests.</p>
    #[doc(hidden)]
    pub spot_fleet_request_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Indicates whether to terminate the associated instances when the Spot Fleet request is canceled. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the Spot Fleet request is canceled, specify <code>no-terminate-instances</code>.</p>
    #[doc(hidden)]
    pub terminate_instances: ::std::option::Option<bool>,
}
impl CancelSpotFleetRequestsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the Spot Fleet requests.</p>
    pub fn spot_fleet_request_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.spot_fleet_request_ids.as_deref()
    }
    /// <p>Indicates whether to terminate the associated instances when the Spot Fleet request is canceled. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the Spot Fleet request is canceled, specify <code>no-terminate-instances</code>.</p>
    pub fn terminate_instances(&self) -> ::std::option::Option<bool> {
        self.terminate_instances
    }
}
impl CancelSpotFleetRequestsInput {
    /// Creates a new builder-style object to manufacture [`CancelSpotFleetRequestsInput`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsInput).
    pub fn builder(
    ) -> crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsInputBuilder
    {
        crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsInputBuilder::default()
    }
}

/// A builder for [`CancelSpotFleetRequestsInput`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelSpotFleetRequestsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) spot_fleet_request_ids:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) terminate_instances: ::std::option::Option<bool>,
}
impl CancelSpotFleetRequestsInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Appends an item to `spot_fleet_request_ids`.
    ///
    /// To override the contents of this collection use [`set_spot_fleet_request_ids`](Self::set_spot_fleet_request_ids).
    ///
    /// <p>The IDs of the Spot Fleet requests.</p>
    pub fn spot_fleet_request_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.spot_fleet_request_ids.unwrap_or_default();
        v.push(input.into());
        self.spot_fleet_request_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the Spot Fleet requests.</p>
    pub fn set_spot_fleet_request_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.spot_fleet_request_ids = input;
        self
    }
    /// <p>Indicates whether to terminate the associated instances when the Spot Fleet request is canceled. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the Spot Fleet request is canceled, specify <code>no-terminate-instances</code>.</p>
    pub fn terminate_instances(mut self, input: bool) -> Self {
        self.terminate_instances = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to terminate the associated instances when the Spot Fleet request is canceled. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the Spot Fleet request is canceled, specify <code>no-terminate-instances</code>.</p>
    pub fn set_terminate_instances(mut self, input: ::std::option::Option<bool>) -> Self {
        self.terminate_instances = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelSpotFleetRequestsInput`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsInput {
                dry_run: self.dry_run,
                spot_fleet_request_ids: self.spot_fleet_request_ids,
                terminate_instances: self.terminate_instances,
            },
        )
    }
}
