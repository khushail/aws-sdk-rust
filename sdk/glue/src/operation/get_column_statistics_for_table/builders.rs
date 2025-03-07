// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_column_statistics_for_table::_get_column_statistics_for_table_output::GetColumnStatisticsForTableOutputBuilder;

pub use crate::operation::get_column_statistics_for_table::_get_column_statistics_for_table_input::GetColumnStatisticsForTableInputBuilder;

/// Fluent builder constructing a request to `GetColumnStatisticsForTable`.
///
/// <p>Retrieves table statistics of columns.</p>
/// <p>The Identity and Access Management (IAM) permission required for this operation is <code>GetTable</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetColumnStatisticsForTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_column_statistics_for_table::builders::GetColumnStatisticsForTableInputBuilder,
}
impl GetColumnStatisticsForTableFluentBuilder {
    /// Creates a new `GetColumnStatisticsForTable`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTable,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTableError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTableOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTableError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTableOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTableError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTable,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_column_statistics_for_table::GetColumnStatisticsForTableError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the Amazon Web Services account ID is used by default.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the Amazon Web Services account ID is used by default.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The name of the catalog database where the partitions reside.</p>
    pub fn database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>The name of the catalog database where the partitions reside.</p>
    pub fn set_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>The name of the partitions' table.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the partitions' table.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// Appends an item to `ColumnNames`.
    ///
    /// To override the contents of this collection use [`set_column_names`](Self::set_column_names).
    ///
    /// <p>A list of the column names.</p>
    pub fn column_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.column_names(input.into());
        self
    }
    /// <p>A list of the column names.</p>
    pub fn set_column_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_column_names(input);
        self
    }
}
