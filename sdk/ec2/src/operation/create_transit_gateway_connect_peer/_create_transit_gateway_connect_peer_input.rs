// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTransitGatewayConnectPeerInput {
    /// <p>The ID of the Connect attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>The peer IP address (GRE outer IP address) on the transit gateway side of the Connect peer, which must be specified from a transit gateway CIDR block. If not specified, Amazon automatically assigns the first available IP address from the transit gateway CIDR block.</p>
    #[doc(hidden)]
    pub transit_gateway_address: ::std::option::Option<::std::string::String>,
    /// <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p>
    #[doc(hidden)]
    pub peer_address: ::std::option::Option<::std::string::String>,
    /// <p>The BGP options for the Connect peer.</p>
    #[doc(hidden)]
    pub bgp_options: ::std::option::Option<crate::types::TransitGatewayConnectRequestBgpOptions>,
    /// <p>The range of inside IP addresses that are used for BGP peering. You must specify a size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address from the range must be configured on the appliance as the BGP IP address. You can also optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code> range.</p>
    #[doc(hidden)]
    pub inside_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The tags to apply to the Connect peer.</p>
    #[doc(hidden)]
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl CreateTransitGatewayConnectPeerInput {
    /// <p>The ID of the Connect attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The peer IP address (GRE outer IP address) on the transit gateway side of the Connect peer, which must be specified from a transit gateway CIDR block. If not specified, Amazon automatically assigns the first available IP address from the transit gateway CIDR block.</p>
    pub fn transit_gateway_address(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_address.as_deref()
    }
    /// <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p>
    pub fn peer_address(&self) -> ::std::option::Option<&str> {
        self.peer_address.as_deref()
    }
    /// <p>The BGP options for the Connect peer.</p>
    pub fn bgp_options(
        &self,
    ) -> ::std::option::Option<&crate::types::TransitGatewayConnectRequestBgpOptions> {
        self.bgp_options.as_ref()
    }
    /// <p>The range of inside IP addresses that are used for BGP peering. You must specify a size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address from the range must be configured on the appliance as the BGP IP address. You can also optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code> range.</p>
    pub fn inside_cidr_blocks(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inside_cidr_blocks.as_deref()
    }
    /// <p>The tags to apply to the Connect peer.</p>
    pub fn tag_specifications(&self) -> ::std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateTransitGatewayConnectPeerInput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayConnectPeerInput`](crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerInput).
    pub fn builder() -> crate::operation::create_transit_gateway_connect_peer::builders::CreateTransitGatewayConnectPeerInputBuilder{
        crate::operation::create_transit_gateway_connect_peer::builders::CreateTransitGatewayConnectPeerInputBuilder::default()
    }
}

/// A builder for [`CreateTransitGatewayConnectPeerInput`](crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateTransitGatewayConnectPeerInputBuilder {
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_address: ::std::option::Option<::std::string::String>,
    pub(crate) peer_address: ::std::option::Option<::std::string::String>,
    pub(crate) bgp_options:
        ::std::option::Option<crate::types::TransitGatewayConnectRequestBgpOptions>,
    pub(crate) inside_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) tag_specifications:
        ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CreateTransitGatewayConnectPeerInputBuilder {
    /// <p>The ID of the Connect attachment.</p>
    pub fn transit_gateway_attachment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Connect attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the transit gateway side of the Connect peer, which must be specified from a transit gateway CIDR block. If not specified, Amazon automatically assigns the first available IP address from the transit gateway CIDR block.</p>
    pub fn transit_gateway_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the transit gateway side of the Connect peer, which must be specified from a transit gateway CIDR block. If not specified, Amazon automatically assigns the first available IP address from the transit gateway CIDR block.</p>
    pub fn set_transit_gateway_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_address = input;
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p>
    pub fn peer_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.peer_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p>
    pub fn set_peer_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.peer_address = input;
        self
    }
    /// <p>The BGP options for the Connect peer.</p>
    pub fn bgp_options(
        mut self,
        input: crate::types::TransitGatewayConnectRequestBgpOptions,
    ) -> Self {
        self.bgp_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The BGP options for the Connect peer.</p>
    pub fn set_bgp_options(
        mut self,
        input: ::std::option::Option<crate::types::TransitGatewayConnectRequestBgpOptions>,
    ) -> Self {
        self.bgp_options = input;
        self
    }
    /// Appends an item to `inside_cidr_blocks`.
    ///
    /// To override the contents of this collection use [`set_inside_cidr_blocks`](Self::set_inside_cidr_blocks).
    ///
    /// <p>The range of inside IP addresses that are used for BGP peering. You must specify a size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address from the range must be configured on the appliance as the BGP IP address. You can also optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code> range.</p>
    pub fn inside_cidr_blocks(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.inside_cidr_blocks.unwrap_or_default();
        v.push(input.into());
        self.inside_cidr_blocks = ::std::option::Option::Some(v);
        self
    }
    /// <p>The range of inside IP addresses that are used for BGP peering. You must specify a size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address from the range must be configured on the appliance as the BGP IP address. You can also optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code> range.</p>
    pub fn set_inside_cidr_blocks(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inside_cidr_blocks = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Connect peer.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the Connect peer.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
        self
    }
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
    /// Consumes the builder and constructs a [`CreateTransitGatewayConnectPeerInput`](crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerInput {
                transit_gateway_attachment_id: self.transit_gateway_attachment_id
                ,
                transit_gateway_address: self.transit_gateway_address
                ,
                peer_address: self.peer_address
                ,
                bgp_options: self.bgp_options
                ,
                inside_cidr_blocks: self.inside_cidr_blocks
                ,
                tag_specifications: self.tag_specifications
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}
