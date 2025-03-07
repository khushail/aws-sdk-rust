// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the reference data source configured for an application.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReferenceDataSourceDescription {
    /// <p>ID of the reference data source. This is the ID that Amazon Kinesis Analytics assigns when you add the reference data source to your application using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation.</p>
    #[doc(hidden)]
    pub reference_id: ::std::option::Option<::std::string::String>,
    /// <p>The in-application table name created by the specific reference data source configuration.</p>
    #[doc(hidden)]
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>Provides the S3 bucket name, the object key name that contains the reference data. It also provides the Amazon Resource Name (ARN) of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application reference table.</p>
    #[doc(hidden)]
    pub s3_reference_data_source_description:
        ::std::option::Option<crate::types::S3ReferenceDataSourceDescription>,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    #[doc(hidden)]
    pub reference_schema: ::std::option::Option<crate::types::SourceSchema>,
}
impl ReferenceDataSourceDescription {
    /// <p>ID of the reference data source. This is the ID that Amazon Kinesis Analytics assigns when you add the reference data source to your application using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation.</p>
    pub fn reference_id(&self) -> ::std::option::Option<&str> {
        self.reference_id.as_deref()
    }
    /// <p>The in-application table name created by the specific reference data source configuration.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>Provides the S3 bucket name, the object key name that contains the reference data. It also provides the Amazon Resource Name (ARN) of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application reference table.</p>
    pub fn s3_reference_data_source_description(
        &self,
    ) -> ::std::option::Option<&crate::types::S3ReferenceDataSourceDescription> {
        self.s3_reference_data_source_description.as_ref()
    }
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    pub fn reference_schema(&self) -> ::std::option::Option<&crate::types::SourceSchema> {
        self.reference_schema.as_ref()
    }
}
impl ReferenceDataSourceDescription {
    /// Creates a new builder-style object to manufacture [`ReferenceDataSourceDescription`](crate::types::ReferenceDataSourceDescription).
    pub fn builder() -> crate::types::builders::ReferenceDataSourceDescriptionBuilder {
        crate::types::builders::ReferenceDataSourceDescriptionBuilder::default()
    }
}

/// A builder for [`ReferenceDataSourceDescription`](crate::types::ReferenceDataSourceDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReferenceDataSourceDescriptionBuilder {
    pub(crate) reference_id: ::std::option::Option<::std::string::String>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) s3_reference_data_source_description:
        ::std::option::Option<crate::types::S3ReferenceDataSourceDescription>,
    pub(crate) reference_schema: ::std::option::Option<crate::types::SourceSchema>,
}
impl ReferenceDataSourceDescriptionBuilder {
    /// <p>ID of the reference data source. This is the ID that Amazon Kinesis Analytics assigns when you add the reference data source to your application using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation.</p>
    pub fn reference_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reference_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ID of the reference data source. This is the ID that Amazon Kinesis Analytics assigns when you add the reference data source to your application using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation.</p>
    pub fn set_reference_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reference_id = input;
        self
    }
    /// <p>The in-application table name created by the specific reference data source configuration.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The in-application table name created by the specific reference data source configuration.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>Provides the S3 bucket name, the object key name that contains the reference data. It also provides the Amazon Resource Name (ARN) of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application reference table.</p>
    pub fn s3_reference_data_source_description(
        mut self,
        input: crate::types::S3ReferenceDataSourceDescription,
    ) -> Self {
        self.s3_reference_data_source_description = ::std::option::Option::Some(input);
        self
    }
    /// <p>Provides the S3 bucket name, the object key name that contains the reference data. It also provides the Amazon Resource Name (ARN) of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application reference table.</p>
    pub fn set_s3_reference_data_source_description(
        mut self,
        input: ::std::option::Option<crate::types::S3ReferenceDataSourceDescription>,
    ) -> Self {
        self.s3_reference_data_source_description = input;
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
    /// Consumes the builder and constructs a [`ReferenceDataSourceDescription`](crate::types::ReferenceDataSourceDescription).
    pub fn build(self) -> crate::types::ReferenceDataSourceDescription {
        crate::types::ReferenceDataSourceDescription {
            reference_id: self.reference_id,
            table_name: self.table_name,
            s3_reference_data_source_description: self.s3_reference_data_source_description,
            reference_schema: self.reference_schema,
        }
    }
}
