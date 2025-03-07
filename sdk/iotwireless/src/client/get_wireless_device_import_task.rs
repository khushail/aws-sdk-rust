// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetWirelessDeviceImportTask`](crate::operation::get_wireless_device_import_task::builders::GetWirelessDeviceImportTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_wireless_device_import_task::builders::GetWirelessDeviceImportTaskFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_wireless_device_import_task::builders::GetWirelessDeviceImportTaskFluentBuilder::set_id): <p>The identifier of the import task for which information is requested.</p>
    /// - On success, responds with [`GetWirelessDeviceImportTaskOutput`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::id): <p>The identifier of the import task for which information is retrieved.</p>
    ///   - [`arn(Option<String>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::arn): <p>The ARN (Amazon Resource Name) of the import task.</p>
    ///   - [`destination_name(Option<String>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::destination_name): <p>The name of the destination that's assigned to the wireless devices in the import task.</p>
    ///   - [`sidewalk(Option<SidewalkGetStartImportInfo>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::sidewalk): <p>The Sidewalk-related information about an import task.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::creation_time): <p>The time at which the import task was created.</p>
    ///   - [`status(Option<ImportTaskStatus>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::status): <p>The import task status.</p>
    ///   - [`status_reason(Option<String>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::status_reason): <p>The reason for the provided status information, such as a validation error that causes the import task to fail.</p>
    ///   - [`initialized_imported_device_count(Option<i64>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::initialized_imported_device_count): <p>The number of devices in the import task that are waiting for the control log to start processing.</p>
    ///   - [`pending_imported_device_count(Option<i64>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::pending_imported_device_count): <p>The number of devices in the import task that are waiting in the import task queue to be onboarded.</p>
    ///   - [`onboarded_imported_device_count(Option<i64>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::onboarded_imported_device_count): <p>The number of devices in the import task that have been onboarded to the import task.</p>
    ///   - [`failed_imported_device_count(Option<i64>)`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskOutput::failed_imported_device_count): <p>The number of devices in the import task that failed to onboard to the import task.</p>
    /// - On failure, responds with [`SdkError<GetWirelessDeviceImportTaskError>`](crate::operation::get_wireless_device_import_task::GetWirelessDeviceImportTaskError)
    pub fn get_wireless_device_import_task(&self) -> crate::operation::get_wireless_device_import_task::builders::GetWirelessDeviceImportTaskFluentBuilder{
        crate::operation::get_wireless_device_import_task::builders::GetWirelessDeviceImportTaskFluentBuilder::new(self.handle.clone())
    }
}
