// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAssociatedStacks`](crate::operation::list_associated_stacks::builders::ListAssociatedStacksFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`fleet_name(impl ::std::convert::Into<String>)`](crate::operation::list_associated_stacks::builders::ListAssociatedStacksFluentBuilder::fleet_name) / [`set_fleet_name(Option<String>)`](crate::operation::list_associated_stacks::builders::ListAssociatedStacksFluentBuilder::set_fleet_name): <p>The name of the fleet.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_associated_stacks::builders::ListAssociatedStacksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_associated_stacks::builders::ListAssociatedStacksFluentBuilder::set_next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    /// - On success, responds with [`ListAssociatedStacksOutput`](crate::operation::list_associated_stacks::ListAssociatedStacksOutput) with field(s):
    ///   - [`names(Option<Vec<String>>)`](crate::operation::list_associated_stacks::ListAssociatedStacksOutput::names): <p>The name of the stack.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_associated_stacks::ListAssociatedStacksOutput::next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    /// - On failure, responds with [`SdkError<ListAssociatedStacksError>`](crate::operation::list_associated_stacks::ListAssociatedStacksError)
    pub fn list_associated_stacks(
        &self,
    ) -> crate::operation::list_associated_stacks::builders::ListAssociatedStacksFluentBuilder {
        crate::operation::list_associated_stacks::builders::ListAssociatedStacksFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
