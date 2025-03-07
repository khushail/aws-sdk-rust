// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAddon`](crate::operation::describe_addon::builders::DescribeAddonFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl ::std::convert::Into<String>)`](crate::operation::describe_addon::builders::DescribeAddonFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::describe_addon::builders::DescribeAddonFluentBuilder::set_cluster_name): <p>The name of the cluster.</p>
    ///   - [`addon_name(impl ::std::convert::Into<String>)`](crate::operation::describe_addon::builders::DescribeAddonFluentBuilder::addon_name) / [`set_addon_name(Option<String>)`](crate::operation::describe_addon::builders::DescribeAddonFluentBuilder::set_addon_name): <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
    /// - On success, responds with [`DescribeAddonOutput`](crate::operation::describe_addon::DescribeAddonOutput) with field(s):
    ///   - [`addon(Option<Addon>)`](crate::operation::describe_addon::DescribeAddonOutput::addon): <p>An Amazon EKS add-on. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-add-ons.html">Amazon EKS add-ons</a> in the <i>Amazon EKS User Guide</i>.</p>
    /// - On failure, responds with [`SdkError<DescribeAddonError>`](crate::operation::describe_addon::DescribeAddonError)
    pub fn describe_addon(
        &self,
    ) -> crate::operation::describe_addon::builders::DescribeAddonFluentBuilder {
        crate::operation::describe_addon::builders::DescribeAddonFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
