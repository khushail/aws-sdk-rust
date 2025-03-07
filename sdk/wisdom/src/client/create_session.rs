// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSession`](crate::operation::create_session::builders::CreateSessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::set_client_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    ///   - [`assistant_id(impl ::std::convert::Into<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::assistant_id) / [`set_assistant_id(Option<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::set_assistant_id): <p>The identifier of the Wisdom assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::set_name): <p>The name of the session.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::set_description): <p>The description.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_session::builders::CreateSessionFluentBuilder::set_tags): <p>The tags used to organize, track, or control access for this resource.</p>
    /// - On success, responds with [`CreateSessionOutput`](crate::operation::create_session::CreateSessionOutput) with field(s):
    ///   - [`session(Option<SessionData>)`](crate::operation::create_session::CreateSessionOutput::session): <p>The session.</p>
    /// - On failure, responds with [`SdkError<CreateSessionError>`](crate::operation::create_session::CreateSessionError)
    pub fn create_session(
        &self,
    ) -> crate::operation::create_session::builders::CreateSessionFluentBuilder {
        crate::operation::create_session::builders::CreateSessionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
