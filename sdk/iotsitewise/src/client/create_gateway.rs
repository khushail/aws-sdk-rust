// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateGateway`](crate::operation::create_gateway::builders::CreateGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_name(impl ::std::convert::Into<String>)`](crate::operation::create_gateway::builders::CreateGatewayFluentBuilder::gateway_name) / [`set_gateway_name(Option<String>)`](crate::operation::create_gateway::builders::CreateGatewayFluentBuilder::set_gateway_name): <p>A unique, friendly name for the gateway.</p>
    ///   - [`gateway_platform(GatewayPlatform)`](crate::operation::create_gateway::builders::CreateGatewayFluentBuilder::gateway_platform) / [`set_gateway_platform(Option<GatewayPlatform>)`](crate::operation::create_gateway::builders::CreateGatewayFluentBuilder::set_gateway_platform): <p>The gateway's platform. You can only specify one platform in a gateway.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_gateway::builders::CreateGatewayFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_gateway::builders::CreateGatewayFluentBuilder::set_tags): <p>A list of key-value pairs that contain metadata for the gateway. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise resources</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// - On success, responds with [`CreateGatewayOutput`](crate::operation::create_gateway::CreateGatewayOutput) with field(s):
    ///   - [`gateway_id(Option<String>)`](crate::operation::create_gateway::CreateGatewayOutput::gateway_id): <p>The ID of the gateway device. You can use this ID when you call other IoT SiteWise APIs.</p>
    ///   - [`gateway_arn(Option<String>)`](crate::operation::create_gateway::CreateGatewayOutput::gateway_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the gateway, which has the following format.</p>  <p> <code>arn:${Partition}:iotsitewise:${Region}:${Account}:gateway/${GatewayId}</code> </p>
    /// - On failure, responds with [`SdkError<CreateGatewayError>`](crate::operation::create_gateway::CreateGatewayError)
    pub fn create_gateway(
        &self,
    ) -> crate::operation::create_gateway::builders::CreateGatewayFluentBuilder {
        crate::operation::create_gateway::builders::CreateGatewayFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
