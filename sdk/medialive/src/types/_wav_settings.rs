// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Wav Settings
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct WavSettings {
    /// Bits per sample.
    #[doc(hidden)]
    pub bit_depth: ::std::option::Option<f64>,
    /// The audio coding mode for the WAV audio. The mode determines the number of channels in the audio.
    #[doc(hidden)]
    pub coding_mode: ::std::option::Option<crate::types::WavCodingMode>,
    /// Sample rate in Hz.
    #[doc(hidden)]
    pub sample_rate: ::std::option::Option<f64>,
}
impl WavSettings {
    /// Bits per sample.
    pub fn bit_depth(&self) -> ::std::option::Option<f64> {
        self.bit_depth
    }
    /// The audio coding mode for the WAV audio. The mode determines the number of channels in the audio.
    pub fn coding_mode(&self) -> ::std::option::Option<&crate::types::WavCodingMode> {
        self.coding_mode.as_ref()
    }
    /// Sample rate in Hz.
    pub fn sample_rate(&self) -> ::std::option::Option<f64> {
        self.sample_rate
    }
}
impl WavSettings {
    /// Creates a new builder-style object to manufacture [`WavSettings`](crate::types::WavSettings).
    pub fn builder() -> crate::types::builders::WavSettingsBuilder {
        crate::types::builders::WavSettingsBuilder::default()
    }
}

/// A builder for [`WavSettings`](crate::types::WavSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WavSettingsBuilder {
    pub(crate) bit_depth: ::std::option::Option<f64>,
    pub(crate) coding_mode: ::std::option::Option<crate::types::WavCodingMode>,
    pub(crate) sample_rate: ::std::option::Option<f64>,
}
impl WavSettingsBuilder {
    /// Bits per sample.
    pub fn bit_depth(mut self, input: f64) -> Self {
        self.bit_depth = ::std::option::Option::Some(input);
        self
    }
    /// Bits per sample.
    pub fn set_bit_depth(mut self, input: ::std::option::Option<f64>) -> Self {
        self.bit_depth = input;
        self
    }
    /// The audio coding mode for the WAV audio. The mode determines the number of channels in the audio.
    pub fn coding_mode(mut self, input: crate::types::WavCodingMode) -> Self {
        self.coding_mode = ::std::option::Option::Some(input);
        self
    }
    /// The audio coding mode for the WAV audio. The mode determines the number of channels in the audio.
    pub fn set_coding_mode(
        mut self,
        input: ::std::option::Option<crate::types::WavCodingMode>,
    ) -> Self {
        self.coding_mode = input;
        self
    }
    /// Sample rate in Hz.
    pub fn sample_rate(mut self, input: f64) -> Self {
        self.sample_rate = ::std::option::Option::Some(input);
        self
    }
    /// Sample rate in Hz.
    pub fn set_sample_rate(mut self, input: ::std::option::Option<f64>) -> Self {
        self.sample_rate = input;
        self
    }
    /// Consumes the builder and constructs a [`WavSettings`](crate::types::WavSettings).
    pub fn build(self) -> crate::types::WavSettings {
        crate::types::WavSettings {
            bit_depth: self.bit_depth,
            coding_mode: self.coding_mode,
            sample_rate: self.sample_rate,
        }
    }
}
