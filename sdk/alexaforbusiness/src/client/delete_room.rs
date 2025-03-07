// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRoom`](crate::operation::delete_room::builders::DeleteRoomFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`room_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_room::builders::DeleteRoomFluentBuilder::room_arn) / [`set_room_arn(Option<String>)`](crate::operation::delete_room::builders::DeleteRoomFluentBuilder::set_room_arn): <p>The ARN of the room to delete. Required.</p>
    /// - On success, responds with [`DeleteRoomOutput`](crate::operation::delete_room::DeleteRoomOutput)
    /// - On failure, responds with [`SdkError<DeleteRoomError>`](crate::operation::delete_room::DeleteRoomError)
    pub fn delete_room(&self) -> crate::operation::delete_room::builders::DeleteRoomFluentBuilder {
        crate::operation::delete_room::builders::DeleteRoomFluentBuilder::new(self.handle.clone())
    }
}
