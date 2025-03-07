// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateVPCEConfiguration`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to update.</p>
    ///   - [`vpce_configuration_name(impl ::std::convert::Into<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::vpce_configuration_name) / [`set_vpce_configuration_name(Option<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::set_vpce_configuration_name): <p>The friendly name you give to your VPC endpoint configuration to manage your configurations more easily.</p>
    ///   - [`vpce_service_name(impl ::std::convert::Into<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::vpce_service_name) / [`set_vpce_service_name(Option<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::set_vpce_service_name): <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
    ///   - [`service_dns_name(impl ::std::convert::Into<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::service_dns_name) / [`set_service_dns_name(Option<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::set_service_dns_name): <p>The DNS (domain) name used to connect to your private service in your VPC. The DNS name must not already be in use on the internet.</p>
    ///   - [`vpce_configuration_description(impl ::std::convert::Into<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::vpce_configuration_description) / [`set_vpce_configuration_description(Option<String>)`](crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::set_vpce_configuration_description): <p>An optional description that provides details about your VPC endpoint configuration.</p>
    /// - On success, responds with [`UpdateVpceConfigurationOutput`](crate::operation::update_vpce_configuration::UpdateVpceConfigurationOutput) with field(s):
    ///   - [`vpce_configuration(Option<VpceConfiguration>)`](crate::operation::update_vpce_configuration::UpdateVpceConfigurationOutput::vpce_configuration): <p>An object that contains information about your VPC endpoint configuration.</p>
    /// - On failure, responds with [`SdkError<UpdateVPCEConfigurationError>`](crate::operation::update_vpce_configuration::UpdateVPCEConfigurationError)
    pub fn update_vpce_configuration(
        &self,
    ) -> crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder
    {
        crate::operation::update_vpce_configuration::builders::UpdateVPCEConfigurationFluentBuilder::new(self.handle.clone())
    }
}
