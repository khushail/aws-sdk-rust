// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartSchemaCreation`](crate::operation::start_schema_creation::builders::StartSchemaCreationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl ::std::convert::Into<String>)`](crate::operation::start_schema_creation::builders::StartSchemaCreationFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::start_schema_creation::builders::StartSchemaCreationFluentBuilder::set_api_id): <p>The API ID.</p>
    ///   - [`definition(Blob)`](crate::operation::start_schema_creation::builders::StartSchemaCreationFluentBuilder::definition) / [`set_definition(Option<Blob>)`](crate::operation::start_schema_creation::builders::StartSchemaCreationFluentBuilder::set_definition): <p>The schema definition, in GraphQL schema language format.</p>
    /// - On success, responds with [`StartSchemaCreationOutput`](crate::operation::start_schema_creation::StartSchemaCreationOutput) with field(s):
    ///   - [`status(Option<SchemaStatus>)`](crate::operation::start_schema_creation::StartSchemaCreationOutput::status): <p>The current state of the schema (PROCESSING, FAILED, SUCCESS, or NOT_APPLICABLE). When the schema is in the ACTIVE state, you can add data.</p>
    /// - On failure, responds with [`SdkError<StartSchemaCreationError>`](crate::operation::start_schema_creation::StartSchemaCreationError)
    pub fn start_schema_creation(
        &self,
    ) -> crate::operation::start_schema_creation::builders::StartSchemaCreationFluentBuilder {
        crate::operation::start_schema_creation::builders::StartSchemaCreationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
