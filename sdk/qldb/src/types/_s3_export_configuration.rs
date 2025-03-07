// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Amazon Simple Storage Service (Amazon S3) bucket location in which a journal export job writes the journal contents.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3ExportConfiguration {
    /// <p>The Amazon S3 bucket name in which a journal export job writes the journal contents.</p>
    /// <p>The bucket name must comply with the Amazon S3 bucket naming conventions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html">Bucket Restrictions and Limitations</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    #[doc(hidden)]
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The prefix for the Amazon S3 bucket in which a journal export job writes the journal contents.</p>
    /// <p>The prefix must comply with Amazon S3 key naming rules and restrictions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    /// <p>The following are examples of valid <code>Prefix</code> values:</p>
    /// <ul>
    /// <li> <p> <code>JournalExports-ForMyLedger/Testing/</code> </p> </li>
    /// <li> <p> <code>JournalExports</code> </p> </li>
    /// <li> <p> <code>My:Tests/</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p>The encryption settings that are used by a journal export job to write data in an Amazon S3 bucket.</p>
    #[doc(hidden)]
    pub encryption_configuration: ::std::option::Option<crate::types::S3EncryptionConfiguration>,
}
impl S3ExportConfiguration {
    /// <p>The Amazon S3 bucket name in which a journal export job writes the journal contents.</p>
    /// <p>The bucket name must comply with the Amazon S3 bucket naming conventions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html">Bucket Restrictions and Limitations</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The prefix for the Amazon S3 bucket in which a journal export job writes the journal contents.</p>
    /// <p>The prefix must comply with Amazon S3 key naming rules and restrictions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    /// <p>The following are examples of valid <code>Prefix</code> values:</p>
    /// <ul>
    /// <li> <p> <code>JournalExports-ForMyLedger/Testing/</code> </p> </li>
    /// <li> <p> <code>JournalExports</code> </p> </li>
    /// <li> <p> <code>My:Tests/</code> </p> </li>
    /// </ul>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>The encryption settings that are used by a journal export job to write data in an Amazon S3 bucket.</p>
    pub fn encryption_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::S3EncryptionConfiguration> {
        self.encryption_configuration.as_ref()
    }
}
impl S3ExportConfiguration {
    /// Creates a new builder-style object to manufacture [`S3ExportConfiguration`](crate::types::S3ExportConfiguration).
    pub fn builder() -> crate::types::builders::S3ExportConfigurationBuilder {
        crate::types::builders::S3ExportConfigurationBuilder::default()
    }
}

/// A builder for [`S3ExportConfiguration`](crate::types::S3ExportConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct S3ExportConfigurationBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) encryption_configuration:
        ::std::option::Option<crate::types::S3EncryptionConfiguration>,
}
impl S3ExportConfigurationBuilder {
    /// <p>The Amazon S3 bucket name in which a journal export job writes the journal contents.</p>
    /// <p>The bucket name must comply with the Amazon S3 bucket naming conventions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html">Bucket Restrictions and Limitations</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon S3 bucket name in which a journal export job writes the journal contents.</p>
    /// <p>The bucket name must comply with the Amazon S3 bucket naming conventions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html">Bucket Restrictions and Limitations</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The prefix for the Amazon S3 bucket in which a journal export job writes the journal contents.</p>
    /// <p>The prefix must comply with Amazon S3 key naming rules and restrictions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    /// <p>The following are examples of valid <code>Prefix</code> values:</p>
    /// <ul>
    /// <li> <p> <code>JournalExports-ForMyLedger/Testing/</code> </p> </li>
    /// <li> <p> <code>JournalExports</code> </p> </li>
    /// <li> <p> <code>My:Tests/</code> </p> </li>
    /// </ul>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The prefix for the Amazon S3 bucket in which a journal export job writes the journal contents.</p>
    /// <p>The prefix must comply with Amazon S3 key naming rules and restrictions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    /// <p>The following are examples of valid <code>Prefix</code> values:</p>
    /// <ul>
    /// <li> <p> <code>JournalExports-ForMyLedger/Testing/</code> </p> </li>
    /// <li> <p> <code>JournalExports</code> </p> </li>
    /// <li> <p> <code>My:Tests/</code> </p> </li>
    /// </ul>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>The encryption settings that are used by a journal export job to write data in an Amazon S3 bucket.</p>
    pub fn encryption_configuration(
        mut self,
        input: crate::types::S3EncryptionConfiguration,
    ) -> Self {
        self.encryption_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption settings that are used by a journal export job to write data in an Amazon S3 bucket.</p>
    pub fn set_encryption_configuration(
        mut self,
        input: ::std::option::Option<crate::types::S3EncryptionConfiguration>,
    ) -> Self {
        self.encryption_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`S3ExportConfiguration`](crate::types::S3ExportConfiguration).
    pub fn build(self) -> crate::types::S3ExportConfiguration {
        crate::types::S3ExportConfiguration {
            bucket: self.bucket,
            prefix: self.prefix,
            encryption_configuration: self.encryption_configuration,
        }
    }
}
