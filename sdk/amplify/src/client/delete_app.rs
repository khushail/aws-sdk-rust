// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteApp`](crate::operation::delete_app::builders::DeleteAppFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::set_app_id): <p> The unique ID for an Amplify app. </p>
    /// - On success, responds with [`DeleteAppOutput`](crate::operation::delete_app::DeleteAppOutput) with field(s):
    ///   - [`app(Option<App>)`](crate::operation::delete_app::DeleteAppOutput::app): <p> Represents the different branches of a repository for building, deploying, and hosting an Amplify app. </p>
    /// - On failure, responds with [`SdkError<DeleteAppError>`](crate::operation::delete_app::DeleteAppError)
    pub fn delete_app(&self) -> crate::operation::delete_app::builders::DeleteAppFluentBuilder {
        crate::operation::delete_app::builders::DeleteAppFluentBuilder::new(self.handle.clone())
    }
}
