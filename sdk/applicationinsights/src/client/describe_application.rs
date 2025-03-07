// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeApplication`](crate::operation::describe_application::builders::DescribeApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_group_name(impl ::std::convert::Into<String>)`](crate::operation::describe_application::builders::DescribeApplicationFluentBuilder::resource_group_name) / [`set_resource_group_name(Option<String>)`](crate::operation::describe_application::builders::DescribeApplicationFluentBuilder::set_resource_group_name): <p>The name of the resource group.</p>
    /// - On success, responds with [`DescribeApplicationOutput`](crate::operation::describe_application::DescribeApplicationOutput) with field(s):
    ///   - [`application_info(Option<ApplicationInfo>)`](crate::operation::describe_application::DescribeApplicationOutput::application_info): <p>Information about the application.</p>
    /// - On failure, responds with [`SdkError<DescribeApplicationError>`](crate::operation::describe_application::DescribeApplicationError)
    pub fn describe_application(
        &self,
    ) -> crate::operation::describe_application::builders::DescribeApplicationFluentBuilder {
        crate::operation::describe_application::builders::DescribeApplicationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
