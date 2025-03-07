// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLicenseConversionTask`](crate::operation::get_license_conversion_task::builders::GetLicenseConversionTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`license_conversion_task_id(impl ::std::convert::Into<String>)`](crate::operation::get_license_conversion_task::builders::GetLicenseConversionTaskFluentBuilder::license_conversion_task_id) / [`set_license_conversion_task_id(Option<String>)`](crate::operation::get_license_conversion_task::builders::GetLicenseConversionTaskFluentBuilder::set_license_conversion_task_id): <p>ID of the license type conversion task to retrieve information on.</p>
    /// - On success, responds with [`GetLicenseConversionTaskOutput`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput) with field(s):
    ///   - [`license_conversion_task_id(Option<String>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::license_conversion_task_id): <p>ID of the license type conversion task.</p>
    ///   - [`resource_arn(Option<String>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::resource_arn): <p>Amazon Resource Names (ARN) of the resources the license conversion task is associated with.</p>
    ///   - [`source_license_context(Option<LicenseConversionContext>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::source_license_context): <p>Information about the license type converted from.</p>
    ///   - [`destination_license_context(Option<LicenseConversionContext>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::destination_license_context): <p>Information about the license type converted to.</p>
    ///   - [`status_message(Option<String>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::status_message): <p>The status message for the conversion task.</p>
    ///   - [`status(Option<LicenseConversionTaskStatus>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::status): <p>Status of the license type conversion task.</p>
    ///   - [`start_time(Option<DateTime>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::start_time): <p>Time at which the license type conversion task was started .</p>
    ///   - [`license_conversion_time(Option<DateTime>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::license_conversion_time): <p>Amount of time to complete the license type conversion.</p>
    ///   - [`end_time(Option<DateTime>)`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskOutput::end_time): <p>Time at which the license type conversion task was completed.</p>
    /// - On failure, responds with [`SdkError<GetLicenseConversionTaskError>`](crate::operation::get_license_conversion_task::GetLicenseConversionTaskError)
    pub fn get_license_conversion_task(&self) -> crate::operation::get_license_conversion_task::builders::GetLicenseConversionTaskFluentBuilder{
        crate::operation::get_license_conversion_task::builders::GetLicenseConversionTaskFluentBuilder::new(self.handle.clone())
    }
}
