// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ReservationCodec`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let reservationcodec = unimplemented!();
/// match reservationcodec {
///     ReservationCodec::Audio => { /* ... */ },
///     ReservationCodec::Avc => { /* ... */ },
///     ReservationCodec::Hevc => { /* ... */ },
///     ReservationCodec::Link => { /* ... */ },
///     ReservationCodec::Mpeg2 => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `reservationcodec` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ReservationCodec::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ReservationCodec::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ReservationCodec::NewFeature` is defined.
/// Specifically, when `reservationcodec` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ReservationCodec::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// Codec, 'MPEG2', 'AVC', 'HEVC', or 'AUDIO'
#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum ReservationCodec {
    #[allow(missing_docs)] // documentation missing in model
    Audio,
    #[allow(missing_docs)] // documentation missing in model
    Avc,
    #[allow(missing_docs)] // documentation missing in model
    Hevc,
    #[allow(missing_docs)] // documentation missing in model
    Link,
    #[allow(missing_docs)] // documentation missing in model
    Mpeg2,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for ReservationCodec {
    fn from(s: &str) -> Self {
        match s {
            "AUDIO" => ReservationCodec::Audio,
            "AVC" => ReservationCodec::Avc,
            "HEVC" => ReservationCodec::Hevc,
            "LINK" => ReservationCodec::Link,
            "MPEG2" => ReservationCodec::Mpeg2,
            other => {
                ReservationCodec::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl ::std::str::FromStr for ReservationCodec {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(ReservationCodec::from(s))
    }
}
impl ReservationCodec {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ReservationCodec::Audio => "AUDIO",
            ReservationCodec::Avc => "AVC",
            ReservationCodec::Hevc => "HEVC",
            ReservationCodec::Link => "LINK",
            ReservationCodec::Mpeg2 => "MPEG2",
            ReservationCodec::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["AUDIO", "AVC", "HEVC", "LINK", "MPEG2"]
    }
}
impl ::std::convert::AsRef<str> for ReservationCodec {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
