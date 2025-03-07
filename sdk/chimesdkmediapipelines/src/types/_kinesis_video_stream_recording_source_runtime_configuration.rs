// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that contains the runtime settings for recording a Kinesis video stream.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KinesisVideoStreamRecordingSourceRuntimeConfiguration {
    /// <p>The stream or streams to be recorded.</p>
    #[doc(hidden)]
    pub streams: ::std::option::Option<::std::vec::Vec<crate::types::RecordingStreamConfiguration>>,
    /// <p>Describes the timestamp range and timestamp origin of a range of fragments in the Kinesis video stream.</p>
    #[doc(hidden)]
    pub fragment_selector: ::std::option::Option<crate::types::FragmentSelector>,
}
impl KinesisVideoStreamRecordingSourceRuntimeConfiguration {
    /// <p>The stream or streams to be recorded.</p>
    pub fn streams(&self) -> ::std::option::Option<&[crate::types::RecordingStreamConfiguration]> {
        self.streams.as_deref()
    }
    /// <p>Describes the timestamp range and timestamp origin of a range of fragments in the Kinesis video stream.</p>
    pub fn fragment_selector(&self) -> ::std::option::Option<&crate::types::FragmentSelector> {
        self.fragment_selector.as_ref()
    }
}
impl KinesisVideoStreamRecordingSourceRuntimeConfiguration {
    /// Creates a new builder-style object to manufacture [`KinesisVideoStreamRecordingSourceRuntimeConfiguration`](crate::types::KinesisVideoStreamRecordingSourceRuntimeConfiguration).
    pub fn builder(
    ) -> crate::types::builders::KinesisVideoStreamRecordingSourceRuntimeConfigurationBuilder {
        crate::types::builders::KinesisVideoStreamRecordingSourceRuntimeConfigurationBuilder::default()
    }
}

/// A builder for [`KinesisVideoStreamRecordingSourceRuntimeConfiguration`](crate::types::KinesisVideoStreamRecordingSourceRuntimeConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KinesisVideoStreamRecordingSourceRuntimeConfigurationBuilder {
    pub(crate) streams:
        ::std::option::Option<::std::vec::Vec<crate::types::RecordingStreamConfiguration>>,
    pub(crate) fragment_selector: ::std::option::Option<crate::types::FragmentSelector>,
}
impl KinesisVideoStreamRecordingSourceRuntimeConfigurationBuilder {
    /// Appends an item to `streams`.
    ///
    /// To override the contents of this collection use [`set_streams`](Self::set_streams).
    ///
    /// <p>The stream or streams to be recorded.</p>
    pub fn streams(mut self, input: crate::types::RecordingStreamConfiguration) -> Self {
        let mut v = self.streams.unwrap_or_default();
        v.push(input);
        self.streams = ::std::option::Option::Some(v);
        self
    }
    /// <p>The stream or streams to be recorded.</p>
    pub fn set_streams(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RecordingStreamConfiguration>>,
    ) -> Self {
        self.streams = input;
        self
    }
    /// <p>Describes the timestamp range and timestamp origin of a range of fragments in the Kinesis video stream.</p>
    pub fn fragment_selector(mut self, input: crate::types::FragmentSelector) -> Self {
        self.fragment_selector = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the timestamp range and timestamp origin of a range of fragments in the Kinesis video stream.</p>
    pub fn set_fragment_selector(
        mut self,
        input: ::std::option::Option<crate::types::FragmentSelector>,
    ) -> Self {
        self.fragment_selector = input;
        self
    }
    /// Consumes the builder and constructs a [`KinesisVideoStreamRecordingSourceRuntimeConfiguration`](crate::types::KinesisVideoStreamRecordingSourceRuntimeConfiguration).
    pub fn build(self) -> crate::types::KinesisVideoStreamRecordingSourceRuntimeConfiguration {
        crate::types::KinesisVideoStreamRecordingSourceRuntimeConfiguration {
            streams: self.streams,
            fragment_selector: self.fragment_selector,
        }
    }
}
