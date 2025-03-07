// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDatasetGroupInput {
    /// <p>The ARN of the dataset group to delete.</p>
    #[doc(hidden)]
    pub dataset_group_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteDatasetGroupInput {
    /// <p>The ARN of the dataset group to delete.</p>
    pub fn dataset_group_arn(&self) -> ::std::option::Option<&str> {
        self.dataset_group_arn.as_deref()
    }
}
impl DeleteDatasetGroupInput {
    /// Creates a new builder-style object to manufacture [`DeleteDatasetGroupInput`](crate::operation::delete_dataset_group::DeleteDatasetGroupInput).
    pub fn builder(
    ) -> crate::operation::delete_dataset_group::builders::DeleteDatasetGroupInputBuilder {
        crate::operation::delete_dataset_group::builders::DeleteDatasetGroupInputBuilder::default()
    }
}

/// A builder for [`DeleteDatasetGroupInput`](crate::operation::delete_dataset_group::DeleteDatasetGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteDatasetGroupInputBuilder {
    pub(crate) dataset_group_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteDatasetGroupInputBuilder {
    /// <p>The ARN of the dataset group to delete.</p>
    pub fn dataset_group_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.dataset_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the dataset group to delete.</p>
    pub fn set_dataset_group_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.dataset_group_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteDatasetGroupInput`](crate::operation::delete_dataset_group::DeleteDatasetGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_dataset_group::DeleteDatasetGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_dataset_group::DeleteDatasetGroupInput {
                dataset_group_arn: self.dataset_group_arn,
            },
        )
    }
}
