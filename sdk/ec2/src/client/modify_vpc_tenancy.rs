// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVpcTenancy`](crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpc_id(impl ::std::convert::Into<String>)`](crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder::vpc_id) / [`set_vpc_id(Option<String>)`](crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder::set_vpc_id): <p>The ID of the VPC.</p>
    ///   - [`instance_tenancy(VpcTenancy)`](crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder::instance_tenancy) / [`set_instance_tenancy(Option<VpcTenancy>)`](crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder::set_instance_tenancy): <p>The instance tenancy attribute for the VPC. </p>
    ///   - [`dry_run(bool)`](crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifyVpcTenancyOutput`](crate::operation::modify_vpc_tenancy::ModifyVpcTenancyOutput) with field(s):
    ///   - [`return_value(Option<bool>)`](crate::operation::modify_vpc_tenancy::ModifyVpcTenancyOutput::return_value): <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    /// - On failure, responds with [`SdkError<ModifyVpcTenancyError>`](crate::operation::modify_vpc_tenancy::ModifyVpcTenancyError)
    pub fn modify_vpc_tenancy(
        &self,
    ) -> crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder {
        crate::operation::modify_vpc_tenancy::builders::ModifyVpcTenancyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
