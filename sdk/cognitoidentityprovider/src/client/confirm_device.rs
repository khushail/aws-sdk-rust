// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ConfirmDevice`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`access_token(impl ::std::convert::Into<String>)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::access_token) / [`set_access_token(Option<String>)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::set_access_token): <p>A valid access token that Amazon Cognito issued to the user whose device you want to confirm.</p>
    ///   - [`device_key(impl ::std::convert::Into<String>)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::device_key) / [`set_device_key(Option<String>)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::set_device_key): <p>The device key.</p>
    ///   - [`device_secret_verifier_config(DeviceSecretVerifierConfigType)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::device_secret_verifier_config) / [`set_device_secret_verifier_config(Option<DeviceSecretVerifierConfigType>)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::set_device_secret_verifier_config): <p>The configuration of the device secret verifier.</p>
    ///   - [`device_name(impl ::std::convert::Into<String>)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::device_name) / [`set_device_name(Option<String>)`](crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::set_device_name): <p>The device name.</p>
    /// - On success, responds with [`ConfirmDeviceOutput`](crate::operation::confirm_device::ConfirmDeviceOutput) with field(s):
    ///   - [`user_confirmation_necessary(bool)`](crate::operation::confirm_device::ConfirmDeviceOutput::user_confirmation_necessary): <p>Indicates whether the user confirmation must confirm the device response.</p>
    /// - On failure, responds with [`SdkError<ConfirmDeviceError>`](crate::operation::confirm_device::ConfirmDeviceError)
    pub fn confirm_device(
        &self,
    ) -> crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder {
        crate::operation::confirm_device::builders::ConfirmDeviceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
