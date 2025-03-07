// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDevelopmentSchemaArns`](crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder::set_next_token): <p>The pagination token.</p>
    ///   - [`max_results(i32)`](crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder::set_max_results): <p>The maximum number of results to retrieve.</p>
    /// - On success, responds with [`ListDevelopmentSchemaArnsOutput`](crate::operation::list_development_schema_arns::ListDevelopmentSchemaArnsOutput) with field(s):
    ///   - [`schema_arns(Option<Vec<String>>)`](crate::operation::list_development_schema_arns::ListDevelopmentSchemaArnsOutput::schema_arns): <p>The ARNs of retrieved development schemas.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_development_schema_arns::ListDevelopmentSchemaArnsOutput::next_token): <p>The pagination token.</p>
    /// - On failure, responds with [`SdkError<ListDevelopmentSchemaArnsError>`](crate::operation::list_development_schema_arns::ListDevelopmentSchemaArnsError)
    pub fn list_development_schema_arns(&self) -> crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder{
        crate::operation::list_development_schema_arns::builders::ListDevelopmentSchemaArnsFluentBuilder::new(self.handle.clone())
    }
}
