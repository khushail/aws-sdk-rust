// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBranch`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`repository_name(impl ::std::convert::Into<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::set_repository_name): <p>The name of the repository that contains the branch to be deleted.</p>
    ///   - [`branch_name(impl ::std::convert::Into<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::branch_name) / [`set_branch_name(Option<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::set_branch_name): <p>The name of the branch to delete.</p>
    /// - On success, responds with [`DeleteBranchOutput`](crate::operation::delete_branch::DeleteBranchOutput) with field(s):
    ///   - [`deleted_branch(Option<BranchInfo>)`](crate::operation::delete_branch::DeleteBranchOutput::deleted_branch): <p>Information about the branch deleted by the operation, including the branch name and the commit ID that was the tip of the branch.</p>
    /// - On failure, responds with [`SdkError<DeleteBranchError>`](crate::operation::delete_branch::DeleteBranchError)
    pub fn delete_branch(
        &self,
    ) -> crate::operation::delete_branch::builders::DeleteBranchFluentBuilder {
        crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
