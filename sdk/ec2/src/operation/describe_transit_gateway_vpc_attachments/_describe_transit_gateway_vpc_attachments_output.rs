// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTransitGatewayVpcAttachmentsOutput {
    /// <p>Information about the VPC attachments.</p>
    #[doc(hidden)]
    pub transit_gateway_vpc_attachments:
        ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayVpcAttachment>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTransitGatewayVpcAttachmentsOutput {
    /// <p>Information about the VPC attachments.</p>
    pub fn transit_gateway_vpc_attachments(
        &self,
    ) -> ::std::option::Option<&[crate::types::TransitGatewayVpcAttachment]> {
        self.transit_gateway_vpc_attachments.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeTransitGatewayVpcAttachmentsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTransitGatewayVpcAttachmentsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTransitGatewayVpcAttachmentsOutput`](crate::operation::describe_transit_gateway_vpc_attachments::DescribeTransitGatewayVpcAttachmentsOutput).
    pub fn builder() -> crate::operation::describe_transit_gateway_vpc_attachments::builders::DescribeTransitGatewayVpcAttachmentsOutputBuilder{
        crate::operation::describe_transit_gateway_vpc_attachments::builders::DescribeTransitGatewayVpcAttachmentsOutputBuilder::default()
    }
}

/// A builder for [`DescribeTransitGatewayVpcAttachmentsOutput`](crate::operation::describe_transit_gateway_vpc_attachments::DescribeTransitGatewayVpcAttachmentsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTransitGatewayVpcAttachmentsOutputBuilder {
    pub(crate) transit_gateway_vpc_attachments:
        ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayVpcAttachment>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTransitGatewayVpcAttachmentsOutputBuilder {
    /// Appends an item to `transit_gateway_vpc_attachments`.
    ///
    /// To override the contents of this collection use [`set_transit_gateway_vpc_attachments`](Self::set_transit_gateway_vpc_attachments).
    ///
    /// <p>Information about the VPC attachments.</p>
    pub fn transit_gateway_vpc_attachments(
        mut self,
        input: crate::types::TransitGatewayVpcAttachment,
    ) -> Self {
        let mut v = self.transit_gateway_vpc_attachments.unwrap_or_default();
        v.push(input);
        self.transit_gateway_vpc_attachments = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the VPC attachments.</p>
    pub fn set_transit_gateway_vpc_attachments(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayVpcAttachment>>,
    ) -> Self {
        self.transit_gateway_vpc_attachments = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribeTransitGatewayVpcAttachmentsOutput`](crate::operation::describe_transit_gateway_vpc_attachments::DescribeTransitGatewayVpcAttachmentsOutput).
    pub fn build(self) -> crate::operation::describe_transit_gateway_vpc_attachments::DescribeTransitGatewayVpcAttachmentsOutput{
        crate::operation::describe_transit_gateway_vpc_attachments::DescribeTransitGatewayVpcAttachmentsOutput {
            transit_gateway_vpc_attachments: self.transit_gateway_vpc_attachments
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
