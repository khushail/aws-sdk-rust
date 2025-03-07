// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyAccount`](crate::operation::modify_account::builders::ModifyAccountFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dedicated_tenancy_support(DedicatedTenancySupportEnum)`](crate::operation::modify_account::builders::ModifyAccountFluentBuilder::dedicated_tenancy_support) / [`set_dedicated_tenancy_support(Option<DedicatedTenancySupportEnum>)`](crate::operation::modify_account::builders::ModifyAccountFluentBuilder::set_dedicated_tenancy_support): <p>The status of BYOL.</p>
    ///   - [`dedicated_tenancy_management_cidr_range(impl ::std::convert::Into<String>)`](crate::operation::modify_account::builders::ModifyAccountFluentBuilder::dedicated_tenancy_management_cidr_range) / [`set_dedicated_tenancy_management_cidr_range(Option<String>)`](crate::operation::modify_account::builders::ModifyAccountFluentBuilder::set_dedicated_tenancy_management_cidr_range): <p>The IP address range, specified as an IPv4 CIDR block, for the management network interface. Specify an IP address range that is compatible with your network and in CIDR notation (that is, specify the range as an IPv4 CIDR block). The CIDR block size must be /16 (for example, 203.0.113.25/16). It must also be specified as available by the <code>ListAvailableManagementCidrRanges</code> operation.</p>
    /// - On success, responds with [`ModifyAccountOutput`](crate::operation::modify_account::ModifyAccountOutput)
    /// - On failure, responds with [`SdkError<ModifyAccountError>`](crate::operation::modify_account::ModifyAccountError)
    pub fn modify_account(
        &self,
    ) -> crate::operation::modify_account::builders::ModifyAccountFluentBuilder {
        crate::operation::modify_account::builders::ModifyAccountFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
