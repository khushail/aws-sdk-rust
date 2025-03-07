// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An event source type, accompanied by one or more event category names.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EventCategoriesMap {
    /// <p>The source type that the returned categories belong to.</p>
    #[doc(hidden)]
    pub source_type: ::std::option::Option<::std::string::String>,
    /// <p>The event categories for the specified source type.</p>
    #[doc(hidden)]
    pub event_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl EventCategoriesMap {
    /// <p>The source type that the returned categories belong to.</p>
    pub fn source_type(&self) -> ::std::option::Option<&str> {
        self.source_type.as_deref()
    }
    /// <p>The event categories for the specified source type.</p>
    pub fn event_categories(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.event_categories.as_deref()
    }
}
impl EventCategoriesMap {
    /// Creates a new builder-style object to manufacture [`EventCategoriesMap`](crate::types::EventCategoriesMap).
    pub fn builder() -> crate::types::builders::EventCategoriesMapBuilder {
        crate::types::builders::EventCategoriesMapBuilder::default()
    }
}

/// A builder for [`EventCategoriesMap`](crate::types::EventCategoriesMap).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EventCategoriesMapBuilder {
    pub(crate) source_type: ::std::option::Option<::std::string::String>,
    pub(crate) event_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl EventCategoriesMapBuilder {
    /// <p>The source type that the returned categories belong to.</p>
    pub fn source_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source type that the returned categories belong to.</p>
    pub fn set_source_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_type = input;
        self
    }
    /// Appends an item to `event_categories`.
    ///
    /// To override the contents of this collection use [`set_event_categories`](Self::set_event_categories).
    ///
    /// <p>The event categories for the specified source type.</p>
    pub fn event_categories(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.event_categories.unwrap_or_default();
        v.push(input.into());
        self.event_categories = ::std::option::Option::Some(v);
        self
    }
    /// <p>The event categories for the specified source type.</p>
    pub fn set_event_categories(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.event_categories = input;
        self
    }
    /// Consumes the builder and constructs a [`EventCategoriesMap`](crate::types::EventCategoriesMap).
    pub fn build(self) -> crate::types::EventCategoriesMap {
        crate::types::EventCategoriesMap {
            source_type: self.source_type,
            event_categories: self.event_categories,
        }
    }
}
