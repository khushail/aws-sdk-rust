// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RebuildEnvironment`](crate::operation::rebuild_environment::builders::RebuildEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`environment_id(impl ::std::convert::Into<String>)`](crate::operation::rebuild_environment::builders::RebuildEnvironmentFluentBuilder::environment_id) / [`set_environment_id(Option<String>)`](crate::operation::rebuild_environment::builders::RebuildEnvironmentFluentBuilder::set_environment_id): <p>The ID of the environment to rebuild.</p>  <p> Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    ///   - [`environment_name(impl ::std::convert::Into<String>)`](crate::operation::rebuild_environment::builders::RebuildEnvironmentFluentBuilder::environment_name) / [`set_environment_name(Option<String>)`](crate::operation::rebuild_environment::builders::RebuildEnvironmentFluentBuilder::set_environment_name): <p>The name of the environment to rebuild.</p>  <p> Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    /// - On success, responds with [`RebuildEnvironmentOutput`](crate::operation::rebuild_environment::RebuildEnvironmentOutput)
    /// - On failure, responds with [`SdkError<RebuildEnvironmentError>`](crate::operation::rebuild_environment::RebuildEnvironmentError)
    pub fn rebuild_environment(
        &self,
    ) -> crate::operation::rebuild_environment::builders::RebuildEnvironmentFluentBuilder {
        crate::operation::rebuild_environment::builders::RebuildEnvironmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
