// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateEnvironment`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`environment_id(impl ::std::convert::Into<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::environment_id) / [`set_environment_id(Option<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_environment_id): <p>The identifier of the FinSpace environment.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_name): <p>The name of the environment.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_description): <p>The description of the environment.</p>
    ///   - [`federation_mode(FederationMode)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::federation_mode) / [`set_federation_mode(Option<FederationMode>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_federation_mode): <p>Authentication mode for the environment.</p>  <ul>   <li> <p> <code>FEDERATED</code> - Users access FinSpace through Single Sign On (SSO) via your Identity provider.</p> </li>   <li> <p> <code>LOCAL</code> - Users access FinSpace via email and password managed within the FinSpace environment.</p> </li>  </ul>
    ///   - [`federation_parameters(FederationParameters)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::federation_parameters) / [`set_federation_parameters(Option<FederationParameters>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_federation_parameters): <p>Configuration information when authentication mode is FEDERATED.</p>
    /// - On success, responds with [`UpdateEnvironmentOutput`](crate::operation::update_environment::UpdateEnvironmentOutput) with field(s):
    ///   - [`environment(Option<Environment>)`](crate::operation::update_environment::UpdateEnvironmentOutput::environment): <p>Returns the FinSpace environment object.</p>
    /// - On failure, responds with [`SdkError<UpdateEnvironmentError>`](crate::operation::update_environment::UpdateEnvironmentError)
    pub fn update_environment(
        &self,
    ) -> crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder {
        crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
