// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `LanguageCode`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let languagecode = unimplemented!();
/// match languagecode {
///     LanguageCode::DeDe => { /* ... */ },
///     LanguageCode::EnAu => { /* ... */ },
///     LanguageCode::EnGb => { /* ... */ },
///     LanguageCode::EnUs => { /* ... */ },
///     LanguageCode::EsUs => { /* ... */ },
///     LanguageCode::FrCa => { /* ... */ },
///     LanguageCode::FrFr => { /* ... */ },
///     LanguageCode::HiIn => { /* ... */ },
///     LanguageCode::ItIt => { /* ... */ },
///     LanguageCode::JaJp => { /* ... */ },
///     LanguageCode::KoKr => { /* ... */ },
///     LanguageCode::PtBr => { /* ... */ },
///     LanguageCode::ThTh => { /* ... */ },
///     LanguageCode::ZhCn => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `languagecode` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `LanguageCode::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `LanguageCode::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `LanguageCode::NewFeature` is defined.
/// Specifically, when `languagecode` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `LanguageCode::NewFeature` also yielding `"NewFeature"`.
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
pub enum LanguageCode {
    #[allow(missing_docs)] // documentation missing in model
    DeDe,
    #[allow(missing_docs)] // documentation missing in model
    EnAu,
    #[allow(missing_docs)] // documentation missing in model
    EnGb,
    #[allow(missing_docs)] // documentation missing in model
    EnUs,
    #[allow(missing_docs)] // documentation missing in model
    EsUs,
    #[allow(missing_docs)] // documentation missing in model
    FrCa,
    #[allow(missing_docs)] // documentation missing in model
    FrFr,
    #[allow(missing_docs)] // documentation missing in model
    HiIn,
    #[allow(missing_docs)] // documentation missing in model
    ItIt,
    #[allow(missing_docs)] // documentation missing in model
    JaJp,
    #[allow(missing_docs)] // documentation missing in model
    KoKr,
    #[allow(missing_docs)] // documentation missing in model
    PtBr,
    #[allow(missing_docs)] // documentation missing in model
    ThTh,
    #[allow(missing_docs)] // documentation missing in model
    ZhCn,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for LanguageCode {
    fn from(s: &str) -> Self {
        match s {
            "de-DE" => LanguageCode::DeDe,
            "en-AU" => LanguageCode::EnAu,
            "en-GB" => LanguageCode::EnGb,
            "en-US" => LanguageCode::EnUs,
            "es-US" => LanguageCode::EsUs,
            "fr-CA" => LanguageCode::FrCa,
            "fr-FR" => LanguageCode::FrFr,
            "hi-IN" => LanguageCode::HiIn,
            "it-IT" => LanguageCode::ItIt,
            "ja-JP" => LanguageCode::JaJp,
            "ko-KR" => LanguageCode::KoKr,
            "pt-BR" => LanguageCode::PtBr,
            "th-TH" => LanguageCode::ThTh,
            "zh-CN" => LanguageCode::ZhCn,
            other => {
                LanguageCode::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl ::std::str::FromStr for LanguageCode {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(LanguageCode::from(s))
    }
}
impl LanguageCode {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            LanguageCode::DeDe => "de-DE",
            LanguageCode::EnAu => "en-AU",
            LanguageCode::EnGb => "en-GB",
            LanguageCode::EnUs => "en-US",
            LanguageCode::EsUs => "es-US",
            LanguageCode::FrCa => "fr-CA",
            LanguageCode::FrFr => "fr-FR",
            LanguageCode::HiIn => "hi-IN",
            LanguageCode::ItIt => "it-IT",
            LanguageCode::JaJp => "ja-JP",
            LanguageCode::KoKr => "ko-KR",
            LanguageCode::PtBr => "pt-BR",
            LanguageCode::ThTh => "th-TH",
            LanguageCode::ZhCn => "zh-CN",
            LanguageCode::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "de-DE", "en-AU", "en-GB", "en-US", "es-US", "fr-CA", "fr-FR", "hi-IN", "it-IT",
            "ja-JP", "ko-KR", "pt-BR", "th-TH", "zh-CN",
        ]
    }
}
impl ::std::convert::AsRef<str> for LanguageCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
