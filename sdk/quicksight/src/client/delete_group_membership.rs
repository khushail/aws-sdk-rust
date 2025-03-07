// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteGroupMembership`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`member_name(impl ::std::convert::Into<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::member_name) / [`set_member_name(Option<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::set_member_name): <p>The name of the user that you want to delete from the group membership.</p>
    ///   - [`group_name(impl ::std::convert::Into<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::set_group_name): <p>The name of the group that you want to delete the user from.</p>
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::set_aws_account_id): <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    ///   - [`namespace(impl ::std::convert::Into<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::set_namespace): <p>The namespace of the group that you want to remove a user from.</p>
    /// - On success, responds with [`DeleteGroupMembershipOutput`](crate::operation::delete_group_membership::DeleteGroupMembershipOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::delete_group_membership::DeleteGroupMembershipOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::delete_group_membership::DeleteGroupMembershipOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<DeleteGroupMembershipError>`](crate::operation::delete_group_membership::DeleteGroupMembershipError)
    pub fn delete_group_membership(
        &self,
    ) -> crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder
    {
        crate::operation::delete_group_membership::builders::DeleteGroupMembershipFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
