// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListThingTypes`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::set_next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::set_max_results): <p>The maximum number of results to return in this operation.</p>
    ///   - [`thing_type_name(impl ::std::convert::Into<String>)`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::thing_type_name) / [`set_thing_type_name(Option<String>)`](crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::set_thing_type_name): <p>The name of the thing type.</p>
    /// - On success, responds with [`ListThingTypesOutput`](crate::operation::list_thing_types::ListThingTypesOutput) with field(s):
    ///   - [`thing_types(Option<Vec<ThingTypeDefinition>>)`](crate::operation::list_thing_types::ListThingTypesOutput::thing_types): <p>The thing types.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_thing_types::ListThingTypesOutput::next_token): <p>The token for the next set of results. Will not be returned if operation has returned all results.</p>
    /// - On failure, responds with [`SdkError<ListThingTypesError>`](crate::operation::list_thing_types::ListThingTypesError)
    pub fn list_thing_types(
        &self,
    ) -> crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder {
        crate::operation::list_thing_types::builders::ListThingTypesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
