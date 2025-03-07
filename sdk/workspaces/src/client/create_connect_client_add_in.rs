// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateConnectClientAddIn`](crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder::set_resource_id): <p>The directory identifier for which to configure the client add-in.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder::set_name): <p>The name of the client add-in.</p>
    ///   - [`url(impl ::std::convert::Into<String>)`](crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder::url) / [`set_url(Option<String>)`](crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder::set_url): <p>The endpoint URL of the Amazon Connect client add-in.</p>
    /// - On success, responds with [`CreateConnectClientAddInOutput`](crate::operation::create_connect_client_add_in::CreateConnectClientAddInOutput) with field(s):
    ///   - [`add_in_id(Option<String>)`](crate::operation::create_connect_client_add_in::CreateConnectClientAddInOutput::add_in_id): <p>The client add-in identifier.</p>
    /// - On failure, responds with [`SdkError<CreateConnectClientAddInError>`](crate::operation::create_connect_client_add_in::CreateConnectClientAddInError)
    pub fn create_connect_client_add_in(&self) -> crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder{
        crate::operation::create_connect_client_add_in::builders::CreateConnectClientAddInFluentBuilder::new(self.handle.clone())
    }
}
