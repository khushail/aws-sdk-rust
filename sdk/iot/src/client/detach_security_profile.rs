// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DetachSecurityProfile`](crate::operation::detach_security_profile::builders::DetachSecurityProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`security_profile_name(impl ::std::convert::Into<String>)`](crate::operation::detach_security_profile::builders::DetachSecurityProfileFluentBuilder::security_profile_name) / [`set_security_profile_name(Option<String>)`](crate::operation::detach_security_profile::builders::DetachSecurityProfileFluentBuilder::set_security_profile_name): <p>The security profile that is detached.</p>
    ///   - [`security_profile_target_arn(impl ::std::convert::Into<String>)`](crate::operation::detach_security_profile::builders::DetachSecurityProfileFluentBuilder::security_profile_target_arn) / [`set_security_profile_target_arn(Option<String>)`](crate::operation::detach_security_profile::builders::DetachSecurityProfileFluentBuilder::set_security_profile_target_arn): <p>The ARN of the thing group from which the security profile is detached.</p>
    /// - On success, responds with [`DetachSecurityProfileOutput`](crate::operation::detach_security_profile::DetachSecurityProfileOutput)
    /// - On failure, responds with [`SdkError<DetachSecurityProfileError>`](crate::operation::detach_security_profile::DetachSecurityProfileError)
    pub fn detach_security_profile(
        &self,
    ) -> crate::operation::detach_security_profile::builders::DetachSecurityProfileFluentBuilder
    {
        crate::operation::detach_security_profile::builders::DetachSecurityProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
