// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Encoder Settings
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EncoderSettings {
    /// Placeholder documentation for __listOfAudioDescription
    #[doc(hidden)]
    pub audio_descriptions: ::std::option::Option<::std::vec::Vec<crate::types::AudioDescription>>,
    /// Settings for ad avail blanking.
    #[doc(hidden)]
    pub avail_blanking: ::std::option::Option<crate::types::AvailBlanking>,
    /// Event-wide configuration settings for ad avail insertion.
    #[doc(hidden)]
    pub avail_configuration: ::std::option::Option<crate::types::AvailConfiguration>,
    /// Settings for blackout slate.
    #[doc(hidden)]
    pub blackout_slate: ::std::option::Option<crate::types::BlackoutSlate>,
    /// Settings for caption decriptions
    #[doc(hidden)]
    pub caption_descriptions:
        ::std::option::Option<::std::vec::Vec<crate::types::CaptionDescription>>,
    /// Feature Activations
    #[doc(hidden)]
    pub feature_activations: ::std::option::Option<crate::types::FeatureActivations>,
    /// Configuration settings that apply to the event as a whole.
    #[doc(hidden)]
    pub global_configuration: ::std::option::Option<crate::types::GlobalConfiguration>,
    /// Settings for motion graphics.
    #[doc(hidden)]
    pub motion_graphics_configuration:
        ::std::option::Option<crate::types::MotionGraphicsConfiguration>,
    /// Nielsen configuration settings.
    #[doc(hidden)]
    pub nielsen_configuration: ::std::option::Option<crate::types::NielsenConfiguration>,
    /// Placeholder documentation for __listOfOutputGroup
    #[doc(hidden)]
    pub output_groups: ::std::option::Option<::std::vec::Vec<crate::types::OutputGroup>>,
    /// Contains settings used to acquire and adjust timecode information from inputs.
    #[doc(hidden)]
    pub timecode_config: ::std::option::Option<crate::types::TimecodeConfig>,
    /// Placeholder documentation for __listOfVideoDescription
    #[doc(hidden)]
    pub video_descriptions: ::std::option::Option<::std::vec::Vec<crate::types::VideoDescription>>,
}
impl EncoderSettings {
    /// Placeholder documentation for __listOfAudioDescription
    pub fn audio_descriptions(&self) -> ::std::option::Option<&[crate::types::AudioDescription]> {
        self.audio_descriptions.as_deref()
    }
    /// Settings for ad avail blanking.
    pub fn avail_blanking(&self) -> ::std::option::Option<&crate::types::AvailBlanking> {
        self.avail_blanking.as_ref()
    }
    /// Event-wide configuration settings for ad avail insertion.
    pub fn avail_configuration(&self) -> ::std::option::Option<&crate::types::AvailConfiguration> {
        self.avail_configuration.as_ref()
    }
    /// Settings for blackout slate.
    pub fn blackout_slate(&self) -> ::std::option::Option<&crate::types::BlackoutSlate> {
        self.blackout_slate.as_ref()
    }
    /// Settings for caption decriptions
    pub fn caption_descriptions(
        &self,
    ) -> ::std::option::Option<&[crate::types::CaptionDescription]> {
        self.caption_descriptions.as_deref()
    }
    /// Feature Activations
    pub fn feature_activations(&self) -> ::std::option::Option<&crate::types::FeatureActivations> {
        self.feature_activations.as_ref()
    }
    /// Configuration settings that apply to the event as a whole.
    pub fn global_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::GlobalConfiguration> {
        self.global_configuration.as_ref()
    }
    /// Settings for motion graphics.
    pub fn motion_graphics_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::MotionGraphicsConfiguration> {
        self.motion_graphics_configuration.as_ref()
    }
    /// Nielsen configuration settings.
    pub fn nielsen_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::NielsenConfiguration> {
        self.nielsen_configuration.as_ref()
    }
    /// Placeholder documentation for __listOfOutputGroup
    pub fn output_groups(&self) -> ::std::option::Option<&[crate::types::OutputGroup]> {
        self.output_groups.as_deref()
    }
    /// Contains settings used to acquire and adjust timecode information from inputs.
    pub fn timecode_config(&self) -> ::std::option::Option<&crate::types::TimecodeConfig> {
        self.timecode_config.as_ref()
    }
    /// Placeholder documentation for __listOfVideoDescription
    pub fn video_descriptions(&self) -> ::std::option::Option<&[crate::types::VideoDescription]> {
        self.video_descriptions.as_deref()
    }
}
impl EncoderSettings {
    /// Creates a new builder-style object to manufacture [`EncoderSettings`](crate::types::EncoderSettings).
    pub fn builder() -> crate::types::builders::EncoderSettingsBuilder {
        crate::types::builders::EncoderSettingsBuilder::default()
    }
}

