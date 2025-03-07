// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartDeviceSyncOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for StartDeviceSyncOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartDeviceSyncOutput {
    /// Creates a new builder-style object to manufacture [`StartDeviceSyncOutput`](crate::operation::start_device_sync::StartDeviceSyncOutput).
    pub fn builder() -> crate::operation::start_device_sync::builders::StartDeviceSyncOutputBuilder
    {
        crate::operation::start_device_sync::builders::StartDeviceSyncOutputBuilder::default()
    }
}

/// A builder for [`StartDeviceSyncOutput`](crate::operation::start_device_sync::StartDeviceSyncOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartDeviceSyncOutputBuilder {
    _request_id: Option<String>,
}
impl StartDeviceSyncOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartDeviceSyncOutput`](crate::operation::start_device_sync::StartDeviceSyncOutput).
    pub fn build(self) -> crate::operation::start_device_sync::StartDeviceSyncOutput {
        crate::operation::start_device_sync::StartDeviceSyncOutput {
            _request_id: self._request_id,
        }
    }
}
