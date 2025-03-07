// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeGatewayRoute`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_route_name(impl ::std::convert::Into<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::gateway_route_name) / [`set_gateway_route_name(Option<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::set_gateway_route_name): <p>The name of the gateway route to describe.</p>
    ///   - [`mesh_name(impl ::std::convert::Into<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::mesh_name) / [`set_mesh_name(Option<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::set_mesh_name): <p>The name of the service mesh that the gateway route resides in.</p>
    ///   - [`virtual_gateway_name(impl ::std::convert::Into<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::virtual_gateway_name) / [`set_virtual_gateway_name(Option<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::set_virtual_gateway_name): <p>The name of the virtual gateway that the gateway route is associated with.</p>
    ///   - [`mesh_owner(impl ::std::convert::Into<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::mesh_owner) / [`set_mesh_owner(Option<String>)`](crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::set_mesh_owner): <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    /// - On success, responds with [`DescribeGatewayRouteOutput`](crate::operation::describe_gateway_route::DescribeGatewayRouteOutput) with field(s):
    ///   - [`gateway_route(Option<GatewayRouteData>)`](crate::operation::describe_gateway_route::DescribeGatewayRouteOutput::gateway_route): <p>The full description of your gateway route.</p>
    /// - On failure, responds with [`SdkError<DescribeGatewayRouteError>`](crate::operation::describe_gateway_route::DescribeGatewayRouteError)
    pub fn describe_gateway_route(
        &self,
    ) -> crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder {
        crate::operation::describe_gateway_route::builders::DescribeGatewayRouteFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
