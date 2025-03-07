// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteUser`](crate::operation::delete_user::builders::DeleteUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl ::std::convert::Into<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::set_user_name): <p>The name of the user that you want to delete.</p>
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::set_aws_account_id): <p>The ID for the Amazon Web Services account that the user is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    ///   - [`namespace(impl ::std::convert::Into<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::set_namespace): <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    /// - On success, responds with [`DeleteUserOutput`](crate::operation::delete_user::DeleteUserOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::delete_user::DeleteUserOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::delete_user::DeleteUserOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<DeleteUserError>`](crate::operation::delete_user::DeleteUserError)
    pub fn delete_user(&self) -> crate::operation::delete_user::builders::DeleteUserFluentBuilder {
        crate::operation::delete_user::builders::DeleteUserFluentBuilder::new(self.handle.clone())
    }
}
