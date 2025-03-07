// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the properties of the exported table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportDescription {
    /// <p>The Amazon Resource Name (ARN) of the table export.</p>
    #[doc(hidden)]
    pub export_arn: ::std::option::Option<::std::string::String>,
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    #[doc(hidden)]
    pub export_status: ::std::option::Option<crate::types::ExportStatus>,
    /// <p>The time at which the export task began.</p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time at which the export task completed.</p>
    #[doc(hidden)]
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The name of the manifest file for the export task.</p>
    #[doc(hidden)]
    pub export_manifest: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the table that was exported.</p>
    #[doc(hidden)]
    pub table_arn: ::std::option::Option<::std::string::String>,
    /// <p>Unique ID of the table that was exported.</p>
    #[doc(hidden)]
    pub table_id: ::std::option::Option<::std::string::String>,
    /// <p>Point in time from which table data was exported.</p>
    #[doc(hidden)]
    pub export_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The client token that was provided for the export task. A client token makes calls to <code>ExportTableToPointInTimeInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Amazon S3 bucket containing the export.</p>
    #[doc(hidden)]
    pub s3_bucket: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the bucket containing the export.</p>
    #[doc(hidden)]
    pub s3_bucket_owner: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon S3 bucket prefix used as the file name and path of the exported snapshot.</p>
    #[doc(hidden)]
    pub s3_prefix: ::std::option::Option<::std::string::String>,
    /// <p>Type of encryption used on the bucket where export data is stored. Valid values for <code>S3SseAlgorithm</code> are:</p>
    /// <ul>
    /// <li> <p> <code>AES256</code> - server-side encryption with Amazon S3 managed keys</p> </li>
    /// <li> <p> <code>KMS</code> - server-side encryption with KMS managed keys</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub s3_sse_algorithm: ::std::option::Option<crate::types::S3SseAlgorithm>,
    /// <p>The ID of the KMS managed key used to encrypt the S3 bucket where export data is stored (if applicable).</p>
    #[doc(hidden)]
    pub s3_sse_kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Status code for the result of the failed export.</p>
    #[doc(hidden)]
    pub failure_code: ::std::option::Option<::std::string::String>,
    /// <p>Export failure reason description.</p>
    #[doc(hidden)]
    pub failure_message: ::std::option::Option<::std::string::String>,
    /// <p>The format of the exported data. Valid values for <code>ExportFormat</code> are <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    #[doc(hidden)]
    pub export_format: ::std::option::Option<crate::types::ExportFormat>,
    /// <p>The billable size of the table export.</p>
    #[doc(hidden)]
    pub billed_size_bytes: ::std::option::Option<i64>,
    /// <p>The number of items exported.</p>
    #[doc(hidden)]
    pub item_count: ::std::option::Option<i64>,
}
impl ExportDescription {
    /// <p>The Amazon Resource Name (ARN) of the table export.</p>
    pub fn export_arn(&self) -> ::std::option::Option<&str> {
        self.export_arn.as_deref()
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn export_status(&self) -> ::std::option::Option<&crate::types::ExportStatus> {
        self.export_status.as_ref()
    }
    /// <p>The time at which the export task began.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The time at which the export task completed.</p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>The name of the manifest file for the export task.</p>
    pub fn export_manifest(&self) -> ::std::option::Option<&str> {
        self.export_manifest.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the table that was exported.</p>
    pub fn table_arn(&self) -> ::std::option::Option<&str> {
        self.table_arn.as_deref()
    }
    /// <p>Unique ID of the table that was exported.</p>
    pub fn table_id(&self) -> ::std::option::Option<&str> {
        self.table_id.as_deref()
    }
    /// <p>Point in time from which table data was exported.</p>
    pub fn export_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.export_time.as_ref()
    }
    /// <p>The client token that was provided for the export task. A client token makes calls to <code>ExportTableToPointInTimeInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The name of the Amazon S3 bucket containing the export.</p>
    pub fn s3_bucket(&self) -> ::std::option::Option<&str> {
        self.s3_bucket.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the bucket containing the export.</p>
    pub fn s3_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.s3_bucket_owner.as_deref()
    }
    /// <p>The Amazon S3 bucket prefix used as the file name and path of the exported snapshot.</p>
    pub fn s3_prefix(&self) -> ::std::option::Option<&str> {
        self.s3_prefix.as_deref()
    }
    /// <p>Type of encryption used on the bucket where export data is stored. Valid values for <code>S3SseAlgorithm</code> are:</p>
    /// <ul>
    /// <li> <p> <code>AES256</code> - server-side encryption with Amazon S3 managed keys</p> </li>
    /// <li> <p> <code>KMS</code> - server-side encryption with KMS managed keys</p> </li>
    /// </ul>
    pub fn s3_sse_algorithm(&self) -> ::std::option::Option<&crate::types::S3SseAlgorithm> {
        self.s3_sse_algorithm.as_ref()
    }
    /// <p>The ID of the KMS managed key used to encrypt the S3 bucket where export data is stored (if applicable).</p>
    pub fn s3_sse_kms_key_id(&self) -> ::std::option::Option<&str> {
        self.s3_sse_kms_key_id.as_deref()
    }
    /// <p>Status code for the result of the failed export.</p>
    pub fn failure_code(&self) -> ::std::option::Option<&str> {
        self.failure_code.as_deref()
    }
    /// <p>Export failure reason description.</p>
    pub fn failure_message(&self) -> ::std::option::Option<&str> {
        self.failure_message.as_deref()
    }
    /// <p>The format of the exported data. Valid values for <code>ExportFormat</code> are <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn export_format(&self) -> ::std::option::Option<&crate::types::ExportFormat> {
        self.export_format.as_ref()
    }
    /// <p>The billable size of the table export.</p>
    pub fn billed_size_bytes(&self) -> ::std::option::Option<i64> {
        self.billed_size_bytes
    }
    /// <p>The number of items exported.</p>
    pub fn item_count(&self) -> ::std::option::Option<i64> {
        self.item_count
    }
}
impl ExportDescription {
    /// Creates a new builder-style object to manufacture [`ExportDescription`](crate::types::ExportDescription).
    pub fn builder() -> crate::types::builders::ExportDescriptionBuilder {
        crate::types::builders::ExportDescriptionBuilder::default()
    }
}

