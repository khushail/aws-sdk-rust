// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Video settings for this stream.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VideoDescription {
    /// Video codec settings.
    #[doc(hidden)]
    pub codec_settings: ::std::option::Option<crate::types::VideoCodecSettings>,
    /// Output video height, in pixels. Must be an even number. For most codecs, you can leave this field and width blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    #[doc(hidden)]
    pub height: ::std::option::Option<i32>,
    /// The name of this VideoDescription. Outputs will use this name to uniquely identify this Description. Description names should be unique within this Live Event.
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// Indicates how MediaLive will respond to the AFD values that might be in the input video. If you do not know what AFD signaling is, or if your downstream system has not given you guidance, choose PASSTHROUGH. RESPOND: MediaLive clips the input video using a formula that uses the AFD values (configured in afdSignaling ), the input display aspect ratio, and the output display aspect ratio. MediaLive also includes the AFD values in the output, unless the codec for this encode is FRAME_CAPTURE. PASSTHROUGH: MediaLive ignores the AFD values and does not clip the video. But MediaLive does include the values in the output. NONE: MediaLive does not clip the input video and does not include the AFD values in the output
    #[doc(hidden)]
    pub respond_to_afd: ::std::option::Option<crate::types::VideoDescriptionRespondToAfd>,
    /// STRETCH_TO_OUTPUT configures the output position to stretch the video to the specified output resolution (height and width). This option will override any position value. DEFAULT may insert black boxes (pillar boxes or letter boxes) around the video to provide the specified output resolution.
    #[doc(hidden)]
    pub scaling_behavior: ::std::option::Option<crate::types::VideoDescriptionScalingBehavior>,
    /// Changes the strength of the anti-alias filter used for scaling. 0 is the softest setting, 100 is the sharpest. A setting of 50 is recommended for most content.
    #[doc(hidden)]
    pub sharpness: ::std::option::Option<i32>,
    /// Output video width, in pixels. Must be an even number. For most codecs, you can leave this field and height blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    #[doc(hidden)]
    pub width: ::std::option::Option<i32>,
}
impl VideoDescription {
    /// Video codec settings.
    pub fn codec_settings(&self) -> ::std::option::Option<&crate::types::VideoCodecSettings> {
        self.codec_settings.as_ref()
    }
    /// Output video height, in pixels. Must be an even number. For most codecs, you can leave this field and width blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    pub fn height(&self) -> ::std::option::Option<i32> {
        self.height
    }
    /// The name of this VideoDescription. Outputs will use this name to uniquely identify this Description. Description names should be unique within this Live Event.
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// Indicates how MediaLive will respond to the AFD values that might be in the input video. If you do not know what AFD signaling is, or if your downstream system has not given you guidance, choose PASSTHROUGH. RESPOND: MediaLive clips the input video using a formula that uses the AFD values (configured in afdSignaling ), the input display aspect ratio, and the output display aspect ratio. MediaLive also includes the AFD values in the output, unless the codec for this encode is FRAME_CAPTURE. PASSTHROUGH: MediaLive ignores the AFD values and does not clip the video. But MediaLive does include the values in the output. NONE: MediaLive does not clip the input video and does not include the AFD values in the output
    pub fn respond_to_afd(
        &self,
    ) -> ::std::option::Option<&crate::types::VideoDescriptionRespondToAfd> {
        self.respond_to_afd.as_ref()
    }
    /// STRETCH_TO_OUTPUT configures the output position to stretch the video to the specified output resolution (height and width). This option will override any position value. DEFAULT may insert black boxes (pillar boxes or letter boxes) around the video to provide the specified output resolution.
    pub fn scaling_behavior(
        &self,
    ) -> ::std::option::Option<&crate::types::VideoDescriptionScalingBehavior> {
        self.scaling_behavior.as_ref()
    }
    /// Changes the strength of the anti-alias filter used for scaling. 0 is the softest setting, 100 is the sharpest. A setting of 50 is recommended for most content.
    pub fn sharpness(&self) -> ::std::option::Option<i32> {
        self.sharpness
    }
    /// Output video width, in pixels. Must be an even number. For most codecs, you can leave this field and height blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    pub fn width(&self) -> ::std::option::Option<i32> {
        self.width
    }
}
impl VideoDescription {
    /// Creates a new builder-style object to manufacture [`VideoDescription`](crate::types::VideoDescription).
    pub fn builder() -> crate::types::builders::VideoDescriptionBuilder {
        crate::types::builders::VideoDescriptionBuilder::default()
    }
}

