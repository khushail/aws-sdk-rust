// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSchema`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token_id(impl ::std::convert::Into<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::client_token_id) / [`set_client_token_id(Option<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::set_client_token_id): <p>The ID of the client token.</p>
    ///   - [`content(impl ::std::convert::Into<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::content) / [`set_content(Option<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::set_content): <p>The source of the schema definition.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::set_description): <p>The description of the schema.</p>
    ///   - [`registry_name(impl ::std::convert::Into<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::registry_name) / [`set_registry_name(Option<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::set_registry_name): <p>The name of the registry.</p>
    ///   - [`schema_name(impl ::std::convert::Into<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::schema_name) / [`set_schema_name(Option<String>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::set_schema_name): <p>The name of the schema.</p>
    ///   - [`r#type(Type)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::type) / [`set_type(Option<Type>)`](crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::set_type): <p>The schema type for the events schema.</p>
    /// - On success, responds with [`UpdateSchemaOutput`](crate::operation::update_schema::UpdateSchemaOutput) with field(s):
    ///   - [`description(Option<String>)`](crate::operation::update_schema::UpdateSchemaOutput::description): <p>The description of the schema.</p>
    ///   - [`last_modified(Option<DateTime>)`](crate::operation::update_schema::UpdateSchemaOutput::last_modified): <p>The date and time that schema was modified.</p>
    ///   - [`schema_arn(Option<String>)`](crate::operation::update_schema::UpdateSchemaOutput::schema_arn): <p>The ARN of the schema.</p>
    ///   - [`schema_name(Option<String>)`](crate::operation::update_schema::UpdateSchemaOutput::schema_name): <p>The name of the schema.</p>
    ///   - [`schema_version(Option<String>)`](crate::operation::update_schema::UpdateSchemaOutput::schema_version): <p>The version number of the schema</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::update_schema::UpdateSchemaOutput::tags): <p>Key-value pairs associated with a resource.</p>
    ///   - [`r#type(Option<String>)`](crate::operation::update_schema::UpdateSchemaOutput::type): <p>The type of the schema.</p>
    ///   - [`version_created_date(Option<DateTime>)`](crate::operation::update_schema::UpdateSchemaOutput::version_created_date): <p>The date the schema version was created.</p>
    /// - On failure, responds with [`SdkError<UpdateSchemaError>`](crate::operation::update_schema::UpdateSchemaError)
    pub fn update_schema(
        &self,
    ) -> crate::operation::update_schema::builders::UpdateSchemaFluentBuilder {
        crate::operation::update_schema::builders::UpdateSchemaFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
