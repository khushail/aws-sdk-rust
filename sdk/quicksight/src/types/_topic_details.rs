// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that describes the details of a topic, such as its name, description, and associated data sets.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TopicDetails {
    /// <p>The name of the topic.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the topic.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The data sets that the topic is associated with.</p>
    #[doc(hidden)]
    pub data_sets: ::std::option::Option<::std::vec::Vec<crate::types::DatasetMetadata>>,
}
impl TopicDetails {
    /// <p>The name of the topic.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the topic.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The data sets that the topic is associated with.</p>
    pub fn data_sets(&self) -> ::std::option::Option<&[crate::types::DatasetMetadata]> {
        self.data_sets.as_deref()
    }
}
impl TopicDetails {
    /// Creates a new builder-style object to manufacture [`TopicDetails`](crate::types::TopicDetails).
    pub fn builder() -> crate::types::builders::TopicDetailsBuilder {
        crate::types::builders::TopicDetailsBuilder::default()
    }
}

/// A builder for [`TopicDetails`](crate::types::TopicDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TopicDetailsBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) data_sets: ::std::option::Option<::std::vec::Vec<crate::types::DatasetMetadata>>,
}
impl TopicDetailsBuilder {
    /// <p>The name of the topic.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the topic.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The description of the topic.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the topic.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `data_sets`.
    ///
    /// To override the contents of this collection use [`set_data_sets`](Self::set_data_sets).
    ///
    /// <p>The data sets that the topic is associated with.</p>
    pub fn data_sets(mut self, input: crate::types::DatasetMetadata) -> Self {
        let mut v = self.data_sets.unwrap_or_default();
        v.push(input);
        self.data_sets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The data sets that the topic is associated with.</p>
    pub fn set_data_sets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DatasetMetadata>>,
    ) -> Self {
        self.data_sets = input;
        self
    }
    /// Consumes the builder and constructs a [`TopicDetails`](crate::types::TopicDetails).
    pub fn build(self) -> crate::types::TopicDetails {
        crate::types::TopicDetails {
            name: self.name,
            description: self.description,
            data_sets: self.data_sets,
        }
    }
}
