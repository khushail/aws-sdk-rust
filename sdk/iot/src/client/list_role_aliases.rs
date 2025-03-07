// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListRoleAliases`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`page_size(i32)`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::page_size) / [`set_page_size(Option<i32>)`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::set_page_size): <p>The maximum number of results to return at one time.</p>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::set_marker): <p>A marker used to get the next set of results.</p>
    ///   - [`ascending_order(bool)`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::ascending_order) / [`set_ascending_order(Option<bool>)`](crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::set_ascending_order): <p>Return the list of role aliases in ascending alphabetical order.</p>
    /// - On success, responds with [`ListRoleAliasesOutput`](crate::operation::list_role_aliases::ListRoleAliasesOutput) with field(s):
    ///   - [`role_aliases(Option<Vec<String>>)`](crate::operation::list_role_aliases::ListRoleAliasesOutput::role_aliases): <p>The role aliases.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::list_role_aliases::ListRoleAliasesOutput::next_marker): <p>A marker used to get the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListRoleAliasesError>`](crate::operation::list_role_aliases::ListRoleAliasesError)
    pub fn list_role_aliases(
        &self,
    ) -> crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder {
        crate::operation::list_role_aliases::builders::ListRoleAliasesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
