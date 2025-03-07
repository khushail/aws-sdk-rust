// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specify the list of gateways to which you want to send downlink data traffic when the wireless device is running in class B or class C mode.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ParticipatingGateways {
    /// <p>Indicates whether to send the downlink message in sequential mode or concurrent mode, or to use only the chosen gateways from the previous uplink message transmission.</p>
    #[doc(hidden)]
    pub downlink_mode: ::std::option::Option<crate::types::DownlinkMode>,
    /// <p>The list of gateways that you want to use for sending the downlink data traffic.</p>
    #[doc(hidden)]
    pub gateway_list: ::std::option::Option<::std::vec::Vec<crate::types::GatewayListItem>>,
    /// <p>The duration of time for which AWS IoT Core for LoRaWAN will wait before transmitting the payload to the next gateway.</p>
    #[doc(hidden)]
    pub transmission_interval: ::std::option::Option<i32>,
}
impl ParticipatingGateways {
    /// <p>Indicates whether to send the downlink message in sequential mode or concurrent mode, or to use only the chosen gateways from the previous uplink message transmission.</p>
    pub fn downlink_mode(&self) -> ::std::option::Option<&crate::types::DownlinkMode> {
        self.downlink_mode.as_ref()
    }
    /// <p>The list of gateways that you want to use for sending the downlink data traffic.</p>
    pub fn gateway_list(&self) -> ::std::option::Option<&[crate::types::GatewayListItem]> {
        self.gateway_list.as_deref()
    }
    /// <p>The duration of time for which AWS IoT Core for LoRaWAN will wait before transmitting the payload to the next gateway.</p>
    pub fn transmission_interval(&self) -> ::std::option::Option<i32> {
        self.transmission_interval
    }
}
impl ParticipatingGateways {
    /// Creates a new builder-style object to manufacture [`ParticipatingGateways`](crate::types::ParticipatingGateways).
    pub fn builder() -> crate::types::builders::ParticipatingGatewaysBuilder {
        crate::types::builders::ParticipatingGatewaysBuilder::default()
    }
}

/// A builder for [`ParticipatingGateways`](crate::types::ParticipatingGateways).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ParticipatingGatewaysBuilder {
    pub(crate) downlink_mode: ::std::option::Option<crate::types::DownlinkMode>,
    pub(crate) gateway_list: ::std::option::Option<::std::vec::Vec<crate::types::GatewayListItem>>,
    pub(crate) transmission_interval: ::std::option::Option<i32>,
}
impl ParticipatingGatewaysBuilder {
    /// <p>Indicates whether to send the downlink message in sequential mode or concurrent mode, or to use only the chosen gateways from the previous uplink message transmission.</p>
    pub fn downlink_mode(mut self, input: crate::types::DownlinkMode) -> Self {
        self.downlink_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to send the downlink message in sequential mode or concurrent mode, or to use only the chosen gateways from the previous uplink message transmission.</p>
    pub fn set_downlink_mode(
        mut self,
        input: ::std::option::Option<crate::types::DownlinkMode>,
    ) -> Self {
        self.downlink_mode = input;
        self
    }
    /// Appends an item to `gateway_list`.
    ///
    /// To override the contents of this collection use [`set_gateway_list`](Self::set_gateway_list).
    ///
    /// <p>The list of gateways that you want to use for sending the downlink data traffic.</p>
    pub fn gateway_list(mut self, input: crate::types::GatewayListItem) -> Self {
        let mut v = self.gateway_list.unwrap_or_default();
        v.push(input);
        self.gateway_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of gateways that you want to use for sending the downlink data traffic.</p>
    pub fn set_gateway_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::GatewayListItem>>,
    ) -> Self {
        self.gateway_list = input;
        self
    }
    /// <p>The duration of time for which AWS IoT Core for LoRaWAN will wait before transmitting the payload to the next gateway.</p>
    pub fn transmission_interval(mut self, input: i32) -> Self {
        self.transmission_interval = ::std::option::Option::Some(input);
        self
    }
    /// <p>The duration of time for which AWS IoT Core for LoRaWAN will wait before transmitting the payload to the next gateway.</p>
    pub fn set_transmission_interval(mut self, input: ::std::option::Option<i32>) -> Self {
        self.transmission_interval = input;
        self
    }
    /// Consumes the builder and constructs a [`ParticipatingGateways`](crate::types::ParticipatingGateways).
    pub fn build(self) -> crate::types::ParticipatingGateways {
        crate::types::ParticipatingGateways {
            downlink_mode: self.downlink_mode,
            gateway_list: self.gateway_list,
            transmission_interval: self.transmission_interval,
        }
    }
}
