// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The country and area code for a proxy phone number in a proxy phone session.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GeoMatchParams {
    /// <p>The country.</p>
    #[doc(hidden)]
    pub country: ::std::option::Option<::std::string::String>,
    /// <p>The area code.</p>
    #[doc(hidden)]
    pub area_code: ::std::option::Option<::std::string::String>,
}
impl GeoMatchParams {
    /// <p>The country.</p>
    pub fn country(&self) -> ::std::option::Option<&str> {
        self.country.as_deref()
    }
    /// <p>The area code.</p>
    pub fn area_code(&self) -> ::std::option::Option<&str> {
        self.area_code.as_deref()
    }
}
impl GeoMatchParams {
    /// Creates a new builder-style object to manufacture [`GeoMatchParams`](crate::types::GeoMatchParams).
    pub fn builder() -> crate::types::builders::GeoMatchParamsBuilder {
        crate::types::builders::GeoMatchParamsBuilder::default()
    }
}

/// A builder for [`GeoMatchParams`](crate::types::GeoMatchParams).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GeoMatchParamsBuilder {
    pub(crate) country: ::std::option::Option<::std::string::String>,
    pub(crate) area_code: ::std::option::Option<::std::string::String>,
}
impl GeoMatchParamsBuilder {
    /// <p>The country.</p>
    pub fn country(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.country = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The country.</p>
    pub fn set_country(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.country = input;
        self
    }
    /// <p>The area code.</p>
    pub fn area_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.area_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The area code.</p>
    pub fn set_area_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.area_code = input;
        self
    }
    /// Consumes the builder and constructs a [`GeoMatchParams`](crate::types::GeoMatchParams).
    pub fn build(self) -> crate::types::GeoMatchParams {
        crate::types::GeoMatchParams {
            country: self.country,
            area_code: self.area_code,
        }
    }
}
