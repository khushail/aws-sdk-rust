// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeregisterGatewayInstanceOutput {
    /// The Amazon Resource Name (ARN) of the instance.
    #[doc(hidden)]
    pub gateway_instance_arn: ::std::option::Option<::std::string::String>,
    /// The status of the instance.
    #[doc(hidden)]
    pub instance_state: ::std::option::Option<crate::types::InstanceState>,
    _request_id: Option<String>,
}
impl DeregisterGatewayInstanceOutput {
    /// The Amazon Resource Name (ARN) of the instance.
    pub fn gateway_instance_arn(&self) -> ::std::option::Option<&str> {
        self.gateway_instance_arn.as_deref()
    }
    /// The status of the instance.
    pub fn instance_state(&self) -> ::std::option::Option<&crate::types::InstanceState> {
        self.instance_state.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DeregisterGatewayInstanceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeregisterGatewayInstanceOutput {
    /// Creates a new builder-style object to manufacture [`DeregisterGatewayInstanceOutput`](crate::operation::deregister_gateway_instance::DeregisterGatewayInstanceOutput).
    pub fn builder() -> crate::operation::deregister_gateway_instance::builders::DeregisterGatewayInstanceOutputBuilder{
        crate::operation::deregister_gateway_instance::builders::DeregisterGatewayInstanceOutputBuilder::default()
    }
}

/// A builder for [`DeregisterGatewayInstanceOutput`](crate::operation::deregister_gateway_instance::DeregisterGatewayInstanceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeregisterGatewayInstanceOutputBuilder {
    pub(crate) gateway_instance_arn: ::std::option::Option<::std::string::String>,
    pub(crate) instance_state: ::std::option::Option<crate::types::InstanceState>,
    _request_id: Option<String>,
}
impl DeregisterGatewayInstanceOutputBuilder {
    /// The Amazon Resource Name (ARN) of the instance.
    pub fn gateway_instance_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.gateway_instance_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The Amazon Resource Name (ARN) of the instance.
    pub fn set_gateway_instance_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.gateway_instance_arn = input;
        self
    }
    /// The status of the instance.
    pub fn instance_state(mut self, input: crate::types::InstanceState) -> Self {
        self.instance_state = ::std::option::Option::Some(input);
        self
    }
    /// The status of the instance.
    pub fn set_instance_state(
        mut self,
        input: ::std::option::Option<crate::types::InstanceState>,
    ) -> Self {
        self.instance_state = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeregisterGatewayInstanceOutput`](crate::operation::deregister_gateway_instance::DeregisterGatewayInstanceOutput).
    pub fn build(
        self,
    ) -> crate::operation::deregister_gateway_instance::DeregisterGatewayInstanceOutput {
        crate::operation::deregister_gateway_instance::DeregisterGatewayInstanceOutput {
            gateway_instance_arn: self.gateway_instance_arn,
            instance_state: self.instance_state,
            _request_id: self._request_id,
        }
    }
}
