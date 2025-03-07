// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeDbClusterSnapshotsOutput {
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterSnapshots</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>Provides a list of DB cluster snapshots for the user.</p>
    #[doc(hidden)]
    pub db_cluster_snapshots:
        ::std::option::Option<::std::vec::Vec<crate::types::DbClusterSnapshot>>,
    _request_id: Option<String>,
}
impl DescribeDbClusterSnapshotsOutput {
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterSnapshots</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>Provides a list of DB cluster snapshots for the user.</p>
    pub fn db_cluster_snapshots(
        &self,
    ) -> ::std::option::Option<&[crate::types::DbClusterSnapshot]> {
        self.db_cluster_snapshots.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeDbClusterSnapshotsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeDbClusterSnapshotsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDbClusterSnapshotsOutput`](crate::operation::describe_db_cluster_snapshots::DescribeDbClusterSnapshotsOutput).
    pub fn builder() -> crate::operation::describe_db_cluster_snapshots::builders::DescribeDbClusterSnapshotsOutputBuilder{
        crate::operation::describe_db_cluster_snapshots::builders::DescribeDbClusterSnapshotsOutputBuilder::default()
    }
}

/// A builder for [`DescribeDbClusterSnapshotsOutput`](crate::operation::describe_db_cluster_snapshots::DescribeDbClusterSnapshotsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeDbClusterSnapshotsOutputBuilder {
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) db_cluster_snapshots:
        ::std::option::Option<::std::vec::Vec<crate::types::DbClusterSnapshot>>,
    _request_id: Option<String>,
}
impl DescribeDbClusterSnapshotsOutputBuilder {
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterSnapshots</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterSnapshots</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// Appends an item to `db_cluster_snapshots`.
    ///
    /// To override the contents of this collection use [`set_db_cluster_snapshots`](Self::set_db_cluster_snapshots).
    ///
    /// <p>Provides a list of DB cluster snapshots for the user.</p>
    pub fn db_cluster_snapshots(mut self, input: crate::types::DbClusterSnapshot) -> Self {
        let mut v = self.db_cluster_snapshots.unwrap_or_default();
        v.push(input);
        self.db_cluster_snapshots = ::std::option::Option::Some(v);
        self
    }
    /// <p>Provides a list of DB cluster snapshots for the user.</p>
    pub fn set_db_cluster_snapshots(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DbClusterSnapshot>>,
    ) -> Self {
        self.db_cluster_snapshots = input;
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
    /// Consumes the builder and constructs a [`DescribeDbClusterSnapshotsOutput`](crate::operation::describe_db_cluster_snapshots::DescribeDbClusterSnapshotsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_db_cluster_snapshots::DescribeDbClusterSnapshotsOutput {
        crate::operation::describe_db_cluster_snapshots::DescribeDbClusterSnapshotsOutput {
            marker: self.marker,
            db_cluster_snapshots: self.db_cluster_snapshots,
            _request_id: self._request_id,
        }
    }
}
