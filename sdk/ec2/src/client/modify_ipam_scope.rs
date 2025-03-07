// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyIpamScope`](crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ipam_scope_id(impl ::std::convert::Into<String>)`](crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder::ipam_scope_id) / [`set_ipam_scope_id(Option<String>)`](crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder::set_ipam_scope_id): <p>The ID of the scope you want to modify.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder::set_description): <p>The description of the scope you want to modify.</p>
    /// - On success, responds with [`ModifyIpamScopeOutput`](crate::operation::modify_ipam_scope::ModifyIpamScopeOutput) with field(s):
    ///   - [`ipam_scope(Option<IpamScope>)`](crate::operation::modify_ipam_scope::ModifyIpamScopeOutput::ipam_scope): <p>The results of the modification.</p>
    /// - On failure, responds with [`SdkError<ModifyIpamScopeError>`](crate::operation::modify_ipam_scope::ModifyIpamScopeError)
    pub fn modify_ipam_scope(
        &self,
    ) -> crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder {
        crate::operation::modify_ipam_scope::builders::ModifyIpamScopeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
