// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a filter.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SavingsPlanRateFilter {
    /// <p>The filter name.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::SavingsPlanRateFilterName>,
    /// <p>The filter values.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl SavingsPlanRateFilter {
    /// <p>The filter name.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::SavingsPlanRateFilterName> {
        self.name.as_ref()
    }
    /// <p>The filter values.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl SavingsPlanRateFilter {
    /// Creates a new builder-style object to manufacture [`SavingsPlanRateFilter`](crate::types::SavingsPlanRateFilter).
    pub fn builder() -> crate::types::builders::SavingsPlanRateFilterBuilder {
        crate::types::builders::SavingsPlanRateFilterBuilder::default()
    }
}

/// A builder for [`SavingsPlanRateFilter`](crate::types::SavingsPlanRateFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SavingsPlanRateFilterBuilder {
    pub(crate) name: ::std::option::Option<crate::types::SavingsPlanRateFilterName>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl SavingsPlanRateFilterBuilder {
    /// <p>The filter name.</p>
    pub fn name(mut self, input: crate::types::SavingsPlanRateFilterName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The filter name.</p>
    pub fn set_name(
        mut self,
        input: ::std::option::Option<crate::types::SavingsPlanRateFilterName>,
    ) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The filter values.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filter values.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`SavingsPlanRateFilter`](crate::types::SavingsPlanRateFilter).
    pub fn build(self) -> crate::types::SavingsPlanRateFilter {
        crate::types::SavingsPlanRateFilter {
            name: self.name,
            values: self.values,
        }
    }
}
