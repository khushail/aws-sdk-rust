// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSignalingChannelEndpointOutput {
    /// <p>A list of endpoints for the specified signaling channel.</p>
    #[doc(hidden)]
    pub resource_endpoint_list:
        ::std::option::Option<::std::vec::Vec<crate::types::ResourceEndpointListItem>>,
    _request_id: Option<String>,
}
impl GetSignalingChannelEndpointOutput {
    /// <p>A list of endpoints for the specified signaling channel.</p>
    pub fn resource_endpoint_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::ResourceEndpointListItem]> {
        self.resource_endpoint_list.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetSignalingChannelEndpointOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSignalingChannelEndpointOutput {
    /// Creates a new builder-style object to manufacture [`GetSignalingChannelEndpointOutput`](crate::operation::get_signaling_channel_endpoint::GetSignalingChannelEndpointOutput).
    pub fn builder() -> crate::operation::get_signaling_channel_endpoint::builders::GetSignalingChannelEndpointOutputBuilder{
        crate::operation::get_signaling_channel_endpoint::builders::GetSignalingChannelEndpointOutputBuilder::default()
    }
}

/// A builder for [`GetSignalingChannelEndpointOutput`](crate::operation::get_signaling_channel_endpoint::GetSignalingChannelEndpointOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetSignalingChannelEndpointOutputBuilder {
    pub(crate) resource_endpoint_list:
        ::std::option::Option<::std::vec::Vec<crate::types::ResourceEndpointListItem>>,
    _request_id: Option<String>,
}
impl GetSignalingChannelEndpointOutputBuilder {
    /// Appends an item to `resource_endpoint_list`.
    ///
    /// To override the contents of this collection use [`set_resource_endpoint_list`](Self::set_resource_endpoint_list).
    ///
    /// <p>A list of endpoints for the specified signaling channel.</p>
    pub fn resource_endpoint_list(mut self, input: crate::types::ResourceEndpointListItem) -> Self {
        let mut v = self.resource_endpoint_list.unwrap_or_default();
        v.push(input);
        self.resource_endpoint_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of endpoints for the specified signaling channel.</p>
    pub fn set_resource_endpoint_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceEndpointListItem>>,
    ) -> Self {
        self.resource_endpoint_list = input;
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
    /// Consumes the builder and constructs a [`GetSignalingChannelEndpointOutput`](crate::operation::get_signaling_channel_endpoint::GetSignalingChannelEndpointOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_signaling_channel_endpoint::GetSignalingChannelEndpointOutput {
        crate::operation::get_signaling_channel_endpoint::GetSignalingChannelEndpointOutput {
            resource_endpoint_list: self.resource_endpoint_list,
            _request_id: self._request_id,
        }
    }
}
