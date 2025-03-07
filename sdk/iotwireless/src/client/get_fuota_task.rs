// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetFuotaTask`](crate::operation::get_fuota_task::builders::GetFuotaTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_fuota_task::builders::GetFuotaTaskFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_fuota_task::builders::GetFuotaTaskFluentBuilder::set_id): <p>The ID of a FUOTA task.</p>
    /// - On success, responds with [`GetFuotaTaskOutput`](crate::operation::get_fuota_task::GetFuotaTaskOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::arn): <p>The arn of a FUOTA task.</p>
    ///   - [`id(Option<String>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::id): <p>The ID of a FUOTA task.</p>
    ///   - [`status(Option<FuotaTaskStatus>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::status): <p>The status of a FUOTA task.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::name): <p>The name of a FUOTA task.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::description): <p>The description of the new resource.</p>
    ///   - [`lo_ra_wan(Option<LoRaWanFuotaTaskGetInfo>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::lo_ra_wan): <p>The LoRaWAN information returned from getting a FUOTA task.</p>
    ///   - [`firmware_update_image(Option<String>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::firmware_update_image): <p>The S3 URI points to a firmware update image that is to be used with a FUOTA task.</p>
    ///   - [`firmware_update_role(Option<String>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::firmware_update_role): <p>The firmware update role that is to be used with a FUOTA task.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::created_at): <p>Created at timestamp for the resource.</p>
    ///   - [`redundancy_percent(Option<i32>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::redundancy_percent): <p>The percentage of added redundant fragments. For example, if firmware file is 100 bytes and fragment size is 10 bytes, with <code>RedundancyPercent</code> set to 50(%), the final number of encoded fragments is (100 / 10) + (100 / 10 * 50%) = 15.</p>
    ///   - [`fragment_size_bytes(Option<i32>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::fragment_size_bytes): <p>The size of each fragment in bytes. Currently only supported in fuota tasks with multicast groups.</p>
    ///   - [`fragment_interval_ms(Option<i32>)`](crate::operation::get_fuota_task::GetFuotaTaskOutput::fragment_interval_ms): <p>The interval of sending fragments in milliseconds. Currently the interval will be rounded to the nearest second. Note that this interval only controls the timing when the cloud sends the fragments down. The actual delay of receiving fragments at device side depends on the device's class and the communication delay with the cloud.</p>
    /// - On failure, responds with [`SdkError<GetFuotaTaskError>`](crate::operation::get_fuota_task::GetFuotaTaskError)
    pub fn get_fuota_task(
        &self,
    ) -> crate::operation::get_fuota_task::builders::GetFuotaTaskFluentBuilder {
        crate::operation::get_fuota_task::builders::GetFuotaTaskFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
