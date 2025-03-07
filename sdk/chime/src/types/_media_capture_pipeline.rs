// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A media capture pipeline object consisting of an ID, source type, source ARN, a sink type, a sink ARN, and a configuration object.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct MediaCapturePipeline {
    /// <p>The ID of a media capture pipeline.</p>
    #[doc(hidden)]
    pub media_pipeline_id: ::std::option::Option<::std::string::String>,
    /// <p>Source type from which media artifacts are saved. You must use <code>ChimeMeeting</code>.</p>
    #[doc(hidden)]
    pub source_type: ::std::option::Option<crate::types::MediaPipelineSourceType>,
    /// <p>ARN of the source from which the media artifacts will be saved.</p>
    #[doc(hidden)]
    pub source_arn: ::std::option::Option<::std::string::String>,
    /// <p>The status of the media capture pipeline.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::MediaPipelineStatus>,
    /// <p>Destination type to which the media artifacts are saved. You must use an S3 Bucket.</p>
    #[doc(hidden)]
    pub sink_type: ::std::option::Option<crate::types::MediaPipelineSinkType>,
    /// <p>ARN of the destination to which the media artifacts are saved.</p>
    #[doc(hidden)]
    pub sink_arn: ::std::option::Option<::std::string::String>,
    /// <p>The time at which the capture pipeline was created, in ISO 8601 format.</p>
    #[doc(hidden)]
    pub created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time at which the capture pipeline was updated, in ISO 8601 format.</p>
    #[doc(hidden)]
    pub updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The configuration for a specified media capture pipeline. <code>SourceType</code> must be <code>ChimeSdkMeeting</code>.</p>
    #[doc(hidden)]
    pub chime_sdk_meeting_configuration:
        ::std::option::Option<crate::types::ChimeSdkMeetingConfiguration>,
}
impl MediaCapturePipeline {
    /// <p>The ID of a media capture pipeline.</p>
    pub fn media_pipeline_id(&self) -> ::std::option::Option<&str> {
        self.media_pipeline_id.as_deref()
    }
    /// <p>Source type from which media artifacts are saved. You must use <code>ChimeMeeting</code>.</p>
    pub fn source_type(&self) -> ::std::option::Option<&crate::types::MediaPipelineSourceType> {
        self.source_type.as_ref()
    }
    /// <p>ARN of the source from which the media artifacts will be saved.</p>
    pub fn source_arn(&self) -> ::std::option::Option<&str> {
        self.source_arn.as_deref()
    }
    /// <p>The status of the media capture pipeline.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::MediaPipelineStatus> {
        self.status.as_ref()
    }
    /// <p>Destination type to which the media artifacts are saved. You must use an S3 Bucket.</p>
    pub fn sink_type(&self) -> ::std::option::Option<&crate::types::MediaPipelineSinkType> {
        self.sink_type.as_ref()
    }
    /// <p>ARN of the destination to which the media artifacts are saved.</p>
    pub fn sink_arn(&self) -> ::std::option::Option<&str> {
        self.sink_arn.as_deref()
    }
    /// <p>The time at which the capture pipeline was created, in ISO 8601 format.</p>
    pub fn created_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_timestamp.as_ref()
    }
    /// <p>The time at which the capture pipeline was updated, in ISO 8601 format.</p>
    pub fn updated_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.updated_timestamp.as_ref()
    }
    /// <p>The configuration for a specified media capture pipeline. <code>SourceType</code> must be <code>ChimeSdkMeeting</code>.</p>
    pub fn chime_sdk_meeting_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ChimeSdkMeetingConfiguration> {
        self.chime_sdk_meeting_configuration.as_ref()
    }
}
impl ::std::fmt::Debug for MediaCapturePipeline {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("MediaCapturePipeline");
        formatter.field("media_pipeline_id", &self.media_pipeline_id);
        formatter.field("source_type", &self.source_type);
        formatter.field("source_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("status", &self.status);
        formatter.field("sink_type", &self.sink_type);
        formatter.field("sink_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("updated_timestamp", &self.updated_timestamp);
        formatter.field(
            "chime_sdk_meeting_configuration",
            &self.chime_sdk_meeting_configuration,
        );
        formatter.finish()
    }
}
impl MediaCapturePipeline {
    /// Creates a new builder-style object to manufacture [`MediaCapturePipeline`](crate::types::MediaCapturePipeline).
    pub fn builder() -> crate::types::builders::MediaCapturePipelineBuilder {
        crate::types::builders::MediaCapturePipelineBuilder::default()
    }
}

/// A builder for [`MediaCapturePipeline`](crate::types::MediaCapturePipeline).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct MediaCapturePipelineBuilder {
    pub(crate) media_pipeline_id: ::std::option::Option<::std::string::String>,
    pub(crate) source_type: ::std::option::Option<crate::types::MediaPipelineSourceType>,
    pub(crate) source_arn: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::MediaPipelineStatus>,
    pub(crate) sink_type: ::std::option::Option<crate::types::MediaPipelineSinkType>,
    pub(crate) sink_arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) chime_sdk_meeting_configuration:
        ::std::option::Option<crate::types::ChimeSdkMeetingConfiguration>,
}
impl MediaCapturePipelineBuilder {
    /// <p>The ID of a media capture pipeline.</p>
    pub fn media_pipeline_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.media_pipeline_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of a media capture pipeline.</p>
    pub fn set_media_pipeline_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.media_pipeline_id = input;
        self
    }
    /// <p>Source type from which media artifacts are saved. You must use <code>ChimeMeeting</code>.</p>
    pub fn source_type(mut self, input: crate::types::MediaPipelineSourceType) -> Self {
        self.source_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Source type from which media artifacts are saved. You must use <code>ChimeMeeting</code>.</p>
    pub fn set_source_type(
        mut self,
        input: ::std::option::Option<crate::types::MediaPipelineSourceType>,
    ) -> Self {
        self.source_type = input;
        self
    }
    /// <p>ARN of the source from which the media artifacts will be saved.</p>
    pub fn source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN of the source from which the media artifacts will be saved.</p>
    pub fn set_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_arn = input;
        self
    }
    /// <p>The status of the media capture pipeline.</p>
    pub fn status(mut self, input: crate::types::MediaPipelineStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the media capture pipeline.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::MediaPipelineStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>Destination type to which the media artifacts are saved. You must use an S3 Bucket.</p>
    pub fn sink_type(mut self, input: crate::types::MediaPipelineSinkType) -> Self {
        self.sink_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Destination type to which the media artifacts are saved. You must use an S3 Bucket.</p>
    pub fn set_sink_type(
        mut self,
        input: ::std::option::Option<crate::types::MediaPipelineSinkType>,
    ) -> Self {
        self.sink_type = input;
        self
    }
    /// <p>ARN of the destination to which the media artifacts are saved.</p>
    pub fn sink_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sink_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN of the destination to which the media artifacts are saved.</p>
    pub fn set_sink_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sink_arn = input;
        self
    }
    /// <p>The time at which the capture pipeline was created, in ISO 8601 format.</p>
    pub fn created_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the capture pipeline was created, in ISO 8601 format.</p>
    pub fn set_created_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_timestamp = input;
        self
    }
    /// <p>The time at which the capture pipeline was updated, in ISO 8601 format.</p>
    pub fn updated_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the capture pipeline was updated, in ISO 8601 format.</p>
    pub fn set_updated_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.updated_timestamp = input;
        self
    }
    /// <p>The configuration for a specified media capture pipeline. <code>SourceType</code> must be <code>ChimeSdkMeeting</code>.</p>
    pub fn chime_sdk_meeting_configuration(
        mut self,
        input: crate::types::ChimeSdkMeetingConfiguration,
    ) -> Self {
        self.chime_sdk_meeting_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for a specified media capture pipeline. <code>SourceType</code> must be <code>ChimeSdkMeeting</code>.</p>
    pub fn set_chime_sdk_meeting_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ChimeSdkMeetingConfiguration>,
    ) -> Self {
        self.chime_sdk_meeting_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`MediaCapturePipeline`](crate::types::MediaCapturePipeline).
    pub fn build(self) -> crate::types::MediaCapturePipeline {
        crate::types::MediaCapturePipeline {
            media_pipeline_id: self.media_pipeline_id,
            source_type: self.source_type,
            source_arn: self.source_arn,
            status: self.status,
            sink_type: self.sink_type,
            sink_arn: self.sink_arn,
            created_timestamp: self.created_timestamp,
            updated_timestamp: self.updated_timestamp,
            chime_sdk_meeting_configuration: self.chime_sdk_meeting_configuration,
        }
    }
}
impl ::std::fmt::Debug for MediaCapturePipelineBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("MediaCapturePipelineBuilder");
        formatter.field("media_pipeline_id", &self.media_pipeline_id);
        formatter.field("source_type", &self.source_type);
        formatter.field("source_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("status", &self.status);
        formatter.field("sink_type", &self.sink_type);
        formatter.field("sink_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("updated_timestamp", &self.updated_timestamp);
        formatter.field(
            "chime_sdk_meeting_configuration",
            &self.chime_sdk_meeting_configuration,
        );
        formatter.finish()
    }
}
