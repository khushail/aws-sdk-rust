// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteUser`](crate::operation::delete_user::builders::DeleteUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`server_id(impl ::std::convert::Into<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::server_id) / [`set_server_id(Option<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::set_server_id): <p>A system-assigned unique identifier for a server instance that has the user assigned to it.</p>
    ///   - [`user_name(impl ::std::convert::Into<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::set_user_name): <p>A unique string that identifies a user that is being deleted from a server.</p>
    /// - On success, responds with [`DeleteUserOutput`](crate::operation::delete_user::DeleteUserOutput)
    /// - On failure, responds with [`SdkError<DeleteUserError>`](crate::operation::delete_user::DeleteUserError)
    pub fn delete_user(&self) -> crate::operation::delete_user::builders::DeleteUserFluentBuilder {
        crate::operation::delete_user::builders::DeleteUserFluentBuilder::new(self.handle.clone())
    }
}
