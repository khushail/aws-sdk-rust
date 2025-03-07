// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateStreamingImage`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::set_description): <p>The description.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::set_name): <p>The name for the streaming image.</p>
    ///   - [`streaming_image_id(impl ::std::convert::Into<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::streaming_image_id) / [`set_streaming_image_id(Option<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::set_streaming_image_id): <p>The streaming image ID.</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::set_studio_id): <p>The studio ID. </p>
    /// - On success, responds with [`UpdateStreamingImageOutput`](crate::operation::update_streaming_image::UpdateStreamingImageOutput) with field(s):
    ///   - [`streaming_image(Option<StreamingImage>)`](crate::operation::update_streaming_image::UpdateStreamingImageOutput::streaming_image): <p>Represents a streaming image resource.</p>  <p>Streaming images are used by studio users to select which operating system and software they want to use in a Nimble Studio streaming session.</p>  <p>Amazon provides a number of streaming images that include popular 3rd-party software.</p>  <p>You can create your own streaming images using an Amazon EC2 machine image that you create for this purpose. You can also include software that your users require.</p>
    /// - On failure, responds with [`SdkError<UpdateStreamingImageError>`](crate::operation::update_streaming_image::UpdateStreamingImageError)
    pub fn update_streaming_image(
        &self,
    ) -> crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder {
        crate::operation::update_streaming_image::builders::UpdateStreamingImageFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