/// A builder for [`EncoderSettings`](crate::types::EncoderSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EncoderSettingsBuilder {
    pub(crate) audio_descriptions:
        ::std::option::Option<::std::vec::Vec<crate::types::AudioDescription>>,
    pub(crate) avail_blanking: ::std::option::Option<crate::types::AvailBlanking>,
    pub(crate) avail_configuration: ::std::option::Option<crate::types::AvailConfiguration>,
    pub(crate) blackout_slate: ::std::option::Option<crate::types::BlackoutSlate>,
    pub(crate) caption_descriptions:
        ::std::option::Option<::std::vec::Vec<crate::types::CaptionDescription>>,
    pub(crate) feature_activations: ::std::option::Option<crate::types::FeatureActivations>,
    pub(crate) global_configuration: ::std::option::Option<crate::types::GlobalConfiguration>,
    pub(crate) motion_graphics_configuration:
        ::std::option::Option<crate::types::MotionGraphicsConfiguration>,
    pub(crate) nielsen_configuration: ::std::option::Option<crate::types::NielsenConfiguration>,
    pub(crate) output_groups: ::std::option::Option<::std::vec::Vec<crate::types::OutputGroup>>,
    pub(crate) timecode_config: ::std::option::Option<crate::types::TimecodeConfig>,
    pub(crate) video_descriptions:
        ::std::option::Option<::std::vec::Vec<crate::types::VideoDescription>>,
}
impl EncoderSettingsBuilder {
    /// Appends an item to `audio_descriptions`.
    ///
    /// To override the contents of this collection use [`set_audio_descriptions`](Self::set_audio_descriptions).
    ///
    /// Placeholder documentation for __listOfAudioDescription
    pub fn audio_descriptions(mut self, input: crate::types::AudioDescription) -> Self {
        let mut v = self.audio_descriptions.unwrap_or_default();
        v.push(input);
        self.audio_descriptions = ::std::option::Option::Some(v);
        self
    }
    /// Placeholder documentation for __listOfAudioDescription
    pub fn set_audio_descriptions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AudioDescription>>,
    ) -> Self {
        self.audio_descriptions = input;
        self
    }
    /// Settings for ad avail blanking.
    pub fn avail_blanking(mut self, input: crate::types::AvailBlanking) -> Self {
        self.avail_blanking = ::std::option::Option::Some(input);
        self
    }
    /// Settings for ad avail blanking.
    pub fn set_avail_blanking(
        mut self,
        input: ::std::option::Option<crate::types::AvailBlanking>,
    ) -> Self {
        self.avail_blanking = input;
        self
    }
    /// Event-wide configuration settings for ad avail insertion.
    pub fn avail_configuration(mut self, input: crate::types::AvailConfiguration) -> Self {
        self.avail_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Event-wide configuration settings for ad avail insertion.
    pub fn set_avail_configuration(
        mut self,
        input: ::std::option::Option<crate::types::AvailConfiguration>,
    ) -> Self {
        self.avail_configuration = input;
        self
    }
    /// Settings for blackout slate.
    pub fn blackout_slate(mut self, input: crate::types::BlackoutSlate) -> Self {
        self.blackout_slate = ::std::option::Option::Some(input);
        self
    }
    /// Settings for blackout slate.
    pub fn set_blackout_slate(
        mut self,
        input: ::std::option::Option<crate::types::BlackoutSlate>,
    ) -> Self {
        self.blackout_slate = input;
        self
    }
    /// Appends an item to `caption_descriptions`.
    ///
    /// To override the contents of this collection use [`set_caption_descriptions`](Self::set_caption_descriptions).
    ///
    /// Settings for caption decriptions
    pub fn caption_descriptions(mut self, input: crate::types::CaptionDescription) -> Self {
        let mut v = self.caption_descriptions.unwrap_or_default();
        v.push(input);
        self.caption_descriptions = ::std::option::Option::Some(v);
        self
    }
    /// Settings for caption decriptions
    pub fn set_caption_descriptions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CaptionDescription>>,
    ) -> Self {
        self.caption_descriptions = input;
        self
    }
    /// Feature Activations
    pub fn feature_activations(mut self, input: crate::types::FeatureActivations) -> Self {
        self.feature_activations = ::std::option::Option::Some(input);
        self
    }
    /// Feature Activations
    pub fn set_feature_activations(
        mut self,
        input: ::std::option::Option<crate::types::FeatureActivations>,
    ) -> Self {
        self.feature_activations = input;
        self
    }
    /// Configuration settings that apply to the event as a whole.
    pub fn global_configuration(mut self, input: crate::types::GlobalConfiguration) -> Self {
        self.global_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Configuration settings that apply to the event as a whole.
    pub fn set_global_configuration(
        mut self,
        input: ::std::option::Option<crate::types::GlobalConfiguration>,
    ) -> Self {
        self.global_configuration = input;
        self
    }
    /// Settings for motion graphics.
    pub fn motion_graphics_configuration(
        mut self,
        input: crate::types::MotionGraphicsConfiguration,
    ) -> Self {
        self.motion_graphics_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Settings for motion graphics.
    pub fn set_motion_graphics_configuration(
        mut self,
        input: ::std::option::Option<crate::types::MotionGraphicsConfiguration>,
    ) -> Self {
        self.motion_graphics_configuration = input;
        self
    }
    /// Nielsen configuration settings.
    pub fn nielsen_configuration(mut self, input: crate::types::NielsenConfiguration) -> Self {
        self.nielsen_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Nielsen configuration settings.
    pub fn set_nielsen_configuration(
        mut self,
        input: ::std::option::Option<crate::types::NielsenConfiguration>,
    ) -> Self {
        self.nielsen_configuration = input;
        self
    }
    /// Appends an item to `output_groups`.
    ///
    /// To override the contents of this collection use [`set_output_groups`](Self::set_output_groups).
    ///
    /// Placeholder documentation for __listOfOutputGroup
    pub fn output_groups(mut self, input: crate::types::OutputGroup) -> Self {
        let mut v = self.output_groups.unwrap_or_default();
        v.push(input);
        self.output_groups = ::std::option::Option::Some(v);
        self
    }
    /// Placeholder documentation for __listOfOutputGroup
    pub fn set_output_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OutputGroup>>,
    ) -> Self {
        self.output_groups = input;
        self
    }
    /// Contains settings used to acquire and adjust timecode information from inputs.
    pub fn timecode_config(mut self, input: crate::types::TimecodeConfig) -> Self {
        self.timecode_config = ::std::option::Option::Some(input);
        self
    }
    /// Contains settings used to acquire and adjust timecode information from inputs.
    pub fn set_timecode_config(
        mut self,
        input: ::std::option::Option<crate::types::TimecodeConfig>,
    ) -> Self {
        self.timecode_config = input;
        self
    }
    /// Appends an item to `video_descriptions`.
    ///
    /// To override the contents of this collection use [`set_video_descriptions`](Self::set_video_descriptions).
    ///
    /// Placeholder documentation for __listOfVideoDescription
    pub fn video_descriptions(mut self, input: crate::types::VideoDescription) -> Self {
        let mut v = self.video_descriptions.unwrap_or_default();
        v.push(input);
        self.video_descriptions = ::std::option::Option::Some(v);
        self
    }
    /// Placeholder documentation for __listOfVideoDescription
    pub fn set_video_descriptions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VideoDescription>>,
    ) -> Self {
        self.video_descriptions = input;
        self
    }
    /// Consumes the builder and constructs a [`EncoderSettings`](crate::types::EncoderSettings).
    pub fn build(self) -> crate::types::EncoderSettings {
        crate::types::EncoderSettings {
            audio_descriptions: self.audio_descriptions,
            avail_blanking: self.avail_blanking,
            avail_configuration: self.avail_configuration,
            blackout_slate: self.blackout_slate,
            caption_descriptions: self.caption_descriptions,
            feature_activations: self.feature_activations,
            global_configuration: self.global_configuration,
            motion_graphics_configuration: self.motion_graphics_configuration,
            nielsen_configuration: self.nielsen_configuration,
            output_groups: self.output_groups,
            timecode_config: self.timecode_config,
            video_descriptions: self.video_descriptions,
        }
    }
}
