// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CompleteMultipartUploadOutput {
    /// <p>The URI that identifies the newly created object.</p>
    #[doc(hidden)]
    pub location: ::std::option::Option<::std::string::String>,
    /// <p>The name of the bucket that contains the newly created object. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The object key of the newly created object.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>If the object expiration is configured, this will contain the expiration date (<code>expiry-date</code>) and rule ID (<code>rule-id</code>). The value of <code>rule-id</code> is URL-encoded.</p>
    #[doc(hidden)]
    pub expiration: ::std::option::Option<::std::string::String>,
    /// <p>Entity tag that identifies the newly created object's data. Objects with different object data will have different entity tags. The entity tag is an opaque string. The entity tag may or may not be an MD5 digest of the object data. If the entity tag is not an MD5 digest of the object data, it will contain one or more nonhexadecimal characters and/or will consist of less than 32 or more than 32 hexadecimal digits. For more information about how the entity tag is calculated, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub e_tag: ::std::option::Option<::std::string::String>,
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_crc32: ::std::option::Option<::std::string::String>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_crc32_c: ::std::option::Option<::std::string::String>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_sha1: ::std::option::Option<::std::string::String>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_sha256: ::std::option::Option<::std::string::String>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, <code>aws:kms</code>).</p>
    #[doc(hidden)]
    pub server_side_encryption: ::std::option::Option<crate::types::ServerSideEncryption>,
    /// <p>Version ID of the newly created object, in case the bucket has versioning turned on.</p>
    #[doc(hidden)]
    pub version_id: ::std::option::Option<::std::string::String>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric encryption customer managed key that was used for the object.</p>
    #[doc(hidden)]
    pub ssekms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    #[doc(hidden)]
    pub bucket_key_enabled: bool,
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    #[doc(hidden)]
    pub request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CompleteMultipartUploadOutput {
    /// <p>The URI that identifies the newly created object.</p>
    pub fn location(&self) -> ::std::option::Option<&str> {
        self.location.as_deref()
    }
    /// <p>The name of the bucket that contains the newly created object. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The object key of the newly created object.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>If the object expiration is configured, this will contain the expiration date (<code>expiry-date</code>) and rule ID (<code>rule-id</code>). The value of <code>rule-id</code> is URL-encoded.</p>
    pub fn expiration(&self) -> ::std::option::Option<&str> {
        self.expiration.as_deref()
    }
    /// <p>Entity tag that identifies the newly created object's data. Objects with different object data will have different entity tags. The entity tag is an opaque string. The entity tag may or may not be an MD5 digest of the object data. If the entity tag is not an MD5 digest of the object data, it will contain one or more nonhexadecimal characters and/or will consist of less than 32 or more than 32 hexadecimal digits. For more information about how the entity tag is calculated, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn e_tag(&self) -> ::std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32(&self) -> ::std::option::Option<&str> {
        self.checksum_crc32.as_deref()
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32_c(&self) -> ::std::option::Option<&str> {
        self.checksum_crc32_c.as_deref()
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha1(&self) -> ::std::option::Option<&str> {
        self.checksum_sha1.as_deref()
    }
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha256(&self) -> ::std::option::Option<&str> {
        self.checksum_sha256.as_deref()
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, <code>aws:kms</code>).</p>
    pub fn server_side_encryption(
        &self,
    ) -> ::std::option::Option<&crate::types::ServerSideEncryption> {
        self.server_side_encryption.as_ref()
    }
    /// <p>Version ID of the newly created object, in case the bucket has versioning turned on.</p>
    pub fn version_id(&self) -> ::std::option::Option<&str> {
        self.version_id.as_deref()
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric encryption customer managed key that was used for the object.</p>
    pub fn ssekms_key_id(&self) -> ::std::option::Option<&str> {
        self.ssekms_key_id.as_deref()
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn bucket_key_enabled(&self) -> bool {
        self.bucket_key_enabled
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(&self) -> ::std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
}
impl ::std::fmt::Debug for CompleteMultipartUploadOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CompleteMultipartUploadOutput");
        formatter.field("location", &self.location);
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("expiration", &self.expiration);
        formatter.field("e_tag", &self.e_tag);
        formatter.field("checksum_crc32", &self.checksum_crc32);
        formatter.field("checksum_crc32_c", &self.checksum_crc32_c);
        formatter.field("checksum_sha1", &self.checksum_sha1);
        formatter.field("checksum_sha256", &self.checksum_sha256);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("version_id", &self.version_id);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl crate::s3_request_id::RequestIdExt for CompleteMultipartUploadOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CompleteMultipartUploadOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CompleteMultipartUploadOutput {
    /// Creates a new builder-style object to manufacture [`CompleteMultipartUploadOutput`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput).
    pub fn builder(
    ) -> crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadOutputBuilder
    {
        crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadOutputBuilder::default()
    }
}

/// A builder for [`CompleteMultipartUploadOutput`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CompleteMultipartUploadOutputBuilder {
    pub(crate) location: ::std::option::Option<::std::string::String>,
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) expiration: ::std::option::Option<::std::string::String>,
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_crc32: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_crc32_c: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_sha1: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_sha256: ::std::option::Option<::std::string::String>,
    pub(crate) server_side_encryption: ::std::option::Option<crate::types::ServerSideEncryption>,
    pub(crate) version_id: ::std::option::Option<::std::string::String>,
    pub(crate) ssekms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) bucket_key_enabled: ::std::option::Option<bool>,
    pub(crate) request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CompleteMultipartUploadOutputBuilder {
    /// <p>The URI that identifies the newly created object.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URI that identifies the newly created object.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// <p>The name of the bucket that contains the newly created object. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket that contains the newly created object. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The object key of the newly created object.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The object key of the newly created object.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>If the object expiration is configured, this will contain the expiration date (<code>expiry-date</code>) and rule ID (<code>rule-id</code>). The value of <code>rule-id</code> is URL-encoded.</p>
    pub fn expiration(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expiration = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the object expiration is configured, this will contain the expiration date (<code>expiry-date</code>) and rule ID (<code>rule-id</code>). The value of <code>rule-id</code> is URL-encoded.</p>
    pub fn set_expiration(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expiration = input;
        self
    }
    /// <p>Entity tag that identifies the newly created object's data. Objects with different object data will have different entity tags. The entity tag is an opaque string. The entity tag may or may not be an MD5 digest of the object data. If the entity tag is not an MD5 digest of the object data, it will contain one or more nonhexadecimal characters and/or will consist of less than 32 or more than 32 hexadecimal digits. For more information about how the entity tag is calculated, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn e_tag(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.e_tag = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Entity tag that identifies the newly created object's data. Objects with different object data will have different entity tags. The entity tag is an opaque string. The entity tag may or may not be an MD5 digest of the object data. If the entity tag is not an MD5 digest of the object data, it will contain one or more nonhexadecimal characters and/or will consist of less than 32 or more than 32 hexadecimal digits. For more information about how the entity tag is calculated, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_e_tag(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.e_tag = input;
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.checksum_crc32 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.checksum_crc32 = input;
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32_c(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.checksum_crc32_c = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32_c(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.checksum_crc32_c = input;
        self
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha1(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.checksum_sha1 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha1(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.checksum_sha1 = input;
        self
    }
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha256(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.checksum_sha256 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha256(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.checksum_sha256 = input;
        self
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, <code>aws:kms</code>).</p>
    pub fn server_side_encryption(mut self, input: crate::types::ServerSideEncryption) -> Self {
        self.server_side_encryption = ::std::option::Option::Some(input);
        self
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, <code>aws:kms</code>).</p>
    pub fn set_server_side_encryption(
        mut self,
        input: ::std::option::Option<crate::types::ServerSideEncryption>,
    ) -> Self {
        self.server_side_encryption = input;
        self
    }
    /// <p>Version ID of the newly created object, in case the bucket has versioning turned on.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Version ID of the newly created object, in case the bucket has versioning turned on.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version_id = input;
        self
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric encryption customer managed key that was used for the object.</p>
    pub fn ssekms_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ssekms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric encryption customer managed key that was used for the object.</p>
    pub fn set_ssekms_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ssekms_key_id = input;
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn bucket_key_enabled(mut self, input: bool) -> Self {
        self.bucket_key_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn set_bucket_key_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.bucket_key_enabled = input;
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(mut self, input: crate::types::RequestCharged) -> Self {
        self.request_charged = ::std::option::Option::Some(input);
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn set_request_charged(
        mut self,
        input: ::std::option::Option<crate::types::RequestCharged>,
    ) -> Self {
        self.request_charged = input;
        self
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(
        &mut self,
        extended_request_id: Option<String>,
    ) -> &mut Self {
        self._extended_request_id = extended_request_id;
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
    /// Consumes the builder and constructs a [`CompleteMultipartUploadOutput`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput).
    pub fn build(
        self,
    ) -> crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput {
        crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput {
            location: self.location,
            bucket: self.bucket,
            key: self.key,
            expiration: self.expiration,
            e_tag: self.e_tag,
            checksum_crc32: self.checksum_crc32,
            checksum_crc32_c: self.checksum_crc32_c,
            checksum_sha1: self.checksum_sha1,
            checksum_sha256: self.checksum_sha256,
            server_side_encryption: self.server_side_encryption,
            version_id: self.version_id,
            ssekms_key_id: self.ssekms_key_id,
            bucket_key_enabled: self.bucket_key_enabled.unwrap_or_default(),
            request_charged: self.request_charged,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for CompleteMultipartUploadOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CompleteMultipartUploadOutputBuilder");
        formatter.field("location", &self.location);
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("expiration", &self.expiration);
        formatter.field("e_tag", &self.e_tag);
        formatter.field("checksum_crc32", &self.checksum_crc32);
        formatter.field("checksum_crc32_c", &self.checksum_crc32_c);
        formatter.field("checksum_sha1", &self.checksum_sha1);
        formatter.field("checksum_sha256", &self.checksum_sha256);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("version_id", &self.version_id);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
