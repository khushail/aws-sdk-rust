// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDeviceDefinitionVersion`](crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`amzn_client_token(impl ::std::convert::Into<String>)`](crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder::amzn_client_token) / [`set_amzn_client_token(Option<String>)`](crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder::set_amzn_client_token): A client token used to correlate requests and responses.
    ///   - [`device_definition_id(impl ::std::convert::Into<String>)`](crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder::device_definition_id) / [`set_device_definition_id(Option<String>)`](crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder::set_device_definition_id): The ID of the device definition.
    ///   - [`devices(Vec<Device>)`](crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder::devices) / [`set_devices(Option<Vec<Device>>)`](crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder::set_devices): A list of devices in the definition version.
    /// - On success, responds with [`CreateDeviceDefinitionVersionOutput`](crate::operation::create_device_definition_version::CreateDeviceDefinitionVersionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_device_definition_version::CreateDeviceDefinitionVersionOutput::arn): The ARN of the version.
    ///   - [`creation_timestamp(Option<String>)`](crate::operation::create_device_definition_version::CreateDeviceDefinitionVersionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the version was created.
    ///   - [`id(Option<String>)`](crate::operation::create_device_definition_version::CreateDeviceDefinitionVersionOutput::id): The ID of the parent definition that the version is associated with.
    ///   - [`version(Option<String>)`](crate::operation::create_device_definition_version::CreateDeviceDefinitionVersionOutput::version): The ID of the version.
    /// - On failure, responds with [`SdkError<CreateDeviceDefinitionVersionError>`](crate::operation::create_device_definition_version::CreateDeviceDefinitionVersionError)
    pub fn create_device_definition_version(&self) -> crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder{
        crate::operation::create_device_definition_version::builders::CreateDeviceDefinitionVersionFluentBuilder::new(self.handle.clone())
    }
}
