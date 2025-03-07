// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>For a SQL-based Kinesis Data Analytics application, describes the reference data source by providing the source information (Amazon S3 bucket name and object key name), the resulting in-application table name that is created, and the necessary schema to map the data elements in the Amazon S3 object to the in-application table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReferenceDataSource {
    /// <p>The name of the in-application table to create.</p>
    #[doc(hidden)]
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>Identifies the S3 bucket and object that contains the reference data. A Kinesis Data Analytics application loads reference data only once. If the data changes, you call the <code>UpdateApplication</code> operation to trigger reloading of data into your application. </p>
    #[doc(hidden)]
    pub s3_reference_data_source: ::std::option::Option<crate::types::S3ReferenceDataSource>,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    #[doc(hidden)]
    pub reference_schema: ::std::option::Option<crate::types::SourceSchema>,
}
impl ReferenceDataSource {
    /// <p>The name of the in-application table to create.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>Identifies the S3 bucket and object that contains the reference data. A Kinesis Data Analytics application loads reference data only once. If the data changes, you call the <code>UpdateApplication</code> operation to trigger reloading of data into your application. </p>
    pub fn s3_reference_data_source(
        &self,
    ) -> ::std::option::Option<&crate::types::S3ReferenceDataSource> {
        self.s3_reference_data_source.as_ref()
    }
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    pub fn reference_schema(&self) -> ::std::option::Option<&crate::types::SourceSchema> {
        self.reference_schema.as_ref()
    }
}
impl ReferenceDataSource {
    /// Creates a new builder-style object to manufacture [`ReferenceDataSource`](crate::types::ReferenceDataSource).
    pub fn builder() -> crate::types::builders::ReferenceDataSourceBuilder {
        crate::types::builders::ReferenceDataSourceBuilder::default()
    }
}

/// A builder for [`ReferenceDataSource`](crate::types::ReferenceDataSource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReferenceDataSourceBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) s3_reference_data_source: ::std::option::Option<crate::types::S3ReferenceDataSource>,
    pub(crate) reference_schema: ::std::option::Option<crate::types::SourceSchema>,
}
impl ReferenceDataSourceBuilder {
    /// <p>The name of the in-application table to create.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the in-application table to create.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>Identifies the S3 bucket and object that contains the reference data. A Kinesis Data Analytics application loads reference data only once. If the data changes, you call the <code>UpdateApplication</code> operation to trigger reloading of data into your application. </p>
    pub fn s3_reference_data_source(mut self, input: crate::types::S3ReferenceDataSource) -> Self {
        self.s3_reference_data_source = ::std::option::Option::Some(input);
        self
    }
    /// <p>Identifies the S3 bucket and object that contains the reference data. A Kinesis Data Analytics application loads reference data only once. If the data changes, you call the <code>UpdateApplication</code> operation to trigger reloading of data into your application. </p>
    pub fn set_s3_reference_data_source(
        mut self,
        input: ::std::option::Option<crate::types::S3ReferenceDataSource>,
    ) -> Self {
        self.s3_reference_data_source = input;
        self
    }
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    pub fn reference_schema(mut self, input: crate::types::SourceSchema) -> Self {
        self.reference_schema = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    pub fn set_reference_schema(
        mut self,
        input: ::std::option::Option<crate::types::SourceSchema>,
    ) -> Self {
        self.reference_schema = input;
        self
    }
    /// Consumes the builder and constructs a [`ReferenceDataSource`](crate::types::ReferenceDataSource).
    pub fn build(self) -> crate::types::ReferenceDataSource {
        crate::types::ReferenceDataSource {
            table_name: self.table_name,
            s3_reference_data_source: self.s3_reference_data_source,
            reference_schema: self.reference_schema,
        }
    }
}
