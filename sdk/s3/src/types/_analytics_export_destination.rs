// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Where to publish the analytics results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AnalyticsExportDestination {
    /// <p>A destination signifying output to an S3 bucket.</p>
    #[doc(hidden)]
    pub s3_bucket_destination: ::std::option::Option<crate::types::AnalyticsS3BucketDestination>,
}
impl AnalyticsExportDestination {
    /// <p>A destination signifying output to an S3 bucket.</p>
    pub fn s3_bucket_destination(
        &self,
    ) -> ::std::option::Option<&crate::types::AnalyticsS3BucketDestination> {
        self.s3_bucket_destination.as_ref()
    }
}
impl AnalyticsExportDestination {
    /// Creates a new builder-style object to manufacture [`AnalyticsExportDestination`](crate::types::AnalyticsExportDestination).
    pub fn builder() -> crate::types::builders::AnalyticsExportDestinationBuilder {
        crate::types::builders::AnalyticsExportDestinationBuilder::default()
    }
}

/// A builder for [`AnalyticsExportDestination`](crate::types::AnalyticsExportDestination).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AnalyticsExportDestinationBuilder {
    pub(crate) s3_bucket_destination:
        ::std::option::Option<crate::types::AnalyticsS3BucketDestination>,
}
impl AnalyticsExportDestinationBuilder {
    /// <p>A destination signifying output to an S3 bucket.</p>
    pub fn s3_bucket_destination(
        mut self,
        input: crate::types::AnalyticsS3BucketDestination,
    ) -> Self {
        self.s3_bucket_destination = ::std::option::Option::Some(input);
        self
    }
    /// <p>A destination signifying output to an S3 bucket.</p>
    pub fn set_s3_bucket_destination(
        mut self,
        input: ::std::option::Option<crate::types::AnalyticsS3BucketDestination>,
    ) -> Self {
        self.s3_bucket_destination = input;
        self
    }
    /// Consumes the builder and constructs a [`AnalyticsExportDestination`](crate::types::AnalyticsExportDestination).
    pub fn build(self) -> crate::types::AnalyticsExportDestination {
        crate::types::AnalyticsExportDestination {
            s3_bucket_destination: self.s3_bucket_destination,
        }
    }
}
