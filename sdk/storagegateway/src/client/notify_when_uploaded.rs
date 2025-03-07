// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`NotifyWhenUploaded`](crate::operation::notify_when_uploaded::builders::NotifyWhenUploadedFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`file_share_arn(impl ::std::convert::Into<String>)`](crate::operation::notify_when_uploaded::builders::NotifyWhenUploadedFluentBuilder::file_share_arn) / [`set_file_share_arn(Option<String>)`](crate::operation::notify_when_uploaded::builders::NotifyWhenUploadedFluentBuilder::set_file_share_arn): <p>The Amazon Resource Name (ARN) of the file share.</p>
    /// - On success, responds with [`NotifyWhenUploadedOutput`](crate::operation::notify_when_uploaded::NotifyWhenUploadedOutput) with field(s):
    ///   - [`file_share_arn(Option<String>)`](crate::operation::notify_when_uploaded::NotifyWhenUploadedOutput::file_share_arn): <p>The Amazon Resource Name (ARN) of the file share.</p>
    ///   - [`notification_id(Option<String>)`](crate::operation::notify_when_uploaded::NotifyWhenUploadedOutput::notification_id): <p>The randomly generated ID of the notification that was sent. This ID is in UUID format.</p>
    /// - On failure, responds with [`SdkError<NotifyWhenUploadedError>`](crate::operation::notify_when_uploaded::NotifyWhenUploadedError)
    pub fn notify_when_uploaded(
        &self,
    ) -> crate::operation::notify_when_uploaded::builders::NotifyWhenUploadedFluentBuilder {
        crate::operation::notify_when_uploaded::builders::NotifyWhenUploadedFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