/// A builder for [`VideoDescription`](crate::types::VideoDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VideoDescriptionBuilder {
    pub(crate) codec_settings: ::std::option::Option<crate::types::VideoCodecSettings>,
    pub(crate) height: ::std::option::Option<i32>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) respond_to_afd: ::std::option::Option<crate::types::VideoDescriptionRespondToAfd>,
    pub(crate) scaling_behavior:
        ::std::option::Option<crate::types::VideoDescriptionScalingBehavior>,
    pub(crate) sharpness: ::std::option::Option<i32>,
    pub(crate) width: ::std::option::Option<i32>,
}
impl VideoDescriptionBuilder {
    /// Video codec settings.
    pub fn codec_settings(mut self, input: crate::types::VideoCodecSettings) -> Self {
        self.codec_settings = ::std::option::Option::Some(input);
        self
    }
    /// Video codec settings.
    pub fn set_codec_settings(
        mut self,
        input: ::std::option::Option<crate::types::VideoCodecSettings>,
    ) -> Self {
        self.codec_settings = input;
        self
    }
    /// Output video height, in pixels. Must be an even number. For most codecs, you can leave this field and width blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    pub fn height(mut self, input: i32) -> Self {
        self.height = ::std::option::Option::Some(input);
        self
    }
    /// Output video height, in pixels. Must be an even number. For most codecs, you can leave this field and width blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    pub fn set_height(mut self, input: ::std::option::Option<i32>) -> Self {
        self.height = input;
        self
    }
    /// The name of this VideoDescription. Outputs will use this name to uniquely identify this Description. Description names should be unique within this Live Event.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of this VideoDescription. Outputs will use this name to uniquely identify this Description. Description names should be unique within this Live Event.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Indicates how MediaLive will respond to the AFD values that might be in the input video. If you do not know what AFD signaling is, or if your downstream system has not given you guidance, choose PASSTHROUGH. RESPOND: MediaLive clips the input video using a formula that uses the AFD values (configured in afdSignaling ), the input display aspect ratio, and the output display aspect ratio. MediaLive also includes the AFD values in the output, unless the codec for this encode is FRAME_CAPTURE. PASSTHROUGH: MediaLive ignores the AFD values and does not clip the video. But MediaLive does include the values in the output. NONE: MediaLive does not clip the input video and does not include the AFD values in the output
    pub fn respond_to_afd(mut self, input: crate::types::VideoDescriptionRespondToAfd) -> Self {
        self.respond_to_afd = ::std::option::Option::Some(input);
        self
    }
    /// Indicates how MediaLive will respond to the AFD values that might be in the input video. If you do not know what AFD signaling is, or if your downstream system has not given you guidance, choose PASSTHROUGH. RESPOND: MediaLive clips the input video using a formula that uses the AFD values (configured in afdSignaling ), the input display aspect ratio, and the output display aspect ratio. MediaLive also includes the AFD values in the output, unless the codec for this encode is FRAME_CAPTURE. PASSTHROUGH: MediaLive ignores the AFD values and does not clip the video. But MediaLive does include the values in the output. NONE: MediaLive does not clip the input video and does not include the AFD values in the output
    pub fn set_respond_to_afd(
        mut self,
        input: ::std::option::Option<crate::types::VideoDescriptionRespondToAfd>,
    ) -> Self {
        self.respond_to_afd = input;
        self
    }
    /// STRETCH_TO_OUTPUT configures the output position to stretch the video to the specified output resolution (height and width). This option will override any position value. DEFAULT may insert black boxes (pillar boxes or letter boxes) around the video to provide the specified output resolution.
    pub fn scaling_behavior(
        mut self,
        input: crate::types::VideoDescriptionScalingBehavior,
    ) -> Self {
        self.scaling_behavior = ::std::option::Option::Some(input);
        self
    }
    /// STRETCH_TO_OUTPUT configures the output position to stretch the video to the specified output resolution (height and width). This option will override any position value. DEFAULT may insert black boxes (pillar boxes or letter boxes) around the video to provide the specified output resolution.
    pub fn set_scaling_behavior(
        mut self,
        input: ::std::option::Option<crate::types::VideoDescriptionScalingBehavior>,
    ) -> Self {
        self.scaling_behavior = input;
        self
    }
    /// Changes the strength of the anti-alias filter used for scaling. 0 is the softest setting, 100 is the sharpest. A setting of 50 is recommended for most content.
    pub fn sharpness(mut self, input: i32) -> Self {
        self.sharpness = ::std::option::Option::Some(input);
        self
    }
    /// Changes the strength of the anti-alias filter used for scaling. 0 is the softest setting, 100 is the sharpest. A setting of 50 is recommended for most content.
    pub fn set_sharpness(mut self, input: ::std::option::Option<i32>) -> Self {
        self.sharpness = input;
        self
    }
    /// Output video width, in pixels. Must be an even number. For most codecs, you can leave this field and height blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    pub fn width(mut self, input: i32) -> Self {
        self.width = ::std::option::Option::Some(input);
        self
    }
    /// Output video width, in pixels. Must be an even number. For most codecs, you can leave this field and height blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.
    pub fn set_width(mut self, input: ::std::option::Option<i32>) -> Self {
        self.width = input;
        self
    }
    /// Consumes the builder and constructs a [`VideoDescription`](crate::types::VideoDescription).
    pub fn build(self) -> crate::types::VideoDescription {
        crate::types::VideoDescription {
            codec_settings: self.codec_settings,
            height: self.height,
            name: self.name,
            respond_to_afd: self.respond_to_afd,
            scaling_behavior: self.scaling_behavior,
            sharpness: self.sharpness,
            width: self.width,
        }
    }
}
