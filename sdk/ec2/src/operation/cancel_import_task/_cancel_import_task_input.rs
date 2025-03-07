// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelImportTaskInput {
    /// <p>The reason for canceling the task.</p>
    #[doc(hidden)]
    pub cancel_reason: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the import image or import snapshot task to be canceled.</p>
    #[doc(hidden)]
    pub import_task_id: ::std::option::Option<::std::string::String>,
}
impl CancelImportTaskInput {
    /// <p>The reason for canceling the task.</p>
    pub fn cancel_reason(&self) -> ::std::option::Option<&str> {
        self.cancel_reason.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the import image or import snapshot task to be canceled.</p>
    pub fn import_task_id(&self) -> ::std::option::Option<&str> {
        self.import_task_id.as_deref()
    }
}
impl CancelImportTaskInput {
    /// Creates a new builder-style object to manufacture [`CancelImportTaskInput`](crate::operation::cancel_import_task::CancelImportTaskInput).
    pub fn builder() -> crate::operation::cancel_import_task::builders::CancelImportTaskInputBuilder
    {
        crate::operation::cancel_import_task::builders::CancelImportTaskInputBuilder::default()
    }
}

/// A builder for [`CancelImportTaskInput`](crate::operation::cancel_import_task::CancelImportTaskInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelImportTaskInputBuilder {
    pub(crate) cancel_reason: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) import_task_id: ::std::option::Option<::std::string::String>,
}
impl CancelImportTaskInputBuilder {
    /// <p>The reason for canceling the task.</p>
    pub fn cancel_reason(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.cancel_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for canceling the task.</p>
    pub fn set_cancel_reason(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.cancel_reason = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The ID of the import image or import snapshot task to be canceled.</p>
    pub fn import_task_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.import_task_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the import image or import snapshot task to be canceled.</p>
    pub fn set_import_task_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.import_task_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelImportTaskInput`](crate::operation::cancel_import_task::CancelImportTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::cancel_import_task::CancelImportTaskInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::cancel_import_task::CancelImportTaskInput {
                cancel_reason: self.cancel_reason,
                dry_run: self.dry_run,
                import_task_id: self.import_task_id,
            },
        )
    }
}
