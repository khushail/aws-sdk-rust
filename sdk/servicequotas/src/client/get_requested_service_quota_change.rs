// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRequestedServiceQuotaChange`](crate::operation::get_requested_service_quota_change::builders::GetRequestedServiceQuotaChangeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`request_id(impl ::std::convert::Into<String>)`](crate::operation::get_requested_service_quota_change::builders::GetRequestedServiceQuotaChangeFluentBuilder::request_id) / [`set_request_id(Option<String>)`](crate::operation::get_requested_service_quota_change::builders::GetRequestedServiceQuotaChangeFluentBuilder::set_request_id): <p>The ID of the quota increase request.</p>
    /// - On success, responds with [`GetRequestedServiceQuotaChangeOutput`](crate::operation::get_requested_service_quota_change::GetRequestedServiceQuotaChangeOutput) with field(s):
    ///   - [`requested_quota(Option<RequestedServiceQuotaChange>)`](crate::operation::get_requested_service_quota_change::GetRequestedServiceQuotaChangeOutput::requested_quota): <p>Information about the quota increase request.</p>
    /// - On failure, responds with [`SdkError<GetRequestedServiceQuotaChangeError>`](crate::operation::get_requested_service_quota_change::GetRequestedServiceQuotaChangeError)
    pub fn get_requested_service_quota_change(&self) -> crate::operation::get_requested_service_quota_change::builders::GetRequestedServiceQuotaChangeFluentBuilder{
        crate::operation::get_requested_service_quota_change::builders::GetRequestedServiceQuotaChangeFluentBuilder::new(self.handle.clone())
    }
}
