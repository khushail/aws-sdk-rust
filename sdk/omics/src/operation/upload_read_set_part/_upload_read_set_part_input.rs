// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub struct UploadReadSetPartInput {
    /// <p> The Sequence Store ID used for the multipart upload. </p>
    #[doc(hidden)]
    pub sequence_store_id: ::std::option::Option<::std::string::String>,
    /// <p> The ID for the initiated multipart upload. </p>
    #[doc(hidden)]
    pub upload_id: ::std::option::Option<::std::string::String>,
    /// <p> The source file for an upload part. </p>
    #[doc(hidden)]
    pub part_source: ::std::option::Option<crate::types::ReadSetPartSource>,
    /// <p> The number of the part being uploaded. </p>
    #[doc(hidden)]
    pub part_number: ::std::option::Option<i32>,
    /// <p> The read set data to upload for a part. </p>
    pub payload: ::aws_smithy_http::byte_stream::ByteStream,
}
impl UploadReadSetPartInput {
    /// <p> The Sequence Store ID used for the multipart upload. </p>
    pub fn sequence_store_id(&self) -> ::std::option::Option<&str> {
        self.sequence_store_id.as_deref()
    }
    /// <p> The ID for the initiated multipart upload. </p>
    pub fn upload_id(&self) -> ::std::option::Option<&str> {
        self.upload_id.as_deref()
    }
    /// <p> The source file for an upload part. </p>
    pub fn part_source(&self) -> ::std::option::Option<&crate::types::ReadSetPartSource> {
        self.part_source.as_ref()
    }
    /// <p> The number of the part being uploaded. </p>
    pub fn part_number(&self) -> ::std::option::Option<i32> {
        self.part_number
    }
    /// <p> The read set data to upload for a part. </p>
    pub fn payload(&self) -> &::aws_smithy_http::byte_stream::ByteStream {
        &self.payload
    }
}
impl UploadReadSetPartInput {
    /// Creates a new builder-style object to manufacture [`UploadReadSetPartInput`](crate::operation::upload_read_set_part::UploadReadSetPartInput).
    pub fn builder(
    ) -> crate::operation::upload_read_set_part::builders::UploadReadSetPartInputBuilder {
        crate::operation::upload_read_set_part::builders::UploadReadSetPartInputBuilder::default()
    }
}

/// A builder for [`UploadReadSetPartInput`](crate::operation::upload_read_set_part::UploadReadSetPartInput).
#[non_exhaustive]
#[derive(::std::default::Default, ::std::fmt::Debug)]
pub struct UploadReadSetPartInputBuilder {
    pub(crate) sequence_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) upload_id: ::std::option::Option<::std::string::String>,
    pub(crate) part_source: ::std::option::Option<crate::types::ReadSetPartSource>,
    pub(crate) part_number: ::std::option::Option<i32>,
    pub(crate) payload: ::std::option::Option<::aws_smithy_http::byte_stream::ByteStream>,
}
impl UploadReadSetPartInputBuilder {
    /// <p> The Sequence Store ID used for the multipart upload. </p>
    pub fn sequence_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sequence_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Sequence Store ID used for the multipart upload. </p>
    pub fn set_sequence_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sequence_store_id = input;
        self
    }
    /// <p> The ID for the initiated multipart upload. </p>
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID for the initiated multipart upload. </p>
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id = input;
        self
    }
    /// <p> The source file for an upload part. </p>
    pub fn part_source(mut self, input: crate::types::ReadSetPartSource) -> Self {
        self.part_source = ::std::option::Option::Some(input);
        self
    }
    /// <p> The source file for an upload part. </p>
    pub fn set_part_source(
        mut self,
        input: ::std::option::Option<crate::types::ReadSetPartSource>,
    ) -> Self {
        self.part_source = input;
        self
    }
    /// <p> The number of the part being uploaded. </p>
    pub fn part_number(mut self, input: i32) -> Self {
        self.part_number = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of the part being uploaded. </p>
    pub fn set_part_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.part_number = input;
        self
    }
    /// <p> The read set data to upload for a part. </p>
    pub fn payload(mut self, input: ::aws_smithy_http::byte_stream::ByteStream) -> Self {
        self.payload = ::std::option::Option::Some(input);
        self
    }
    /// <p> The read set data to upload for a part. </p>
    pub fn set_payload(
        mut self,
        input: ::std::option::Option<::aws_smithy_http::byte_stream::ByteStream>,
    ) -> Self {
        self.payload = input;
        self
    }
    /// Consumes the builder and constructs a [`UploadReadSetPartInput`](crate::operation::upload_read_set_part::UploadReadSetPartInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::upload_read_set_part::UploadReadSetPartInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::upload_read_set_part::UploadReadSetPartInput {
                sequence_store_id: self.sequence_store_id,
                upload_id: self.upload_id,
                part_source: self.part_source,
                part_number: self.part_number,
                payload: self.payload.unwrap_or_default(),
            },
        )
    }
}
