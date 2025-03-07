// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExportTasksOutput {
    /// <p>A pagination token that can be used in a later <code>DescribeExportTasks</code> request. A marker is used for pagination to identify the location to begin output for the next response of <code>DescribeExportTasks</code>.</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>Information about an export of a snapshot or cluster to Amazon S3.</p>
    #[doc(hidden)]
    pub export_tasks: ::std::option::Option<::std::vec::Vec<crate::types::ExportTask>>,
    _request_id: Option<String>,
}
impl DescribeExportTasksOutput {
    /// <p>A pagination token that can be used in a later <code>DescribeExportTasks</code> request. A marker is used for pagination to identify the location to begin output for the next response of <code>DescribeExportTasks</code>.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>Information about an export of a snapshot or cluster to Amazon S3.</p>
    pub fn export_tasks(&self) -> ::std::option::Option<&[crate::types::ExportTask]> {
        self.export_tasks.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeExportTasksOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeExportTasksOutput {
    /// Creates a new builder-style object to manufacture [`DescribeExportTasksOutput`](crate::operation::describe_export_tasks::DescribeExportTasksOutput).
    pub fn builder(
    ) -> crate::operation::describe_export_tasks::builders::DescribeExportTasksOutputBuilder {
        crate::operation::describe_export_tasks::builders::DescribeExportTasksOutputBuilder::default(
        )
    }
}

/// A builder for [`DescribeExportTasksOutput`](crate::operation::describe_export_tasks::DescribeExportTasksOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeExportTasksOutputBuilder {
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) export_tasks: ::std::option::Option<::std::vec::Vec<crate::types::ExportTask>>,
    _request_id: Option<String>,
}
impl DescribeExportTasksOutputBuilder {
    /// <p>A pagination token that can be used in a later <code>DescribeExportTasks</code> request. A marker is used for pagination to identify the location to begin output for the next response of <code>DescribeExportTasks</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A pagination token that can be used in a later <code>DescribeExportTasks</code> request. A marker is used for pagination to identify the location to begin output for the next response of <code>DescribeExportTasks</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// Appends an item to `export_tasks`.
    ///
    /// To override the contents of this collection use [`set_export_tasks`](Self::set_export_tasks).
    ///
    /// <p>Information about an export of a snapshot or cluster to Amazon S3.</p>
    pub fn export_tasks(mut self, input: crate::types::ExportTask) -> Self {
        let mut v = self.export_tasks.unwrap_or_default();
        v.push(input);
        self.export_tasks = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about an export of a snapshot or cluster to Amazon S3.</p>
    pub fn set_export_tasks(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ExportTask>>,
    ) -> Self {
        self.export_tasks = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeExportTasksOutput`](crate::operation::describe_export_tasks::DescribeExportTasksOutput).
    pub fn build(self) -> crate::operation::describe_export_tasks::DescribeExportTasksOutput {
        crate::operation::describe_export_tasks::DescribeExportTasksOutput {
            marker: self.marker,
            export_tasks: self.export_tasks,
            _request_id: self._request_id,
        }
    }
}
