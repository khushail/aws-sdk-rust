// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteObjectInput {
    /// Backup job Id for the in-progress backup.
    #[doc(hidden)]
    pub backup_job_id: ::std::option::Option<::std::string::String>,
    /// The name of the Object.
    #[doc(hidden)]
    pub object_name: ::std::option::Option<::std::string::String>,
}
impl DeleteObjectInput {
    /// Backup job Id for the in-progress backup.
    pub fn backup_job_id(&self) -> ::std::option::Option<&str> {
        self.backup_job_id.as_deref()
    }
    /// The name of the Object.
    pub fn object_name(&self) -> ::std::option::Option<&str> {
        self.object_name.as_deref()
    }
}
impl DeleteObjectInput {
    /// Creates a new builder-style object to manufacture [`DeleteObjectInput`](crate::operation::delete_object::DeleteObjectInput).
    pub fn builder() -> crate::operation::delete_object::builders::DeleteObjectInputBuilder {
        crate::operation::delete_object::builders::DeleteObjectInputBuilder::default()
    }
}

/// A builder for [`DeleteObjectInput`](crate::operation::delete_object::DeleteObjectInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteObjectInputBuilder {
    pub(crate) backup_job_id: ::std::option::Option<::std::string::String>,
    pub(crate) object_name: ::std::option::Option<::std::string::String>,
}
impl DeleteObjectInputBuilder {
    /// Backup job Id for the in-progress backup.
    pub fn backup_job_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.backup_job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Backup job Id for the in-progress backup.
    pub fn set_backup_job_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.backup_job_id = input;
        self
    }
    /// The name of the Object.
    pub fn object_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the Object.
    pub fn set_object_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteObjectInput`](crate::operation::delete_object::DeleteObjectInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_object::DeleteObjectInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_object::DeleteObjectInput {
            backup_job_id: self.backup_job_id,
            object_name: self.object_name,
        })
    }
}
