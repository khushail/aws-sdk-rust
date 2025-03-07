// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAppInstanceUserEndpoint`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_instance_user_arn(impl ::std::convert::Into<String>)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::app_instance_user_arn) / [`set_app_instance_user_arn(Option<String>)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::set_app_instance_user_arn): <p>The ARN of the <code>AppInstanceUser</code>.</p>
    ///   - [`endpoint_id(impl ::std::convert::Into<String>)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::endpoint_id) / [`set_endpoint_id(Option<String>)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::set_endpoint_id): <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::set_name): <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    ///   - [`allow_messages(AllowMessages)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::allow_messages) / [`set_allow_messages(Option<AllowMessages>)`](crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::set_allow_messages): <p>Boolean that controls whether the <code>AppInstanceUserEndpoint</code> is opted in to receive messages. <code>ALL</code> indicates the endpoint will receive all messages. <code>NONE</code> indicates the endpoint will receive no messages.</p>
    /// - On success, responds with [`UpdateAppInstanceUserEndpointOutput`](crate::operation::update_app_instance_user_endpoint::UpdateAppInstanceUserEndpointOutput) with field(s):
    ///   - [`app_instance_user_arn(Option<String>)`](crate::operation::update_app_instance_user_endpoint::UpdateAppInstanceUserEndpointOutput::app_instance_user_arn): <p>The ARN of the <code>AppInstanceUser</code>.</p>
    ///   - [`endpoint_id(Option<String>)`](crate::operation::update_app_instance_user_endpoint::UpdateAppInstanceUserEndpointOutput::endpoint_id): <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
    /// - On failure, responds with [`SdkError<UpdateAppInstanceUserEndpointError>`](crate::operation::update_app_instance_user_endpoint::UpdateAppInstanceUserEndpointError)
    pub fn update_app_instance_user_endpoint(&self) -> crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder{
        crate::operation::update_app_instance_user_endpoint::builders::UpdateAppInstanceUserEndpointFluentBuilder::new(self.handle.clone())
    }
}
