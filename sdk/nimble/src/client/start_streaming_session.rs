// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartStreamingSession`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    ///   - [`session_id(impl ::std::convert::Into<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::session_id) / [`set_session_id(Option<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::set_session_id): <p>The streaming session ID for the <code>StartStreamingSessionRequest</code>.</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::set_studio_id): <p>The studio ID for the StartStreamingSessionRequest.</p>
    ///   - [`backup_id(impl ::std::convert::Into<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::backup_id) / [`set_backup_id(Option<String>)`](crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::set_backup_id): <p>The ID of the backup.</p>
    /// - On success, responds with [`StartStreamingSessionOutput`](crate::operation::start_streaming_session::StartStreamingSessionOutput) with field(s):
    ///   - [`session(Option<StreamingSession>)`](crate::operation::start_streaming_session::StartStreamingSessionOutput::session): <p>A streaming session is a virtual workstation created using a particular launch profile.</p>
    /// - On failure, responds with [`SdkError<StartStreamingSessionError>`](crate::operation::start_streaming_session::StartStreamingSessionError)
    pub fn start_streaming_session(
        &self,
    ) -> crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder
    {
        crate::operation::start_streaming_session::builders::StartStreamingSessionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
