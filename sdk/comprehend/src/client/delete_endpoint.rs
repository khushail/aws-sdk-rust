// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteEndpoint`](crate::operation::delete_endpoint::builders::DeleteEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`endpoint_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_endpoint::builders::DeleteEndpointFluentBuilder::endpoint_arn) / [`set_endpoint_arn(Option<String>)`](crate::operation::delete_endpoint::builders::DeleteEndpointFluentBuilder::set_endpoint_arn): <p>The Amazon Resource Number (ARN) of the endpoint being deleted.</p>
    /// - On success, responds with [`DeleteEndpointOutput`](crate::operation::delete_endpoint::DeleteEndpointOutput)
    /// - On failure, responds with [`SdkError<DeleteEndpointError>`](crate::operation::delete_endpoint::DeleteEndpointError)
    pub fn delete_endpoint(
        &self,
    ) -> crate::operation::delete_endpoint::builders::DeleteEndpointFluentBuilder {
        crate::operation::delete_endpoint::builders::DeleteEndpointFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
