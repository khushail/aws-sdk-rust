// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCodeBinding`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`language(impl ::std::convert::Into<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::language) / [`set_language(Option<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::set_language): <p>The language of the code binding.</p>
    ///   - [`registry_name(impl ::std::convert::Into<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::registry_name) / [`set_registry_name(Option<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::set_registry_name): <p>The name of the registry.</p>
    ///   - [`schema_name(impl ::std::convert::Into<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::schema_name) / [`set_schema_name(Option<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::set_schema_name): <p>The name of the schema.</p>
    ///   - [`schema_version(impl ::std::convert::Into<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::schema_version) / [`set_schema_version(Option<String>)`](crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::set_schema_version): <p>Specifying this limits the results to only this schema version.</p>
    /// - On success, responds with [`DescribeCodeBindingOutput`](crate::operation::describe_code_binding::DescribeCodeBindingOutput) with field(s):
    ///   - [`creation_date(Option<DateTime>)`](crate::operation::describe_code_binding::DescribeCodeBindingOutput::creation_date): <p>The time and date that the code binding was created.</p>
    ///   - [`last_modified(Option<DateTime>)`](crate::operation::describe_code_binding::DescribeCodeBindingOutput::last_modified): <p>The date and time that code bindings were modified.</p>
    ///   - [`schema_version(Option<String>)`](crate::operation::describe_code_binding::DescribeCodeBindingOutput::schema_version): <p>The version number of the schema.</p>
    ///   - [`status(Option<CodeGenerationStatus>)`](crate::operation::describe_code_binding::DescribeCodeBindingOutput::status): <p>The current status of code binding generation.</p>
    /// - On failure, responds with [`SdkError<DescribeCodeBindingError>`](crate::operation::describe_code_binding::DescribeCodeBindingError)
    pub fn describe_code_binding(
        &self,
    ) -> crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder {
        crate::operation::describe_code_binding::builders::DescribeCodeBindingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
