// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRestApis`](crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`position(impl ::std::convert::Into<String>)`](crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder::position) / [`set_position(Option<String>)`](crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder::set_position): <p>The current pagination position in the paged result set.</p>
    ///   - [`limit(i32)`](crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder::set_limit): <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    /// - On success, responds with [`GetRestApisOutput`](crate::operation::get_rest_apis::GetRestApisOutput) with field(s):
    ///   - [`items(Option<Vec<RestApi>>)`](crate::operation::get_rest_apis::GetRestApisOutput::items): <p>The current page of elements from this collection.</p>
    ///   - [`position(Option<String>)`](crate::operation::get_rest_apis::GetRestApisOutput::position): <p>The current pagination position in the paged result set.</p>
    /// - On failure, responds with [`SdkError<GetRestApisError>`](crate::operation::get_rest_apis::GetRestApisError)
    pub fn get_rest_apis(
        &self,
    ) -> crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder {
        crate::operation::get_rest_apis::builders::GetRestApisFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
