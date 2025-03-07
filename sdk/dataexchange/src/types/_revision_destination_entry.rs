// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The destination where the assets in the revision will be exported.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RevisionDestinationEntry {
    /// <p>The Amazon S3 bucket that is the destination for the assets in the revision.</p>
    #[doc(hidden)]
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>A string representing the pattern for generated names of the individual assets in the revision. For more information about key patterns, see <a href="https://docs.aws.amazon.com/data-exchange/latest/userguide/jobs.html#revision-export-keypatterns">Key patterns when exporting revisions</a>.</p>
    #[doc(hidden)]
    pub key_pattern: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the revision.</p>
    #[doc(hidden)]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
impl RevisionDestinationEntry {
    /// <p>The Amazon S3 bucket that is the destination for the assets in the revision.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>A string representing the pattern for generated names of the individual assets in the revision. For more information about key patterns, see <a href="https://docs.aws.amazon.com/data-exchange/latest/userguide/jobs.html#revision-export-keypatterns">Key patterns when exporting revisions</a>.</p>
    pub fn key_pattern(&self) -> ::std::option::Option<&str> {
        self.key_pattern.as_deref()
    }
    /// <p>The unique identifier for the revision.</p>
    pub fn revision_id(&self) -> ::std::option::Option<&str> {
        self.revision_id.as_deref()
    }
}
impl RevisionDestinationEntry {
    /// Creates a new builder-style object to manufacture [`RevisionDestinationEntry`](crate::types::RevisionDestinationEntry).
    pub fn builder() -> crate::types::builders::RevisionDestinationEntryBuilder {
        crate::types::builders::RevisionDestinationEntryBuilder::default()
    }
}

/// A builder for [`RevisionDestinationEntry`](crate::types::RevisionDestinationEntry).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RevisionDestinationEntryBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) key_pattern: ::std::option::Option<::std::string::String>,
    pub(crate) revision_id: ::std::option::Option<::std::string::String>,
}
impl RevisionDestinationEntryBuilder {
    /// <p>The Amazon S3 bucket that is the destination for the assets in the revision.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon S3 bucket that is the destination for the assets in the revision.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>A string representing the pattern for generated names of the individual assets in the revision. For more information about key patterns, see <a href="https://docs.aws.amazon.com/data-exchange/latest/userguide/jobs.html#revision-export-keypatterns">Key patterns when exporting revisions</a>.</p>
    pub fn key_pattern(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_pattern = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string representing the pattern for generated names of the individual assets in the revision. For more information about key patterns, see <a href="https://docs.aws.amazon.com/data-exchange/latest/userguide/jobs.html#revision-export-keypatterns">Key patterns when exporting revisions</a>.</p>
    pub fn set_key_pattern(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_pattern = input;
        self
    }
    /// <p>The unique identifier for the revision.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the revision.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.revision_id = input;
        self
    }
    /// Consumes the builder and constructs a [`RevisionDestinationEntry`](crate::types::RevisionDestinationEntry).
    pub fn build(self) -> crate::types::RevisionDestinationEntry {
        crate::types::RevisionDestinationEntry {
            bucket: self.bucket,
            key_pattern: self.key_pattern,
            revision_id: self.revision_id,
        }
    }
}
