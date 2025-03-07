// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPersonTrackingOutput {
    /// <p>The current status of the person tracking job.</p>
    #[doc(hidden)]
    pub job_status: ::std::option::Option<crate::types::VideoJobStatus>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[doc(hidden)]
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[doc(hidden)]
    pub video_metadata: ::std::option::Option<crate::types::VideoMetadata>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>An array of the persons detected in the video and the time(s) their path was tracked throughout the video. An array element will exist for each time a person's path is tracked. </p>
    #[doc(hidden)]
    pub persons: ::std::option::Option<::std::vec::Vec<crate::types::PersonDetection>>,
    /// <p>Job identifier for the person tracking operation for which you want to obtain results. The job identifer is returned by an initial call to StartPersonTracking.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
    /// <p>Video file stored in an Amazon S3 bucket. Amazon Rekognition video start operations such as <code>StartLabelDetection</code> use <code>Video</code> to specify a video for analysis. The supported file formats are .mp4, .mov and .avi.</p>
    #[doc(hidden)]
    pub video: ::std::option::Option<crate::types::Video>,
    /// <p>A job identifier specified in the call to StartCelebrityRecognition and returned in the job completion notification sent to your Amazon Simple Notification Service topic.</p>
    #[doc(hidden)]
    pub job_tag: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetPersonTrackingOutput {
    /// <p>The current status of the person tracking job.</p>
    pub fn job_status(&self) -> ::std::option::Option<&crate::types::VideoJobStatus> {
        self.job_status.as_ref()
    }
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    pub fn video_metadata(&self) -> ::std::option::Option<&crate::types::VideoMetadata> {
        self.video_metadata.as_ref()
    }
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>An array of the persons detected in the video and the time(s) their path was tracked throughout the video. An array element will exist for each time a person's path is tracked. </p>
    pub fn persons(&self) -> ::std::option::Option<&[crate::types::PersonDetection]> {
        self.persons.as_deref()
    }
    /// <p>Job identifier for the person tracking operation for which you want to obtain results. The job identifer is returned by an initial call to StartPersonTracking.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p>Video file stored in an Amazon S3 bucket. Amazon Rekognition video start operations such as <code>StartLabelDetection</code> use <code>Video</code> to specify a video for analysis. The supported file formats are .mp4, .mov and .avi.</p>
    pub fn video(&self) -> ::std::option::Option<&crate::types::Video> {
        self.video.as_ref()
    }
    /// <p>A job identifier specified in the call to StartCelebrityRecognition and returned in the job completion notification sent to your Amazon Simple Notification Service topic.</p>
    pub fn job_tag(&self) -> ::std::option::Option<&str> {
        self.job_tag.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetPersonTrackingOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetPersonTrackingOutput {
    /// Creates a new builder-style object to manufacture [`GetPersonTrackingOutput`](crate::operation::get_person_tracking::GetPersonTrackingOutput).
    pub fn builder(
    ) -> crate::operation::get_person_tracking::builders::GetPersonTrackingOutputBuilder {
        crate::operation::get_person_tracking::builders::GetPersonTrackingOutputBuilder::default()
    }
}

/// A builder for [`GetPersonTrackingOutput`](crate::operation::get_person_tracking::GetPersonTrackingOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPersonTrackingOutputBuilder {
    pub(crate) job_status: ::std::option::Option<crate::types::VideoJobStatus>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) video_metadata: ::std::option::Option<crate::types::VideoMetadata>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) persons: ::std::option::Option<::std::vec::Vec<crate::types::PersonDetection>>,
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) video: ::std::option::Option<crate::types::Video>,
    pub(crate) job_tag: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetPersonTrackingOutputBuilder {
    /// <p>The current status of the person tracking job.</p>
    pub fn job_status(mut self, input: crate::types::VideoJobStatus) -> Self {
        self.job_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the person tracking job.</p>
    pub fn set_job_status(
        mut self,
        input: ::std::option::Option<crate::types::VideoJobStatus>,
    ) -> Self {
        self.job_status = input;
        self
    }
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_message = input;
        self
    }
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    pub fn video_metadata(mut self, input: crate::types::VideoMetadata) -> Self {
        self.video_metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    pub fn set_video_metadata(
        mut self,
        input: ::std::option::Option<crate::types::VideoMetadata>,
    ) -> Self {
        self.video_metadata = input;
        self
    }
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `persons`.
    ///
    /// To override the contents of this collection use [`set_persons`](Self::set_persons).
    ///
    /// <p>An array of the persons detected in the video and the time(s) their path was tracked throughout the video. An array element will exist for each time a person's path is tracked. </p>
    pub fn persons(mut self, input: crate::types::PersonDetection) -> Self {
        let mut v = self.persons.unwrap_or_default();
        v.push(input);
        self.persons = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of the persons detected in the video and the time(s) their path was tracked throughout the video. An array element will exist for each time a person's path is tracked. </p>
    pub fn set_persons(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PersonDetection>>,
    ) -> Self {
        self.persons = input;
        self
    }
    /// <p>Job identifier for the person tracking operation for which you want to obtain results. The job identifer is returned by an initial call to StartPersonTracking.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Job identifier for the person tracking operation for which you want to obtain results. The job identifer is returned by an initial call to StartPersonTracking.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p>Video file stored in an Amazon S3 bucket. Amazon Rekognition video start operations such as <code>StartLabelDetection</code> use <code>Video</code> to specify a video for analysis. The supported file formats are .mp4, .mov and .avi.</p>
    pub fn video(mut self, input: crate::types::Video) -> Self {
        self.video = ::std::option::Option::Some(input);
        self
    }
    /// <p>Video file stored in an Amazon S3 bucket. Amazon Rekognition video start operations such as <code>StartLabelDetection</code> use <code>Video</code> to specify a video for analysis. The supported file formats are .mp4, .mov and .avi.</p>
    pub fn set_video(mut self, input: ::std::option::Option<crate::types::Video>) -> Self {
        self.video = input;
        self
    }
    /// <p>A job identifier specified in the call to StartCelebrityRecognition and returned in the job completion notification sent to your Amazon Simple Notification Service topic.</p>
    pub fn job_tag(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_tag = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A job identifier specified in the call to StartCelebrityRecognition and returned in the job completion notification sent to your Amazon Simple Notification Service topic.</p>
    pub fn set_job_tag(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_tag = input;
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
    /// Consumes the builder and constructs a [`GetPersonTrackingOutput`](crate::operation::get_person_tracking::GetPersonTrackingOutput).
    pub fn build(self) -> crate::operation::get_person_tracking::GetPersonTrackingOutput {
        crate::operation::get_person_tracking::GetPersonTrackingOutput {
            job_status: self.job_status,
            status_message: self.status_message,
            video_metadata: self.video_metadata,
            next_token: self.next_token,
            persons: self.persons,
            job_id: self.job_id,
            video: self.video,
            job_tag: self.job_tag,
            _request_id: self._request_id,
        }
    }
}
