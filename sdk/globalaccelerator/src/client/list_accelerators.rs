// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAccelerators`](crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder::set_max_results): <p>The number of Global Accelerator objects that you want to return with this call. The default value is 10.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder::set_next_token): <p>The token for the next set of results. You receive this token from a previous call.</p>
    /// - On success, responds with [`ListAcceleratorsOutput`](crate::operation::list_accelerators::ListAcceleratorsOutput) with field(s):
    ///   - [`accelerators(Option<Vec<Accelerator>>)`](crate::operation::list_accelerators::ListAcceleratorsOutput::accelerators): <p>The list of accelerators for a customer account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_accelerators::ListAcceleratorsOutput::next_token): <p>The token for the next set of results. You receive this token from a previous call.</p>
    /// - On failure, responds with [`SdkError<ListAcceleratorsError>`](crate::operation::list_accelerators::ListAcceleratorsError)
    pub fn list_accelerators(
        &self,
    ) -> crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder {
        crate::operation::list_accelerators::builders::ListAcceleratorsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
