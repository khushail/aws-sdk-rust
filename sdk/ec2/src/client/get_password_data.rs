// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPasswordData`](crate::operation::get_password_data::builders::GetPasswordDataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::get_password_data::builders::GetPasswordDataFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::get_password_data::builders::GetPasswordDataFluentBuilder::set_instance_id): <p>The ID of the Windows instance.</p>
    ///   - [`dry_run(bool)`](crate::operation::get_password_data::builders::GetPasswordDataFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_password_data::builders::GetPasswordDataFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`GetPasswordDataOutput`](crate::operation::get_password_data::GetPasswordDataOutput) with field(s):
    ///   - [`instance_id(Option<String>)`](crate::operation::get_password_data::GetPasswordDataOutput::instance_id): <p>The ID of the Windows instance.</p>
    ///   - [`password_data(Option<String>)`](crate::operation::get_password_data::GetPasswordDataOutput::password_data): <p>The password of the instance. Returns an empty string if the password is not available.</p>
    ///   - [`timestamp(Option<DateTime>)`](crate::operation::get_password_data::GetPasswordDataOutput::timestamp): <p>The time the data was last updated.</p>
    /// - On failure, responds with [`SdkError<GetPasswordDataError>`](crate::operation::get_password_data::GetPasswordDataError)
    pub fn get_password_data(
        &self,
    ) -> crate::operation::get_password_data::builders::GetPasswordDataFluentBuilder {
        crate::operation::get_password_data::builders::GetPasswordDataFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
