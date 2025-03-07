// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information related to a given configuration set in your Amazon Web Services account.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConfigurationSetInformation {
    /// <p>The Resource Name (ARN) of the ConfigurationSet.</p>
    #[doc(hidden)]
    pub configuration_set_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the ConfigurationSet.</p>
    #[doc(hidden)]
    pub configuration_set_name: ::std::option::Option<::std::string::String>,
    /// <p>An array of EventDestination objects that describe any events to log and where to log them.</p>
    #[doc(hidden)]
    pub event_destinations: ::std::option::Option<::std::vec::Vec<crate::types::EventDestination>>,
    /// <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive.</p>
    #[doc(hidden)]
    pub default_message_type: ::std::option::Option<crate::types::MessageType>,
    /// <p>The default sender ID used by the ConfigurationSet.</p>
    #[doc(hidden)]
    pub default_sender_id: ::std::option::Option<::std::string::String>,
    /// <p>The time when the ConfigurationSet was created, in <a href="https://www.epochconverter.com/">UNIX epoch time</a> format.</p>
    #[doc(hidden)]
    pub created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ConfigurationSetInformation {
    /// <p>The Resource Name (ARN) of the ConfigurationSet.</p>
    pub fn configuration_set_arn(&self) -> ::std::option::Option<&str> {
        self.configuration_set_arn.as_deref()
    }
    /// <p>The name of the ConfigurationSet.</p>
    pub fn configuration_set_name(&self) -> ::std::option::Option<&str> {
        self.configuration_set_name.as_deref()
    }
    /// <p>An array of EventDestination objects that describe any events to log and where to log them.</p>
    pub fn event_destinations(&self) -> ::std::option::Option<&[crate::types::EventDestination]> {
        self.event_destinations.as_deref()
    }
    /// <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive.</p>
    pub fn default_message_type(&self) -> ::std::option::Option<&crate::types::MessageType> {
        self.default_message_type.as_ref()
    }
    /// <p>The default sender ID used by the ConfigurationSet.</p>
    pub fn default_sender_id(&self) -> ::std::option::Option<&str> {
        self.default_sender_id.as_deref()
    }
    /// <p>The time when the ConfigurationSet was created, in <a href="https://www.epochconverter.com/">UNIX epoch time</a> format.</p>
    pub fn created_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_timestamp.as_ref()
    }
}
impl ConfigurationSetInformation {
    /// Creates a new builder-style object to manufacture [`ConfigurationSetInformation`](crate::types::ConfigurationSetInformation).
    pub fn builder() -> crate::types::builders::ConfigurationSetInformationBuilder {
        crate::types::builders::ConfigurationSetInformationBuilder::default()
    }
}

/// A builder for [`ConfigurationSetInformation`](crate::types::ConfigurationSetInformation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConfigurationSetInformationBuilder {
    pub(crate) configuration_set_arn: ::std::option::Option<::std::string::String>,
    pub(crate) configuration_set_name: ::std::option::Option<::std::string::String>,
    pub(crate) event_destinations:
        ::std::option::Option<::std::vec::Vec<crate::types::EventDestination>>,
    pub(crate) default_message_type: ::std::option::Option<crate::types::MessageType>,
    pub(crate) default_sender_id: ::std::option::Option<::std::string::String>,
    pub(crate) created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ConfigurationSetInformationBuilder {
    /// <p>The Resource Name (ARN) of the ConfigurationSet.</p>
    pub fn configuration_set_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration_set_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Resource Name (ARN) of the ConfigurationSet.</p>
    pub fn set_configuration_set_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration_set_arn = input;
        self
    }
    /// <p>The name of the ConfigurationSet.</p>
    pub fn configuration_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration_set_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the ConfigurationSet.</p>
    pub fn set_configuration_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration_set_name = input;
        self
    }
    /// Appends an item to `event_destinations`.
    ///
    /// To override the contents of this collection use [`set_event_destinations`](Self::set_event_destinations).
    ///
    /// <p>An array of EventDestination objects that describe any events to log and where to log them.</p>
    pub fn event_destinations(mut self, input: crate::types::EventDestination) -> Self {
        let mut v = self.event_destinations.unwrap_or_default();
        v.push(input);
        self.event_destinations = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of EventDestination objects that describe any events to log and where to log them.</p>
    pub fn set_event_destinations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EventDestination>>,
    ) -> Self {
        self.event_destinations = input;
        self
    }
    /// <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive.</p>
    pub fn default_message_type(mut self, input: crate::types::MessageType) -> Self {
        self.default_message_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive.</p>
    pub fn set_default_message_type(
        mut self,
        input: ::std::option::Option<crate::types::MessageType>,
    ) -> Self {
        self.default_message_type = input;
        self
    }
    /// <p>The default sender ID used by the ConfigurationSet.</p>
    pub fn default_sender_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.default_sender_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The default sender ID used by the ConfigurationSet.</p>
    pub fn set_default_sender_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.default_sender_id = input;
        self
    }
    /// <p>The time when the ConfigurationSet was created, in <a href="https://www.epochconverter.com/">UNIX epoch time</a> format.</p>
    pub fn created_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the ConfigurationSet was created, in <a href="https://www.epochconverter.com/">UNIX epoch time</a> format.</p>
    pub fn set_created_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`ConfigurationSetInformation`](crate::types::ConfigurationSetInformation).
    pub fn build(self) -> crate::types::ConfigurationSetInformation {
        crate::types::ConfigurationSetInformation {
            configuration_set_arn: self.configuration_set_arn,
            configuration_set_name: self.configuration_set_name,
            event_destinations: self.event_destinations,
            default_message_type: self.default_message_type,
            default_sender_id: self.default_sender_id,
            created_timestamp: self.created_timestamp,
        }
    }
}
