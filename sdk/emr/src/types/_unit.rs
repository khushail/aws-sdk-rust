// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `Unit`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let unit = unimplemented!();
/// match unit {
///     Unit::Bits => { /* ... */ },
///     Unit::BitsPerSecond => { /* ... */ },
///     Unit::Bytes => { /* ... */ },
///     Unit::BytesPerSecond => { /* ... */ },
///     Unit::Count => { /* ... */ },
///     Unit::CountPerSecond => { /* ... */ },
///     Unit::GigaBits => { /* ... */ },
///     Unit::GigaBitsPerSecond => { /* ... */ },
///     Unit::GigaBytes => { /* ... */ },
///     Unit::GigaBytesPerSecond => { /* ... */ },
///     Unit::KiloBits => { /* ... */ },
///     Unit::KiloBitsPerSecond => { /* ... */ },
///     Unit::KiloBytes => { /* ... */ },
///     Unit::KiloBytesPerSecond => { /* ... */ },
///     Unit::MegaBits => { /* ... */ },
///     Unit::MegaBitsPerSecond => { /* ... */ },
///     Unit::MegaBytes => { /* ... */ },
///     Unit::MegaBytesPerSecond => { /* ... */ },
///     Unit::MicroSeconds => { /* ... */ },
///     Unit::MilliSeconds => { /* ... */ },
///     Unit::None => { /* ... */ },
///     Unit::Percent => { /* ... */ },
///     Unit::Seconds => { /* ... */ },
///     Unit::TeraBits => { /* ... */ },
///     Unit::TeraBitsPerSecond => { /* ... */ },
///     Unit::TeraBytes => { /* ... */ },
///     Unit::TeraBytesPerSecond => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `unit` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `Unit::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `Unit::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `Unit::NewFeature` is defined.
/// Specifically, when `unit` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `Unit::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
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
pub enum Unit {
    #[allow(missing_docs)] // documentation missing in model
    Bits,
    #[allow(missing_docs)] // documentation missing in model
    BitsPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    Bytes,
    #[allow(missing_docs)] // documentation missing in model
    BytesPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    Count,
    #[allow(missing_docs)] // documentation missing in model
    CountPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    GigaBits,
    #[allow(missing_docs)] // documentation missing in model
    GigaBitsPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    GigaBytes,
    #[allow(missing_docs)] // documentation missing in model
    GigaBytesPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    KiloBits,
    #[allow(missing_docs)] // documentation missing in model
    KiloBitsPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    KiloBytes,
    #[allow(missing_docs)] // documentation missing in model
    KiloBytesPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    MegaBits,
    #[allow(missing_docs)] // documentation missing in model
    MegaBitsPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    MegaBytes,
    #[allow(missing_docs)] // documentation missing in model
    MegaBytesPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    MicroSeconds,
    #[allow(missing_docs)] // documentation missing in model
    MilliSeconds,
    #[allow(missing_docs)] // documentation missing in model
    None,
    #[allow(missing_docs)] // documentation missing in model
    Percent,
    #[allow(missing_docs)] // documentation missing in model
    Seconds,
    #[allow(missing_docs)] // documentation missing in model
    TeraBits,
    #[allow(missing_docs)] // documentation missing in model
    TeraBitsPerSecond,
    #[allow(missing_docs)] // documentation missing in model
    TeraBytes,
    #[allow(missing_docs)] // documentation missing in model
    TeraBytesPerSecond,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for Unit {
    fn from(s: &str) -> Self {
        match s {
            "BITS" => Unit::Bits,
            "BITS_PER_SECOND" => Unit::BitsPerSecond,
            "BYTES" => Unit::Bytes,
            "BYTES_PER_SECOND" => Unit::BytesPerSecond,
            "COUNT" => Unit::Count,
            "COUNT_PER_SECOND" => Unit::CountPerSecond,
            "GIGA_BITS" => Unit::GigaBits,
            "GIGA_BITS_PER_SECOND" => Unit::GigaBitsPerSecond,
            "GIGA_BYTES" => Unit::GigaBytes,
            "GIGA_BYTES_PER_SECOND" => Unit::GigaBytesPerSecond,
            "KILO_BITS" => Unit::KiloBits,
            "KILO_BITS_PER_SECOND" => Unit::KiloBitsPerSecond,
            "KILO_BYTES" => Unit::KiloBytes,
            "KILO_BYTES_PER_SECOND" => Unit::KiloBytesPerSecond,
            "MEGA_BITS" => Unit::MegaBits,
            "MEGA_BITS_PER_SECOND" => Unit::MegaBitsPerSecond,
            "MEGA_BYTES" => Unit::MegaBytes,
            "MEGA_BYTES_PER_SECOND" => Unit::MegaBytesPerSecond,
            "MICRO_SECONDS" => Unit::MicroSeconds,
            "MILLI_SECONDS" => Unit::MilliSeconds,
            "NONE" => Unit::None,
            "PERCENT" => Unit::Percent,
            "SECONDS" => Unit::Seconds,
            "TERA_BITS" => Unit::TeraBits,
            "TERA_BITS_PER_SECOND" => Unit::TeraBitsPerSecond,
            "TERA_BYTES" => Unit::TeraBytes,
            "TERA_BYTES_PER_SECOND" => Unit::TeraBytesPerSecond,
            other => Unit::Unknown(crate::primitives::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for Unit {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(Unit::from(s))
    }
}
impl Unit {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            Unit::Bits => "BITS",
            Unit::BitsPerSecond => "BITS_PER_SECOND",
            Unit::Bytes => "BYTES",
            Unit::BytesPerSecond => "BYTES_PER_SECOND",
            Unit::Count => "COUNT",
            Unit::CountPerSecond => "COUNT_PER_SECOND",
            Unit::GigaBits => "GIGA_BITS",
            Unit::GigaBitsPerSecond => "GIGA_BITS_PER_SECOND",
            Unit::GigaBytes => "GIGA_BYTES",
            Unit::GigaBytesPerSecond => "GIGA_BYTES_PER_SECOND",
            Unit::KiloBits => "KILO_BITS",
            Unit::KiloBitsPerSecond => "KILO_BITS_PER_SECOND",
            Unit::KiloBytes => "KILO_BYTES",
            Unit::KiloBytesPerSecond => "KILO_BYTES_PER_SECOND",
            Unit::MegaBits => "MEGA_BITS",
            Unit::MegaBitsPerSecond => "MEGA_BITS_PER_SECOND",
            Unit::MegaBytes => "MEGA_BYTES",
            Unit::MegaBytesPerSecond => "MEGA_BYTES_PER_SECOND",
            Unit::MicroSeconds => "MICRO_SECONDS",
            Unit::MilliSeconds => "MILLI_SECONDS",
            Unit::None => "NONE",
            Unit::Percent => "PERCENT",
            Unit::Seconds => "SECONDS",
            Unit::TeraBits => "TERA_BITS",
            Unit::TeraBitsPerSecond => "TERA_BITS_PER_SECOND",
            Unit::TeraBytes => "TERA_BYTES",
            Unit::TeraBytesPerSecond => "TERA_BYTES_PER_SECOND",
            Unit::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "BITS",
            "BITS_PER_SECOND",
            "BYTES",
            "BYTES_PER_SECOND",
            "COUNT",
            "COUNT_PER_SECOND",
            "GIGA_BITS",
            "GIGA_BITS_PER_SECOND",
            "GIGA_BYTES",
            "GIGA_BYTES_PER_SECOND",
            "KILO_BITS",
            "KILO_BITS_PER_SECOND",
            "KILO_BYTES",
            "KILO_BYTES_PER_SECOND",
            "MEGA_BITS",
            "MEGA_BITS_PER_SECOND",
            "MEGA_BYTES",
            "MEGA_BYTES_PER_SECOND",
            "MICRO_SECONDS",
            "MILLI_SECONDS",
            "NONE",
            "PERCENT",
            "SECONDS",
            "TERA_BITS",
            "TERA_BITS_PER_SECOND",
            "TERA_BYTES",
            "TERA_BYTES_PER_SECOND",
        ]
    }
}
impl ::std::convert::AsRef<str> for Unit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
