// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartSmartHomeApplianceDiscoveryInput {
    /// <p>The room where smart home appliance discovery was initiated.</p>
    #[doc(hidden)]
    pub room_arn: ::std::option::Option<::std::string::String>,
}
impl StartSmartHomeApplianceDiscoveryInput {
    /// <p>The room where smart home appliance discovery was initiated.</p>
    pub fn room_arn(&self) -> ::std::option::Option<&str> {
        self.room_arn.as_deref()
    }
}
impl StartSmartHomeApplianceDiscoveryInput {
    /// Creates a new builder-style object to manufacture [`StartSmartHomeApplianceDiscoveryInput`](crate::operation::start_smart_home_appliance_discovery::StartSmartHomeApplianceDiscoveryInput).
    pub fn builder() -> crate::operation::start_smart_home_appliance_discovery::builders::StartSmartHomeApplianceDiscoveryInputBuilder{
        crate::operation::start_smart_home_appliance_discovery::builders::StartSmartHomeApplianceDiscoveryInputBuilder::default()
    }
}

/// A builder for [`StartSmartHomeApplianceDiscoveryInput`](crate::operation::start_smart_home_appliance_discovery::StartSmartHomeApplianceDiscoveryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartSmartHomeApplianceDiscoveryInputBuilder {
    pub(crate) room_arn: ::std::option::Option<::std::string::String>,
}
impl StartSmartHomeApplianceDiscoveryInputBuilder {
    /// <p>The room where smart home appliance discovery was initiated.</p>
    pub fn room_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.room_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The room where smart home appliance discovery was initiated.</p>
    pub fn set_room_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.room_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`StartSmartHomeApplianceDiscoveryInput`](crate::operation::start_smart_home_appliance_discovery::StartSmartHomeApplianceDiscoveryInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::start_smart_home_appliance_discovery::StartSmartHomeApplianceDiscoveryInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::start_smart_home_appliance_discovery::StartSmartHomeApplianceDiscoveryInput {
                room_arn: self.room_arn
                ,
            }
        )
    }
}
