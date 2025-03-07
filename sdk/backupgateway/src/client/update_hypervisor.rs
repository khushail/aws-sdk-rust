// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateHypervisor`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hypervisor_arn(impl ::std::convert::Into<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::hypervisor_arn) / [`set_hypervisor_arn(Option<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::set_hypervisor_arn): <p>The Amazon Resource Name (ARN) of the hypervisor to update.</p>
    ///   - [`host(impl ::std::convert::Into<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::host) / [`set_host(Option<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::set_host): <p>The updated host of the hypervisor. This can be either an IP address or a fully-qualified domain name (FQDN).</p>
    ///   - [`username(impl ::std::convert::Into<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::username) / [`set_username(Option<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::set_username): <p>The updated username for the hypervisor.</p>
    ///   - [`password(impl ::std::convert::Into<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::password) / [`set_password(Option<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::set_password): <p>The updated password for the hypervisor.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::set_name): <p>The updated name for the hypervisor</p>
    ///   - [`log_group_arn(impl ::std::convert::Into<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::log_group_arn) / [`set_log_group_arn(Option<String>)`](crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::set_log_group_arn): <p>The Amazon Resource Name (ARN) of the group of gateways within the requested log.</p>
    /// - On success, responds with [`UpdateHypervisorOutput`](crate::operation::update_hypervisor::UpdateHypervisorOutput) with field(s):
    ///   - [`hypervisor_arn(Option<String>)`](crate::operation::update_hypervisor::UpdateHypervisorOutput::hypervisor_arn): <p>The Amazon Resource Name (ARN) of the hypervisor you updated.</p>
    /// - On failure, responds with [`SdkError<UpdateHypervisorError>`](crate::operation::update_hypervisor::UpdateHypervisorError)
    pub fn update_hypervisor(
        &self,
    ) -> crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder {
        crate::operation::update_hypervisor::builders::UpdateHypervisorFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
