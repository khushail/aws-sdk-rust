// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSnapshotAttribute`](crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`attribute(SnapshotAttributeName)`](crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder::attribute) / [`set_attribute(Option<SnapshotAttributeName>)`](crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder::set_attribute): <p>The snapshot attribute you would like to view.</p>
    ///   - [`snapshot_id(impl ::std::convert::Into<String>)`](crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder::set_snapshot_id): <p>The ID of the EBS snapshot.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DescribeSnapshotAttributeOutput`](crate::operation::describe_snapshot_attribute::DescribeSnapshotAttributeOutput) with field(s):
    ///   - [`create_volume_permissions(Option<Vec<CreateVolumePermission>>)`](crate::operation::describe_snapshot_attribute::DescribeSnapshotAttributeOutput::create_volume_permissions): <p>The users and groups that have the permissions for creating volumes from the snapshot.</p>
    ///   - [`product_codes(Option<Vec<ProductCode>>)`](crate::operation::describe_snapshot_attribute::DescribeSnapshotAttributeOutput::product_codes): <p>The product codes.</p>
    ///   - [`snapshot_id(Option<String>)`](crate::operation::describe_snapshot_attribute::DescribeSnapshotAttributeOutput::snapshot_id): <p>The ID of the EBS snapshot.</p>
    /// - On failure, responds with [`SdkError<DescribeSnapshotAttributeError>`](crate::operation::describe_snapshot_attribute::DescribeSnapshotAttributeError)
    pub fn describe_snapshot_attribute(&self) -> crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder{
        crate::operation::describe_snapshot_attribute::builders::DescribeSnapshotAttributeFluentBuilder::new(self.handle.clone())
    }
}
