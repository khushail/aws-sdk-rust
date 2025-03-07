// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAuthorizer`](crate::operation::delete_authorizer::builders::DeleteAuthorizerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl ::std::convert::Into<String>)`](crate::operation::delete_authorizer::builders::DeleteAuthorizerFluentBuilder::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::operation::delete_authorizer::builders::DeleteAuthorizerFluentBuilder::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`authorizer_id(impl ::std::convert::Into<String>)`](crate::operation::delete_authorizer::builders::DeleteAuthorizerFluentBuilder::authorizer_id) / [`set_authorizer_id(Option<String>)`](crate::operation::delete_authorizer::builders::DeleteAuthorizerFluentBuilder::set_authorizer_id): <p>The identifier of the Authorizer resource.</p>
    /// - On success, responds with [`DeleteAuthorizerOutput`](crate::operation::delete_authorizer::DeleteAuthorizerOutput)
    /// - On failure, responds with [`SdkError<DeleteAuthorizerError>`](crate::operation::delete_authorizer::DeleteAuthorizerError)
    pub fn delete_authorizer(
        &self,
    ) -> crate::operation::delete_authorizer::builders::DeleteAuthorizerFluentBuilder {
        crate::operation::delete_authorizer::builders::DeleteAuthorizerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
