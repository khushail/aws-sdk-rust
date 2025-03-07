// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateUserHierarchyStructure`](crate::operation::update_user_hierarchy_structure::builders::UpdateUserHierarchyStructureFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hierarchy_structure(HierarchyStructureUpdate)`](crate::operation::update_user_hierarchy_structure::builders::UpdateUserHierarchyStructureFluentBuilder::hierarchy_structure) / [`set_hierarchy_structure(Option<HierarchyStructureUpdate>)`](crate::operation::update_user_hierarchy_structure::builders::UpdateUserHierarchyStructureFluentBuilder::set_hierarchy_structure): <p>The hierarchy levels to update.</p>
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::update_user_hierarchy_structure::builders::UpdateUserHierarchyStructureFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::update_user_hierarchy_structure::builders::UpdateUserHierarchyStructureFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    /// - On success, responds with [`UpdateUserHierarchyStructureOutput`](crate::operation::update_user_hierarchy_structure::UpdateUserHierarchyStructureOutput)
    /// - On failure, responds with [`SdkError<UpdateUserHierarchyStructureError>`](crate::operation::update_user_hierarchy_structure::UpdateUserHierarchyStructureError)
    pub fn update_user_hierarchy_structure(&self) -> crate::operation::update_user_hierarchy_structure::builders::UpdateUserHierarchyStructureFluentBuilder{
        crate::operation::update_user_hierarchy_structure::builders::UpdateUserHierarchyStructureFluentBuilder::new(self.handle.clone())
    }
}
