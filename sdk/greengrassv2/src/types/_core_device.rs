// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a Greengrass core device, which is an IoT thing that runs the IoT Greengrass Core software.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CoreDevice {
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    #[doc(hidden)]
    pub core_device_thing_name: ::std::option::Option<::std::string::String>,
    /// <p>The status of the core device. Core devices can have the following statuses:</p>
    /// <ul>
    /// <li> <p> <code>HEALTHY</code> – The IoT Greengrass Core software and all components run on the core device without issue.</p> </li>
    /// <li> <p> <code>UNHEALTHY</code> – The IoT Greengrass Core software or a component is in a failed state on the core device.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::CoreDeviceStatus>,
    /// <p>The time at which the core device's status last updated, expressed in ISO 8601 format.</p>
    #[doc(hidden)]
    pub last_status_update_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CoreDevice {
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn core_device_thing_name(&self) -> ::std::option::Option<&str> {
        self.core_device_thing_name.as_deref()
    }
    /// <p>The status of the core device. Core devices can have the following statuses:</p>
    /// <ul>
    /// <li> <p> <code>HEALTHY</code> – The IoT Greengrass Core software and all components run on the core device without issue.</p> </li>
    /// <li> <p> <code>UNHEALTHY</code> – The IoT Greengrass Core software or a component is in a failed state on the core device.</p> </li>
    /// </ul>
    pub fn status(&self) -> ::std::option::Option<&crate::types::CoreDeviceStatus> {
        self.status.as_ref()
    }
    /// <p>The time at which the core device's status last updated, expressed in ISO 8601 format.</p>
    pub fn last_status_update_timestamp(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_status_update_timestamp.as_ref()
    }
}
impl CoreDevice {
    /// Creates a new builder-style object to manufacture [`CoreDevice`](crate::types::CoreDevice).
    pub fn builder() -> crate::types::builders::CoreDeviceBuilder {
        crate::types::builders::CoreDeviceBuilder::default()
    }
}

/// A builder for [`CoreDevice`](crate::types::CoreDevice).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CoreDeviceBuilder {
    pub(crate) core_device_thing_name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::CoreDeviceStatus>,
    pub(crate) last_status_update_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CoreDeviceBuilder {
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn core_device_thing_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.core_device_thing_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn set_core_device_thing_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.core_device_thing_name = input;
        self
    }
    /// <p>The status of the core device. Core devices can have the following statuses:</p>
    /// <ul>
    /// <li> <p> <code>HEALTHY</code> – The IoT Greengrass Core software and all components run on the core device without issue.</p> </li>
    /// <li> <p> <code>UNHEALTHY</code> – The IoT Greengrass Core software or a component is in a failed state on the core device.</p> </li>
    /// </ul>
    pub fn status(mut self, input: crate::types::CoreDeviceStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the core device. Core devices can have the following statuses:</p>
    /// <ul>
    /// <li> <p> <code>HEALTHY</code> – The IoT Greengrass Core software and all components run on the core device without issue.</p> </li>
    /// <li> <p> <code>UNHEALTHY</code> – The IoT Greengrass Core software or a component is in a failed state on the core device.</p> </li>
    /// </ul>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::CoreDeviceStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The time at which the core device's status last updated, expressed in ISO 8601 format.</p>
    pub fn last_status_update_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_status_update_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the core device's status last updated, expressed in ISO 8601 format.</p>
    pub fn set_last_status_update_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_status_update_timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`CoreDevice`](crate::types::CoreDevice).
    pub fn build(self) -> crate::types::CoreDevice {
        crate::types::CoreDevice {
            core_device_thing_name: self.core_device_thing_name,
            status: self.status,
            last_status_update_timestamp: self.last_status_update_timestamp,
        }
    }
}
