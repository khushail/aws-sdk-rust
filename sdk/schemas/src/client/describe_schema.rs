// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSchema`](crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`registry_name(impl ::std::convert::Into<String>)`](crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder::registry_name) / [`set_registry_name(Option<String>)`](crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder::set_registry_name): <p>The name of the registry.</p>
    ///   - [`schema_name(impl ::std::convert::Into<String>)`](crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder::schema_name) / [`set_schema_name(Option<String>)`](crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder::set_schema_name): <p>The name of the schema.</p>
    ///   - [`schema_version(impl ::std::convert::Into<String>)`](crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder::schema_version) / [`set_schema_version(Option<String>)`](crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder::set_schema_version): <p>Specifying this limits the results to only this schema version.</p>
    /// - On success, responds with [`DescribeSchemaOutput`](crate::operation::describe_schema::DescribeSchemaOutput) with field(s):
    ///   - [`content(Option<String>)`](crate::operation::describe_schema::DescribeSchemaOutput::content): <p>The source of the schema definition.</p>
    ///   - [`description(Option<String>)`](crate::operation::describe_schema::DescribeSchemaOutput::description): <p>The description of the schema.</p>
    ///   - [`last_modified(Option<DateTime>)`](crate::operation::describe_schema::DescribeSchemaOutput::last_modified): <p>The date and time that schema was modified.</p>
    ///   - [`schema_arn(Option<String>)`](crate::operation::describe_schema::DescribeSchemaOutput::schema_arn): <p>The ARN of the schema.</p>
    ///   - [`schema_name(Option<String>)`](crate::operation::describe_schema::DescribeSchemaOutput::schema_name): <p>The name of the schema.</p>
    ///   - [`schema_version(Option<String>)`](crate::operation::describe_schema::DescribeSchemaOutput::schema_version): <p>The version number of the schema</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::describe_schema::DescribeSchemaOutput::tags): <p>Tags associated with the resource.</p>
    ///   - [`r#type(Option<String>)`](crate::operation::describe_schema::DescribeSchemaOutput::type): <p>The type of the schema.</p>
    ///   - [`version_created_date(Option<DateTime>)`](crate::operation::describe_schema::DescribeSchemaOutput::version_created_date): <p>The date the schema version was created.</p>
    /// - On failure, responds with [`SdkError<DescribeSchemaError>`](crate::operation::describe_schema::DescribeSchemaError)
    pub fn describe_schema(
        &self,
    ) -> crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder {
        crate::operation::describe_schema::builders::DescribeSchemaFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
