// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateProject`](crate::operation::create_project::builders::CreateProjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`project_name(impl ::std::convert::Into<String>)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::project_name) / [`set_project_name(Option<String>)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::set_project_name): <p>The name of the project to create.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::set_description): <p>An optional description for the project.</p>
    ///   - [`placement_template(PlacementTemplate)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::placement_template) / [`set_placement_template(Option<PlacementTemplate>)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::set_placement_template): <p>The schema defining the placement to be created. A placement template defines placement default attributes and device templates. You cannot add or remove device templates after the project has been created. However, you can update <code>callbackOverrides</code> for the device templates using the <code>UpdateProject</code> API.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_project::builders::CreateProjectFluentBuilder::set_tags): <p>Optional tags (metadata key/value pairs) to be associated with the project. For example, <code>{ {"key1": "value1", "key2": "value2"} }</code>. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
    /// - On success, responds with [`CreateProjectOutput`](crate::operation::create_project::CreateProjectOutput)
    /// - On failure, responds with [`SdkError<CreateProjectError>`](crate::operation::create_project::CreateProjectError)
    pub fn create_project(
        &self,
    ) -> crate::operation::create_project::builders::CreateProjectFluentBuilder {
        crate::operation::create_project::builders::CreateProjectFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
