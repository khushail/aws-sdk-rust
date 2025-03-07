// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateApplication`](crate::operation::create_application::builders::CreateApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_application::builders::CreateApplicationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_application::builders::CreateApplicationFluentBuilder::set_name): <p>A name for the application.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_application::builders::CreateApplicationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_application::builders::CreateApplicationFluentBuilder::set_description): <p>A description of the application.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_application::builders::CreateApplicationFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_application::builders::CreateApplicationFluentBuilder::set_tags): <p>Metadata to assign to the application. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    /// - On success, responds with [`CreateApplicationOutput`](crate::operation::create_application::CreateApplicationOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::create_application::CreateApplicationOutput::id): <p>The application ID.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_application::CreateApplicationOutput::name): <p>The application name.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_application::CreateApplicationOutput::description): <p>The description of the application.</p>
    /// - On failure, responds with [`SdkError<CreateApplicationError>`](crate::operation::create_application::CreateApplicationError)
    pub fn create_application(
        &self,
    ) -> crate::operation::create_application::builders::CreateApplicationFluentBuilder {
        crate::operation::create_application::builders::CreateApplicationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
