// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_network_interface_attribute::_modify_network_interface_attribute_output::ModifyNetworkInterfaceAttributeOutputBuilder;

pub use crate::operation::modify_network_interface_attribute::_modify_network_interface_attribute_input::ModifyNetworkInterfaceAttributeInputBuilder;

/// Fluent builder constructing a request to `ModifyNetworkInterfaceAttribute`.
///
/// <p>Modifies the specified network interface attribute. You can specify only one attribute at a time. You can use this action to attach and detach security groups from an existing EC2 instance.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyNetworkInterfaceAttributeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeInputBuilder,
}
impl ModifyNetworkInterfaceAttributeFluentBuilder {
    /// Creates a new `ModifyNetworkInterfaceAttribute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttribute, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput, ::aws_smithy_http::result::SdkError<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput, ::aws_smithy_http::result::SdkError<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttribute, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError>
    >{
        self.customize_middleware().await
    }
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn attachment(mut self, input: crate::types::NetworkInterfaceAttachmentChanges) -> Self {
        self.inner = self.inner.attachment(input);
        self
    }
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn set_attachment(
        mut self,
        input: ::std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>,
    ) -> Self {
        self.inner = self.inner.set_attachment(input);
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn description(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.description(input);
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn set_description(
        mut self,
        input: ::std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.inner = self.inner.set_description(input);
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
    /// Appends an item to `Groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.groups(input.into());
        self
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn set_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_groups(input);
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.network_interface_id(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_network_interface_id(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.inner = self.inner.source_dest_check(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn set_source_dest_check(
        mut self,
        input: ::std::option::Option<crate::types::AttributeBooleanValue>,
    ) -> Self {
        self.inner = self.inner.set_source_dest_check(input);
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn ena_srd_specification(mut self, input: crate::types::EnaSrdSpecification) -> Self {
        self.inner = self.inner.ena_srd_specification(input);
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn set_ena_srd_specification(
        mut self,
        input: ::std::option::Option<crate::types::EnaSrdSpecification>,
    ) -> Self {
        self.inner = self.inner.set_ena_srd_specification(input);
        self
    }
}
