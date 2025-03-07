// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRoom`](crate::operation::update_room::builders::UpdateRoomFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`room_arn(impl ::std::convert::Into<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::room_arn) / [`set_room_arn(Option<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::set_room_arn): <p>The ARN of the room to update. </p>
    ///   - [`room_name(impl ::std::convert::Into<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::room_name) / [`set_room_name(Option<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::set_room_name): <p>The updated name for the room.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::set_description): <p>The updated description for the room.</p>
    ///   - [`provider_calendar_id(impl ::std::convert::Into<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::provider_calendar_id) / [`set_provider_calendar_id(Option<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::set_provider_calendar_id): <p>The updated provider calendar ARN for the room.</p>
    ///   - [`profile_arn(impl ::std::convert::Into<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::profile_arn) / [`set_profile_arn(Option<String>)`](crate::operation::update_room::builders::UpdateRoomFluentBuilder::set_profile_arn): <p>The updated profile ARN for the room.</p>
    /// - On success, responds with [`UpdateRoomOutput`](crate::operation::update_room::UpdateRoomOutput)
    /// - On failure, responds with [`SdkError<UpdateRoomError>`](crate::operation::update_room::UpdateRoomError)
    pub fn update_room(&self) -> crate::operation::update_room::builders::UpdateRoomFluentBuilder {
        crate::operation::update_room::builders::UpdateRoomFluentBuilder::new(self.handle.clone())
    }
}
