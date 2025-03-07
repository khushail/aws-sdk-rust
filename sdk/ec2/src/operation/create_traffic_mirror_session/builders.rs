// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_traffic_mirror_session::_create_traffic_mirror_session_output::CreateTrafficMirrorSessionOutputBuilder;

pub use crate::operation::create_traffic_mirror_session::_create_traffic_mirror_session_input::CreateTrafficMirrorSessionInputBuilder;

/// Fluent builder constructing a request to `CreateTrafficMirrorSession`.
///
/// <p>Creates a Traffic Mirror session.</p>
/// <p>A Traffic Mirror session actively copies packets from a Traffic Mirror source to a Traffic Mirror target. Create a filter, and then assign it to the session to define a subset of the traffic to mirror, for example all TCP traffic.</p>
/// <p>The Traffic Mirror source and the Traffic Mirror target (monitoring appliances) can be in the same VPC, or in a different VPC connected via VPC peering or a transit gateway. </p>
/// <p>By default, no traffic is mirrored. Use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTrafficMirrorFilter.htm">CreateTrafficMirrorFilter</a> to create filter rules that specify the traffic to mirror.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTrafficMirrorSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_traffic_mirror_session::builders::CreateTrafficMirrorSessionInputBuilder,
}
impl CreateTrafficMirrorSessionFluentBuilder {
    /// Creates a new `CreateTrafficMirrorSession`.
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
            crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSession,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSessionError,
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
        crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSessionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSessionError,
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
        crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSessionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSessionError,
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
            crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSession,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_mirror_session::CreateTrafficMirrorSessionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the source network interface.</p>
    pub fn network_interface_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.network_interface_id(input.into());
        self
    }
    /// <p>The ID of the source network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_network_interface_id(input);
        self
    }
    /// <p>The ID of the Traffic Mirror target.</p>
    pub fn traffic_mirror_target_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.traffic_mirror_target_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror target.</p>
    pub fn set_traffic_mirror_target_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_mirror_target_id(input);
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn traffic_mirror_filter_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.traffic_mirror_filter_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn set_traffic_mirror_filter_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_mirror_filter_id(input);
        self
    }
    /// <p>The number of bytes in each packet to mirror. These are bytes after the VXLAN header. Do not specify this parameter when you want to mirror the entire packet. To mirror a subset of the packet, set this to the length (in bytes) that you want to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target.</p>
    /// <p>If you do not want to mirror the entire packet, use the <code>PacketLength</code> parameter to specify the number of bytes in each packet to mirror.</p>
    pub fn packet_length(mut self, input: i32) -> Self {
        self.inner = self.inner.packet_length(input);
        self
    }
    /// <p>The number of bytes in each packet to mirror. These are bytes after the VXLAN header. Do not specify this parameter when you want to mirror the entire packet. To mirror a subset of the packet, set this to the length (in bytes) that you want to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target.</p>
    /// <p>If you do not want to mirror the entire packet, use the <code>PacketLength</code> parameter to specify the number of bytes in each packet to mirror.</p>
    pub fn set_packet_length(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_packet_length(input);
        self
    }
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    pub fn session_number(mut self, input: i32) -> Self {
        self.inner = self.inner.session_number(input);
        self
    }
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    pub fn set_session_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_session_number(input);
        self
    }
    /// <p>The VXLAN ID for the Traffic Mirror session. For more information about the VXLAN protocol, see <a href="https://tools.ietf.org/html/rfc7348">RFC 7348</a>. If you do not specify a <code>VirtualNetworkId</code>, an account-wide unique id is chosen at random.</p>
    pub fn virtual_network_id(mut self, input: i32) -> Self {
        self.inner = self.inner.virtual_network_id(input);
        self
    }
    /// <p>The VXLAN ID for the Traffic Mirror session. For more information about the VXLAN protocol, see <a href="https://tools.ietf.org/html/rfc7348">RFC 7348</a>. If you do not specify a <code>VirtualNetworkId</code>, an account-wide unique id is chosen at random.</p>
    pub fn set_virtual_network_id(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_virtual_network_id(input);
        self
    }
    /// <p>The description of the Traffic Mirror session.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the Traffic Mirror session.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to a Traffic Mirror session.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to assign to a Traffic Mirror session.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
