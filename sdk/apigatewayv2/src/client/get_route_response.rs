// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRouteResponse`](crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl ::std::convert::Into<String>)`](crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder::set_api_id): <p>The API identifier.</p>
    ///   - [`route_id(impl ::std::convert::Into<String>)`](crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder::route_id) / [`set_route_id(Option<String>)`](crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder::set_route_id): <p>The route ID.</p>
    ///   - [`route_response_id(impl ::std::convert::Into<String>)`](crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder::route_response_id) / [`set_route_response_id(Option<String>)`](crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder::set_route_response_id): <p>The route response ID.</p>
    /// - On success, responds with [`GetRouteResponseOutput`](crate::operation::get_route_response::GetRouteResponseOutput) with field(s):
    ///   - [`model_selection_expression(Option<String>)`](crate::operation::get_route_response::GetRouteResponseOutput::model_selection_expression): <p>Represents the model selection expression of a route response. Supported only for WebSocket APIs.</p>
    ///   - [`response_models(Option<HashMap<String, String>>)`](crate::operation::get_route_response::GetRouteResponseOutput::response_models): <p>Represents the response models of a route response.</p>
    ///   - [`response_parameters(Option<HashMap<String, ParameterConstraints>>)`](crate::operation::get_route_response::GetRouteResponseOutput::response_parameters): <p>Represents the response parameters of a route response.</p>
    ///   - [`route_response_id(Option<String>)`](crate::operation::get_route_response::GetRouteResponseOutput::route_response_id): <p>Represents the identifier of a route response.</p>
    ///   - [`route_response_key(Option<String>)`](crate::operation::get_route_response::GetRouteResponseOutput::route_response_key): <p>Represents the route response key of a route response.</p>
    /// - On failure, responds with [`SdkError<GetRouteResponseError>`](crate::operation::get_route_response::GetRouteResponseError)
    pub fn get_route_response(
        &self,
    ) -> crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder {
        crate::operation::get_route_response::builders::GetRouteResponseFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
