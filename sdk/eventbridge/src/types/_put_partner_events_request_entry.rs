// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details about an event generated by an SaaS partner.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutPartnerEventsRequestEntry {
    /// <p>The date and time of the event.</p>
    #[doc(hidden)]
    pub time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The event source that is generating the entry.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<::std::string::String>,
    /// <p>Amazon Web Services resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>
    #[doc(hidden)]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.</p>
    #[doc(hidden)]
    pub detail_type: ::std::option::Option<::std::string::String>,
    /// <p>A valid JSON string. There is no other schema imposed. The JSON string may contain fields and nested subobjects.</p>
    #[doc(hidden)]
    pub detail: ::std::option::Option<::std::string::String>,
}
impl PutPartnerEventsRequestEntry {
    /// <p>The date and time of the event.</p>
    pub fn time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.time.as_ref()
    }
    /// <p>The event source that is generating the entry.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>Amazon Web Services resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>
    pub fn resources(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.resources.as_deref()
    }
    /// <p>A free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.</p>
    pub fn detail_type(&self) -> ::std::option::Option<&str> {
        self.detail_type.as_deref()
    }
    /// <p>A valid JSON string. There is no other schema imposed. The JSON string may contain fields and nested subobjects.</p>
    pub fn detail(&self) -> ::std::option::Option<&str> {
        self.detail.as_deref()
    }
}
impl PutPartnerEventsRequestEntry {
    /// Creates a new builder-style object to manufacture [`PutPartnerEventsRequestEntry`](crate::types::PutPartnerEventsRequestEntry).
    pub fn builder() -> crate::types::builders::PutPartnerEventsRequestEntryBuilder {
        crate::types::builders::PutPartnerEventsRequestEntryBuilder::default()
    }
}

/// A builder for [`PutPartnerEventsRequestEntry`](crate::types::PutPartnerEventsRequestEntry).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutPartnerEventsRequestEntryBuilder {
    pub(crate) time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) source: ::std::option::Option<::std::string::String>,
    pub(crate) resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) detail_type: ::std::option::Option<::std::string::String>,
    pub(crate) detail: ::std::option::Option<::std::string::String>,
}
impl PutPartnerEventsRequestEntryBuilder {
    /// <p>The date and time of the event.</p>
    pub fn time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time of the event.</p>
    pub fn set_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.time = input;
        self
    }
    /// <p>The event source that is generating the entry.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The event source that is generating the entry.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// Appends an item to `resources`.
    ///
    /// To override the contents of this collection use [`set_resources`](Self::set_resources).
    ///
    /// <p>Amazon Web Services resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>
    pub fn resources(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.resources.unwrap_or_default();
        v.push(input.into());
        self.resources = ::std::option::Option::Some(v);
        self
    }
    /// <p>Amazon Web Services resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>
    pub fn set_resources(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.resources = input;
        self
    }
    /// <p>A free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.</p>
    pub fn detail_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.detail_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.</p>
    pub fn set_detail_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.detail_type = input;
        self
    }
    /// <p>A valid JSON string. There is no other schema imposed. The JSON string may contain fields and nested subobjects.</p>
    pub fn detail(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.detail = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A valid JSON string. There is no other schema imposed. The JSON string may contain fields and nested subobjects.</p>
    pub fn set_detail(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.detail = input;
        self
    }
    /// Consumes the builder and constructs a [`PutPartnerEventsRequestEntry`](crate::types::PutPartnerEventsRequestEntry).
    pub fn build(self) -> crate::types::PutPartnerEventsRequestEntry {
        crate::types::PutPartnerEventsRequestEntry {
            time: self.time,
            source: self.source,
            resources: self.resources,
            detail_type: self.detail_type,
            detail: self.detail,
        }
    }
}
