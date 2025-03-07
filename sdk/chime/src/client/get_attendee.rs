// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAttendee`](crate::operation::get_attendee::builders::GetAttendeeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`meeting_id(impl ::std::convert::Into<String>)`](crate::operation::get_attendee::builders::GetAttendeeFluentBuilder::meeting_id) / [`set_meeting_id(Option<String>)`](crate::operation::get_attendee::builders::GetAttendeeFluentBuilder::set_meeting_id): <p>The Amazon Chime SDK meeting ID.</p>
    ///   - [`attendee_id(impl ::std::convert::Into<String>)`](crate::operation::get_attendee::builders::GetAttendeeFluentBuilder::attendee_id) / [`set_attendee_id(Option<String>)`](crate::operation::get_attendee::builders::GetAttendeeFluentBuilder::set_attendee_id): <p>The Amazon Chime SDK attendee ID.</p>
    /// - On success, responds with [`GetAttendeeOutput`](crate::operation::get_attendee::GetAttendeeOutput) with field(s):
    ///   - [`attendee(Option<Attendee>)`](crate::operation::get_attendee::GetAttendeeOutput::attendee): <p>The Amazon Chime SDK attendee information.</p>
    /// - On failure, responds with [`SdkError<GetAttendeeError>`](crate::operation::get_attendee::GetAttendeeError)
    pub fn get_attendee(
        &self,
    ) -> crate::operation::get_attendee::builders::GetAttendeeFluentBuilder {
        crate::operation::get_attendee::builders::GetAttendeeFluentBuilder::new(self.handle.clone())
    }
}
