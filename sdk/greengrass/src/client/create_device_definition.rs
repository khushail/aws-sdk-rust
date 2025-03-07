// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDeviceDefinition`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`amzn_client_token(impl ::std::convert::Into<String>)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::amzn_client_token) / [`set_amzn_client_token(Option<String>)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::set_amzn_client_token): A client token used to correlate requests and responses.
    ///   - [`initial_version(DeviceDefinitionVersion)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::initial_version) / [`set_initial_version(Option<DeviceDefinitionVersion>)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::set_initial_version): Information about the initial version of the device definition.
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::set_name): The name of the device definition.
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::set_tags): Tag(s) to add to the new resource.
    /// - On success, responds with [`CreateDeviceDefinitionOutput`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput::arn): The ARN of the definition.
    ///   - [`creation_timestamp(Option<String>)`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the definition was created.
    ///   - [`id(Option<String>)`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput::id): The ID of the definition.
    ///   - [`last_updated_timestamp(Option<String>)`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput::last_updated_timestamp): The time, in milliseconds since the epoch, when the definition was last updated.
    ///   - [`latest_version(Option<String>)`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput::latest_version): The ID of the latest version associated with the definition.
    ///   - [`latest_version_arn(Option<String>)`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput::latest_version_arn): The ARN of the latest version associated with the definition.
    ///   - [`name(Option<String>)`](crate::operation::create_device_definition::CreateDeviceDefinitionOutput::name): The name of the definition.
    /// - On failure, responds with [`SdkError<CreateDeviceDefinitionError>`](crate::operation::create_device_definition::CreateDeviceDefinitionError)
    pub fn create_device_definition(
        &self,
    ) -> crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder
    {
        crate::operation::create_device_definition::builders::CreateDeviceDefinitionFluentBuilder::new(self.handle.clone())
    }
}
