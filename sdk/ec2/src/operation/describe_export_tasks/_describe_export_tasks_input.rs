// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExportTasksInput {
    /// <p>The export task IDs.</p>
    #[doc(hidden)]
    pub export_task_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>the filters for the export tasks.</p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
}
impl DescribeExportTasksInput {
    /// <p>The export task IDs.</p>
    pub fn export_task_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.export_task_ids.as_deref()
    }
    /// <p>the filters for the export tasks.</p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
}
impl DescribeExportTasksInput {
    /// Creates a new builder-style object to manufacture [`DescribeExportTasksInput`](crate::operation::describe_export_tasks::DescribeExportTasksInput).
    pub fn builder(
    ) -> crate::operation::describe_export_tasks::builders::DescribeExportTasksInputBuilder {
        crate::operation::describe_export_tasks::builders::DescribeExportTasksInputBuilder::default(
        )
    }
}

/// A builder for [`DescribeExportTasksInput`](crate::operation::describe_export_tasks::DescribeExportTasksInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeExportTasksInputBuilder {
    pub(crate) export_task_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
}
impl DescribeExportTasksInputBuilder {
    /// Appends an item to `export_task_ids`.
    ///
    /// To override the contents of this collection use [`set_export_task_ids`](Self::set_export_task_ids).
    ///
    /// <p>The export task IDs.</p>
    pub fn export_task_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.export_task_ids.unwrap_or_default();
        v.push(input.into());
        self.export_task_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The export task IDs.</p>
    pub fn set_export_task_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.export_task_ids = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>the filters for the export tasks.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>the filters for the export tasks.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeExportTasksInput`](crate::operation::describe_export_tasks::DescribeExportTasksInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_export_tasks::DescribeExportTasksInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_export_tasks::DescribeExportTasksInput {
                export_task_ids: self.export_task_ids,
                filters: self.filters,
            },
        )
    }
}
