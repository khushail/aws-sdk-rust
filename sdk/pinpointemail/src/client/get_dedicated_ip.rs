// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDedicatedIp`](crate::operation::get_dedicated_ip::builders::GetDedicatedIpFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ip(impl ::std::convert::Into<String>)`](crate::operation::get_dedicated_ip::builders::GetDedicatedIpFluentBuilder::ip) / [`set_ip(Option<String>)`](crate::operation::get_dedicated_ip::builders::GetDedicatedIpFluentBuilder::set_ip): <p>The IP address that you want to obtain more information about. The value you specify has to be a dedicated IP address that's assocaited with your Amazon Pinpoint account.</p>
    /// - On success, responds with [`GetDedicatedIpOutput`](crate::operation::get_dedicated_ip::GetDedicatedIpOutput) with field(s):
    ///   - [`dedicated_ip(Option<DedicatedIp>)`](crate::operation::get_dedicated_ip::GetDedicatedIpOutput::dedicated_ip): <p>An object that contains information about a dedicated IP address.</p>
    /// - On failure, responds with [`SdkError<GetDedicatedIpError>`](crate::operation::get_dedicated_ip::GetDedicatedIpError)
    pub fn get_dedicated_ip(
        &self,
    ) -> crate::operation::get_dedicated_ip::builders::GetDedicatedIpFluentBuilder {
        crate::operation::get_dedicated_ip::builders::GetDedicatedIpFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
