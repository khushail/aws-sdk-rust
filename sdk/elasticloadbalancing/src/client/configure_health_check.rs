// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ConfigureHealthCheck`](crate::operation::configure_health_check::builders::ConfigureHealthCheckFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl ::std::convert::Into<String>)`](crate::operation::configure_health_check::builders::ConfigureHealthCheckFluentBuilder::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::operation::configure_health_check::builders::ConfigureHealthCheckFluentBuilder::set_load_balancer_name): <p>The name of the load balancer.</p>
    ///   - [`health_check(HealthCheck)`](crate::operation::configure_health_check::builders::ConfigureHealthCheckFluentBuilder::health_check) / [`set_health_check(Option<HealthCheck>)`](crate::operation::configure_health_check::builders::ConfigureHealthCheckFluentBuilder::set_health_check): <p>The configuration information.</p>
    /// - On success, responds with [`ConfigureHealthCheckOutput`](crate::operation::configure_health_check::ConfigureHealthCheckOutput) with field(s):
    ///   - [`health_check(Option<HealthCheck>)`](crate::operation::configure_health_check::ConfigureHealthCheckOutput::health_check): <p>The updated health check.</p>
    /// - On failure, responds with [`SdkError<ConfigureHealthCheckError>`](crate::operation::configure_health_check::ConfigureHealthCheckError)
    pub fn configure_health_check(
        &self,
    ) -> crate::operation::configure_health_check::builders::ConfigureHealthCheckFluentBuilder {
        crate::operation::configure_health_check::builders::ConfigureHealthCheckFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
