// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateVPCConnection`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::set_aws_account_id): <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want to update.</p>
    ///   - [`vpc_connection_id(impl ::std::convert::Into<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::vpc_connection_id) / [`set_vpc_connection_id(Option<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::set_vpc_connection_id): <p>The ID of the VPC connection that you're updating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::set_name): <p>The display name for the VPC connection.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::set_subnet_ids): <p>A list of subnet IDs for the VPC connection.</p>
    ///   - [`security_group_ids(Vec<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::security_group_ids) / [`set_security_group_ids(Option<Vec<String>>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::set_security_group_ids): <p>A list of security group IDs for the VPC connection.</p>
    ///   - [`dns_resolvers(Vec<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::dns_resolvers) / [`set_dns_resolvers(Option<Vec<String>>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::set_dns_resolvers): <p>A list of IP addresses of DNS resolver endpoints for the VPC connection.</p>
    ///   - [`role_arn(impl ::std::convert::Into<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::set_role_arn): <p>An IAM role associated with the VPC connection.</p>
    /// - On success, responds with [`UpdateVpcConnectionOutput`](crate::operation::update_vpc_connection::UpdateVpcConnectionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_vpc_connection::UpdateVpcConnectionOutput::arn): <p>The Amazon Resource Name (ARN) of the VPC connection.</p>
    ///   - [`vpc_connection_id(Option<String>)`](crate::operation::update_vpc_connection::UpdateVpcConnectionOutput::vpc_connection_id): <p>The ID of the VPC connection that you are updating. This ID is a unique identifier for each Amazon Web Services Region in anAmazon Web Services account.</p>
    ///   - [`update_status(Option<VpcConnectionResourceStatus>)`](crate::operation::update_vpc_connection::UpdateVpcConnectionOutput::update_status): <p>The update status of the VPC connection's last update.</p>
    ///   - [`availability_status(Option<VpcConnectionAvailabilityStatus>)`](crate::operation::update_vpc_connection::UpdateVpcConnectionOutput::availability_status): <p>The availability status of the VPC connection.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::update_vpc_connection::UpdateVpcConnectionOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::update_vpc_connection::UpdateVpcConnectionOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<UpdateVPCConnectionError>`](crate::operation::update_vpc_connection::UpdateVPCConnectionError)
    pub fn update_vpc_connection(
        &self,
    ) -> crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder {
        crate::operation::update_vpc_connection::builders::UpdateVPCConnectionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
