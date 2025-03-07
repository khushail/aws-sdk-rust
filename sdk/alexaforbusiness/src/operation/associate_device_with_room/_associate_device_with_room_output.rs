// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateDeviceWithRoomOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for AssociateDeviceWithRoomOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociateDeviceWithRoomOutput {
    /// Creates a new builder-style object to manufacture [`AssociateDeviceWithRoomOutput`](crate::operation::associate_device_with_room::AssociateDeviceWithRoomOutput).
    pub fn builder(
    ) -> crate::operation::associate_device_with_room::builders::AssociateDeviceWithRoomOutputBuilder
    {
        crate::operation::associate_device_with_room::builders::AssociateDeviceWithRoomOutputBuilder::default()
    }
}

/// A builder for [`AssociateDeviceWithRoomOutput`](crate::operation::associate_device_with_room::AssociateDeviceWithRoomOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateDeviceWithRoomOutputBuilder {
    _request_id: Option<String>,
}
impl AssociateDeviceWithRoomOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AssociateDeviceWithRoomOutput`](crate::operation::associate_device_with_room::AssociateDeviceWithRoomOutput).
    pub fn build(
        self,
    ) -> crate::operation::associate_device_with_room::AssociateDeviceWithRoomOutput {
        crate::operation::associate_device_with_room::AssociateDeviceWithRoomOutput {
            _request_id: self._request_id,
        }
    }
}
