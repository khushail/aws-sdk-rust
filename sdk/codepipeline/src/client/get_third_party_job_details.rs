// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetThirdPartyJobDetails`](crate::operation::get_third_party_job_details::builders::GetThirdPartyJobDetailsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_id(impl ::std::convert::Into<String>)`](crate::operation::get_third_party_job_details::builders::GetThirdPartyJobDetailsFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::get_third_party_job_details::builders::GetThirdPartyJobDetailsFluentBuilder::set_job_id): <p>The unique system-generated ID used for identifying the job.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::get_third_party_job_details::builders::GetThirdPartyJobDetailsFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::get_third_party_job_details::builders::GetThirdPartyJobDetailsFluentBuilder::set_client_token): <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    /// - On success, responds with [`GetThirdPartyJobDetailsOutput`](crate::operation::get_third_party_job_details::GetThirdPartyJobDetailsOutput) with field(s):
    ///   - [`job_details(Option<ThirdPartyJobDetails>)`](crate::operation::get_third_party_job_details::GetThirdPartyJobDetailsOutput::job_details): <p>The details of the job, including any protected values defined for the job.</p>
    /// - On failure, responds with [`SdkError<GetThirdPartyJobDetailsError>`](crate::operation::get_third_party_job_details::GetThirdPartyJobDetailsError)
    pub fn get_third_party_job_details(
        &self,
    ) -> crate::operation::get_third_party_job_details::builders::GetThirdPartyJobDetailsFluentBuilder
    {
        crate::operation::get_third_party_job_details::builders::GetThirdPartyJobDetailsFluentBuilder::new(self.handle.clone())
    }
}
