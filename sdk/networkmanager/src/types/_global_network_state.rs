// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `GlobalNetworkState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let globalnetworkstate = unimplemented!();
/// match globalnetworkstate {
///     GlobalNetworkState::Available => { /* ... */ },
///     GlobalNetworkState::Deleting => { /* ... */ },
///     GlobalNetworkState::Pending => { /* ... */ },
///     GlobalNetworkState::Updating => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `globalnetworkstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `GlobalNetworkState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `GlobalNetworkState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `GlobalNetworkState::NewFeature` is defined.
/// Specifically, when `globalnetworkstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `GlobalNetworkState::NewFeature` also yielding `"NewFeature"`.
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
pub enum GlobalNetworkState {
    #[allow(missing_docs)] // documentation missing in model
    Available,
    #[allow(missing_docs)] // documentation missing in model
    Deleting,
    #[allow(missing_docs)] // documentation missing in model
    Pending,
    #[allow(missing_docs)] // documentation missing in model
    Updating,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for GlobalNetworkState {
    fn from(s: &str) -> Self {
        match s {
            "AVAILABLE" => GlobalNetworkState::Available,
            "DELETING" => GlobalNetworkState::Deleting,
            "PENDING" => GlobalNetworkState::Pending,
            "UPDATING" => GlobalNetworkState::Updating,
            other => GlobalNetworkState::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl ::std::str::FromStr for GlobalNetworkState {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(GlobalNetworkState::from(s))
    }
}
impl GlobalNetworkState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            GlobalNetworkState::Available => "AVAILABLE",
            GlobalNetworkState::Deleting => "DELETING",
            GlobalNetworkState::Pending => "PENDING",
            GlobalNetworkState::Updating => "UPDATING",
            GlobalNetworkState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["AVAILABLE", "DELETING", "PENDING", "UPDATING"]
    }
}
impl ::std::convert::AsRef<str> for GlobalNetworkState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
