// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchAssociateClientDeviceWithCoreDeviceOutput {
    /// <p>The list of any errors for the entries in the request. Each error entry contains the name of the IoT thing that failed to associate.</p>
    #[doc(hidden)]
    pub error_entries: ::std::option::Option<
        ::std::vec::Vec<crate::types::AssociateClientDeviceWithCoreDeviceErrorEntry>,
    >,
    _request_id: Option<String>,
}
impl BatchAssociateClientDeviceWithCoreDeviceOutput {
    /// <p>The list of any errors for the entries in the request. Each error entry contains the name of the IoT thing that failed to associate.</p>
    pub fn error_entries(
        &self,
    ) -> ::std::option::Option<&[crate::types::AssociateClientDeviceWithCoreDeviceErrorEntry]> {
        self.error_entries.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchAssociateClientDeviceWithCoreDeviceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchAssociateClientDeviceWithCoreDeviceOutput {
    /// Creates a new builder-style object to manufacture [`BatchAssociateClientDeviceWithCoreDeviceOutput`](crate::operation::batch_associate_client_device_with_core_device::BatchAssociateClientDeviceWithCoreDeviceOutput).
    pub fn builder() -> crate::operation::batch_associate_client_device_with_core_device::builders::BatchAssociateClientDeviceWithCoreDeviceOutputBuilder{
        crate::operation::batch_associate_client_device_with_core_device::builders::BatchAssociateClientDeviceWithCoreDeviceOutputBuilder::default()
    }
}

/// A builder for [`BatchAssociateClientDeviceWithCoreDeviceOutput`](crate::operation::batch_associate_client_device_with_core_device::BatchAssociateClientDeviceWithCoreDeviceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchAssociateClientDeviceWithCoreDeviceOutputBuilder {
    pub(crate) error_entries: ::std::option::Option<
        ::std::vec::Vec<crate::types::AssociateClientDeviceWithCoreDeviceErrorEntry>,
    >,
    _request_id: Option<String>,
}
impl BatchAssociateClientDeviceWithCoreDeviceOutputBuilder {
    /// Appends an item to `error_entries`.
    ///
    /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
    ///
    /// <p>The list of any errors for the entries in the request. Each error entry contains the name of the IoT thing that failed to associate.</p>
    pub fn error_entries(
        mut self,
        input: crate::types::AssociateClientDeviceWithCoreDeviceErrorEntry,
    ) -> Self {
        let mut v = self.error_entries.unwrap_or_default();
        v.push(input);
        self.error_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of any errors for the entries in the request. Each error entry contains the name of the IoT thing that failed to associate.</p>
    pub fn set_error_entries(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::AssociateClientDeviceWithCoreDeviceErrorEntry>,
        >,
    ) -> Self {
        self.error_entries = input;
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
    /// Consumes the builder and constructs a [`BatchAssociateClientDeviceWithCoreDeviceOutput`](crate::operation::batch_associate_client_device_with_core_device::BatchAssociateClientDeviceWithCoreDeviceOutput).
    pub fn build(self) -> crate::operation::batch_associate_client_device_with_core_device::BatchAssociateClientDeviceWithCoreDeviceOutput{
        crate::operation::batch_associate_client_device_with_core_device::BatchAssociateClientDeviceWithCoreDeviceOutput {
            error_entries: self.error_entries
            ,
            _request_id: self._request_id,
        }
    }
}
