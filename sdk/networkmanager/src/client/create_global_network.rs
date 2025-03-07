// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateGlobalNetwork`](crate::operation::create_global_network::builders::CreateGlobalNetworkFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_global_network::builders::CreateGlobalNetworkFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_global_network::builders::CreateGlobalNetworkFluentBuilder::set_description): <p>A description of the global network.</p>  <p>Constraints: Maximum length of 256 characters.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_global_network::builders::CreateGlobalNetworkFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_global_network::builders::CreateGlobalNetworkFluentBuilder::set_tags): <p>The tags to apply to the resource during creation.</p>
    /// - On success, responds with [`CreateGlobalNetworkOutput`](crate::operation::create_global_network::CreateGlobalNetworkOutput) with field(s):
    ///   - [`global_network(Option<GlobalNetwork>)`](crate::operation::create_global_network::CreateGlobalNetworkOutput::global_network): <p>Information about the global network object.</p>
    /// - On failure, responds with [`SdkError<CreateGlobalNetworkError>`](crate::operation::create_global_network::CreateGlobalNetworkError)
    pub fn create_global_network(
        &self,
    ) -> crate::operation::create_global_network::builders::CreateGlobalNetworkFluentBuilder {
        crate::operation::create_global_network::builders::CreateGlobalNetworkFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