/// A builder for [`ExportDescription`](crate::types::ExportDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExportDescriptionBuilder {
    pub(crate) export_arn: ::std::option::Option<::std::string::String>,
    pub(crate) export_status: ::std::option::Option<crate::types::ExportStatus>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) export_manifest: ::std::option::Option<::std::string::String>,
    pub(crate) table_arn: ::std::option::Option<::std::string::String>,
    pub(crate) table_id: ::std::option::Option<::std::string::String>,
    pub(crate) export_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) s3_bucket_owner: ::std::option::Option<::std::string::String>,
    pub(crate) s3_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) s3_sse_algorithm: ::std::option::Option<crate::types::S3SseAlgorithm>,
    pub(crate) s3_sse_kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) failure_code: ::std::option::Option<::std::string::String>,
    pub(crate) failure_message: ::std::option::Option<::std::string::String>,
    pub(crate) export_format: ::std::option::Option<crate::types::ExportFormat>,
    pub(crate) billed_size_bytes: ::std::option::Option<i64>,
    pub(crate) item_count: ::std::option::Option<i64>,
}
impl ExportDescriptionBuilder {
    /// <p>The Amazon Resource Name (ARN) of the table export.</p>
    pub fn export_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.export_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the table export.</p>
    pub fn set_export_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.export_arn = input;
        self
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn export_status(mut self, input: crate::types::ExportStatus) -> Self {
        self.export_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn set_export_status(
        mut self,
        input: ::std::option::Option<crate::types::ExportStatus>,
    ) -> Self {
        self.export_status = input;
        self
    }
    /// <p>The time at which the export task began.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the export task began.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The time at which the export task completed.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the export task completed.</p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.end_time = input;
        self
    }
    /// <p>The name of the manifest file for the export task.</p>
    pub fn export_manifest(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.export_manifest = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the manifest file for the export task.</p>
    pub fn set_export_manifest(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.export_manifest = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the table that was exported.</p>
    pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the table that was exported.</p>
    pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_arn = input;
        self
    }
    /// <p>Unique ID of the table that was exported.</p>
    pub fn table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique ID of the table that was exported.</p>
    pub fn set_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_id = input;
        self
    }
    /// <p>Point in time from which table data was exported.</p>
    pub fn export_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.export_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Point in time from which table data was exported.</p>
    pub fn set_export_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.export_time = input;
        self
    }
    /// <p>The client token that was provided for the export task. A client token makes calls to <code>ExportTableToPointInTimeInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The client token that was provided for the export task. A client token makes calls to <code>ExportTableToPointInTimeInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The name of the Amazon S3 bucket containing the export.</p>
    pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket containing the export.</p>
    pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_bucket = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the bucket containing the export.</p>
    pub fn s3_bucket_owner(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.s3_bucket_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the bucket containing the export.</p>
    pub fn set_s3_bucket_owner(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.s3_bucket_owner = input;
        self
    }
    /// <p>The Amazon S3 bucket prefix used as the file name and path of the exported snapshot.</p>
    pub fn s3_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon S3 bucket prefix used as the file name and path of the exported snapshot.</p>
    pub fn set_s3_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_prefix = input;
        self
    }
    /// <p>Type of encryption used on the bucket where export data is stored. Valid values for <code>S3SseAlgorithm</code> are:</p>
    /// <ul>
    /// <li> <p> <code>AES256</code> - server-side encryption with Amazon S3 managed keys</p> </li>
    /// <li> <p> <code>KMS</code> - server-side encryption with KMS managed keys</p> </li>
    /// </ul>
    pub fn s3_sse_algorithm(mut self, input: crate::types::S3SseAlgorithm) -> Self {
        self.s3_sse_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// <p>Type of encryption used on the bucket where export data is stored. Valid values for <code>S3SseAlgorithm</code> are:</p>
    /// <ul>
    /// <li> <p> <code>AES256</code> - server-side encryption with Amazon S3 managed keys</p> </li>
    /// <li> <p> <code>KMS</code> - server-side encryption with KMS managed keys</p> </li>
    /// </ul>
    pub fn set_s3_sse_algorithm(
        mut self,
        input: ::std::option::Option<crate::types::S3SseAlgorithm>,
    ) -> Self {
        self.s3_sse_algorithm = input;
        self
    }
    /// <p>The ID of the KMS managed key used to encrypt the S3 bucket where export data is stored (if applicable).</p>
    pub fn s3_sse_kms_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.s3_sse_kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the KMS managed key used to encrypt the S3 bucket where export data is stored (if applicable).</p>
    pub fn set_s3_sse_kms_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.s3_sse_kms_key_id = input;
        self
    }
    /// <p>Status code for the result of the failed export.</p>
    pub fn failure_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.failure_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Status code for the result of the failed export.</p>
    pub fn set_failure_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.failure_code = input;
        self
    }
    /// <p>Export failure reason description.</p>
    pub fn failure_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.failure_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Export failure reason description.</p>
    pub fn set_failure_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.failure_message = input;
        self
    }
    /// <p>The format of the exported data. Valid values for <code>ExportFormat</code> are <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn export_format(mut self, input: crate::types::ExportFormat) -> Self {
        self.export_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>The format of the exported data. Valid values for <code>ExportFormat</code> are <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn set_export_format(
        mut self,
        input: ::std::option::Option<crate::types::ExportFormat>,
    ) -> Self {
        self.export_format = input;
        self
    }
    /// <p>The billable size of the table export.</p>
    pub fn billed_size_bytes(mut self, input: i64) -> Self {
        self.billed_size_bytes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The billable size of the table export.</p>
    pub fn set_billed_size_bytes(mut self, input: ::std::option::Option<i64>) -> Self {
        self.billed_size_bytes = input;
        self
    }
    /// <p>The number of items exported.</p>
    pub fn item_count(mut self, input: i64) -> Self {
        self.item_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of items exported.</p>
    pub fn set_item_count(mut self, input: ::std::option::Option<i64>) -> Self {
        self.item_count = input;
        self
    }
    /// Consumes the builder and constructs a [`ExportDescription`](crate::types::ExportDescription).
    pub fn build(self) -> crate::types::ExportDescription {
        crate::types::ExportDescription {
            export_arn: self.export_arn,
            export_status: self.export_status,
            start_time: self.start_time,
            end_time: self.end_time,
            export_manifest: self.export_manifest,
            table_arn: self.table_arn,
            table_id: self.table_id,
            export_time: self.export_time,
            client_token: self.client_token,
            s3_bucket: self.s3_bucket,
            s3_bucket_owner: self.s3_bucket_owner,
            s3_prefix: self.s3_prefix,
            s3_sse_algorithm: self.s3_sse_algorithm,
            s3_sse_kms_key_id: self.s3_sse_kms_key_id,
            failure_code: self.failure_code,
            failure_message: self.failure_message,
            export_format: self.export_format,
            billed_size_bytes: self.billed_size_bytes,
            item_count: self.item_count,
        }
    }
}
