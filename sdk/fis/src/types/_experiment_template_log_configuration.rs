// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the configuration for experiment logging.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExperimentTemplateLogConfiguration {
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    #[doc(hidden)]
    pub cloud_watch_logs_configuration:
        ::std::option::Option<crate::types::ExperimentTemplateCloudWatchLogsLogConfiguration>,
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    #[doc(hidden)]
    pub s3_configuration: ::std::option::Option<crate::types::ExperimentTemplateS3LogConfiguration>,
    /// <p>The schema version.</p>
    #[doc(hidden)]
    pub log_schema_version: ::std::option::Option<i32>,
}
impl ExperimentTemplateLogConfiguration {
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    pub fn cloud_watch_logs_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ExperimentTemplateCloudWatchLogsLogConfiguration>
    {
        self.cloud_watch_logs_configuration.as_ref()
    }
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    pub fn s3_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ExperimentTemplateS3LogConfiguration> {
        self.s3_configuration.as_ref()
    }
    /// <p>The schema version.</p>
    pub fn log_schema_version(&self) -> ::std::option::Option<i32> {
        self.log_schema_version
    }
}
impl ExperimentTemplateLogConfiguration {
    /// Creates a new builder-style object to manufacture [`ExperimentTemplateLogConfiguration`](crate::types::ExperimentTemplateLogConfiguration).
    pub fn builder() -> crate::types::builders::ExperimentTemplateLogConfigurationBuilder {
        crate::types::builders::ExperimentTemplateLogConfigurationBuilder::default()
    }
}

/// A builder for [`ExperimentTemplateLogConfiguration`](crate::types::ExperimentTemplateLogConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExperimentTemplateLogConfigurationBuilder {
    pub(crate) cloud_watch_logs_configuration:
        ::std::option::Option<crate::types::ExperimentTemplateCloudWatchLogsLogConfiguration>,
    pub(crate) s3_configuration:
        ::std::option::Option<crate::types::ExperimentTemplateS3LogConfiguration>,
    pub(crate) log_schema_version: ::std::option::Option<i32>,
}
impl ExperimentTemplateLogConfigurationBuilder {
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    pub fn cloud_watch_logs_configuration(
        mut self,
        input: crate::types::ExperimentTemplateCloudWatchLogsLogConfiguration,
    ) -> Self {
        self.cloud_watch_logs_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    pub fn set_cloud_watch_logs_configuration(
        mut self,
        input: ::std::option::Option<
            crate::types::ExperimentTemplateCloudWatchLogsLogConfiguration,
        >,
    ) -> Self {
        self.cloud_watch_logs_configuration = input;
        self
    }
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    pub fn s3_configuration(
        mut self,
        input: crate::types::ExperimentTemplateS3LogConfiguration,
    ) -> Self {
        self.s3_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    pub fn set_s3_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ExperimentTemplateS3LogConfiguration>,
    ) -> Self {
        self.s3_configuration = input;
        self
    }
    /// <p>The schema version.</p>
    pub fn log_schema_version(mut self, input: i32) -> Self {
        self.log_schema_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The schema version.</p>
    pub fn set_log_schema_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.log_schema_version = input;
        self
    }
    /// Consumes the builder and constructs a [`ExperimentTemplateLogConfiguration`](crate::types::ExperimentTemplateLogConfiguration).
    pub fn build(self) -> crate::types::ExperimentTemplateLogConfiguration {
        crate::types::ExperimentTemplateLogConfiguration {
            cloud_watch_logs_configuration: self.cloud_watch_logs_configuration,
            s3_configuration: self.s3_configuration,
            log_schema_version: self.log_schema_version,
        }
    }
}
