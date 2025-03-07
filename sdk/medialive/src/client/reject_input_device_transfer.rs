// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RejectInputDeviceTransfer`](crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`input_device_id(impl ::std::convert::Into<String>)`](crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferFluentBuilder::input_device_id) / [`set_input_device_id(Option<String>)`](crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferFluentBuilder::set_input_device_id): The unique ID of the input device to reject. For example, hd-123456789abcdef.
    /// - On success, responds with [`RejectInputDeviceTransferOutput`](crate::operation::reject_input_device_transfer::RejectInputDeviceTransferOutput)
    /// - On failure, responds with [`SdkError<RejectInputDeviceTransferError>`](crate::operation::reject_input_device_transfer::RejectInputDeviceTransferError)
    pub fn reject_input_device_transfer(&self) -> crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferFluentBuilder{
        crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferFluentBuilder::new(self.handle.clone())
    }
}
