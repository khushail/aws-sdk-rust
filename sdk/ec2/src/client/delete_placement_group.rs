// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeletePlacementGroup`](crate::operation::delete_placement_group::builders::DeletePlacementGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_placement_group::builders::DeletePlacementGroupFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_placement_group::builders::DeletePlacementGroupFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`group_name(impl ::std::convert::Into<String>)`](crate::operation::delete_placement_group::builders::DeletePlacementGroupFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::delete_placement_group::builders::DeletePlacementGroupFluentBuilder::set_group_name): <p>The name of the placement group.</p>
    /// - On success, responds with [`DeletePlacementGroupOutput`](crate::operation::delete_placement_group::DeletePlacementGroupOutput)
    /// - On failure, responds with [`SdkError<DeletePlacementGroupError>`](crate::operation::delete_placement_group::DeletePlacementGroupError)
    pub fn delete_placement_group(
        &self,
    ) -> crate::operation::delete_placement_group::builders::DeletePlacementGroupFluentBuilder {
        crate::operation::delete_placement_group::builders::DeletePlacementGroupFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
