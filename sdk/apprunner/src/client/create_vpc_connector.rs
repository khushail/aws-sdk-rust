// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVpcConnector`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpc_connector_name(impl ::std::convert::Into<String>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::vpc_connector_name) / [`set_vpc_connector_name(Option<String>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::set_vpc_connector_name): <p>A name for the VPC connector.</p>
    ///   - [`subnets(Vec<String>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::subnets) / [`set_subnets(Option<Vec<String>>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::set_subnets): <p>A list of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.</p> <note>   <p> App Runner currently only provides support for IPv4. </p>  </note>
    ///   - [`security_groups(Vec<String>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::security_groups) / [`set_security_groups(Option<Vec<String>>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::set_security_groups): <p>A list of IDs of security groups that App Runner should use for access to Amazon Web Services resources under the specified subnets. If not specified, App Runner uses the default security group of the Amazon VPC. The default security group allows all outbound traffic.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::set_tags): <p>A list of metadata items that you can associate with your VPC connector resource. A tag is a key-value pair.</p>
    /// - On success, responds with [`CreateVpcConnectorOutput`](crate::operation::create_vpc_connector::CreateVpcConnectorOutput) with field(s):
    ///   - [`vpc_connector(Option<VpcConnector>)`](crate::operation::create_vpc_connector::CreateVpcConnectorOutput::vpc_connector): <p>A description of the App Runner VPC connector that's created by this request.</p>
    /// - On failure, responds with [`SdkError<CreateVpcConnectorError>`](crate::operation::create_vpc_connector::CreateVpcConnectorError)
    pub fn create_vpc_connector(
        &self,
    ) -> crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder {
        crate::operation::create_vpc_connector::builders::CreateVpcConnectorFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
