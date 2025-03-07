// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartFaceDetection`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`video(Video)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::video) / [`set_video(Option<Video>)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::set_video): <p>The video in which you want to detect faces. The video must be stored in an Amazon S3 bucket.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::set_client_request_token): <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartFaceDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    ///   - [`notification_channel(NotificationChannel)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::notification_channel) / [`set_notification_channel(Option<NotificationChannel>)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::set_notification_channel): <p>The ARN of the Amazon SNS topic to which you want Amazon Rekognition Video to publish the completion status of the face detection operation. The Amazon SNS topic must have a topic name that begins with <i>AmazonRekognition</i> if you are using the AmazonRekognitionServiceRole permissions policy.</p>
    ///   - [`face_attributes(FaceAttributes)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::face_attributes) / [`set_face_attributes(Option<FaceAttributes>)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::set_face_attributes): <p>The face attributes you want returned.</p>  <p> <code>DEFAULT</code> - The following subset of facial attributes are returned: BoundingBox, Confidence, Pose, Quality and Landmarks. </p>  <p> <code>ALL</code> - All facial attributes are returned.</p>
    ///   - [`job_tag(impl ::std::convert::Into<String>)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::job_tag) / [`set_job_tag(Option<String>)`](crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::set_job_tag): <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    /// - On success, responds with [`StartFaceDetectionOutput`](crate::operation::start_face_detection::StartFaceDetectionOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::operation::start_face_detection::StartFaceDetectionOutput::job_id): <p>The identifier for the face detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetFaceDetection</code>.</p>
    /// - On failure, responds with [`SdkError<StartFaceDetectionError>`](crate::operation::start_face_detection::StartFaceDetectionError)
    pub fn start_face_detection(
        &self,
    ) -> crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder {
        crate::operation::start_face_detection::builders::StartFaceDetectionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
