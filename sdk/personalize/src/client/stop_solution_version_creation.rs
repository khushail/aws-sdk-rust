// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopSolutionVersionCreation`](crate::operation::stop_solution_version_creation::builders::StopSolutionVersionCreationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`solution_version_arn(impl ::std::convert::Into<String>)`](crate::operation::stop_solution_version_creation::builders::StopSolutionVersionCreationFluentBuilder::solution_version_arn) / [`set_solution_version_arn(Option<String>)`](crate::operation::stop_solution_version_creation::builders::StopSolutionVersionCreationFluentBuilder::set_solution_version_arn): <p>The Amazon Resource Name (ARN) of the solution version you want to stop creating.</p>
    /// - On success, responds with [`StopSolutionVersionCreationOutput`](crate::operation::stop_solution_version_creation::StopSolutionVersionCreationOutput)
    /// - On failure, responds with [`SdkError<StopSolutionVersionCreationError>`](crate::operation::stop_solution_version_creation::StopSolutionVersionCreationError)
    pub fn stop_solution_version_creation(&self) -> crate::operation::stop_solution_version_creation::builders::StopSolutionVersionCreationFluentBuilder{
        crate::operation::stop_solution_version_creation::builders::StopSolutionVersionCreationFluentBuilder::new(self.handle.clone())
    }
}
