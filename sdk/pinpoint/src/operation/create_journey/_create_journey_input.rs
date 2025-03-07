// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateJourneyInput {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the configuration and other settings for a journey.</p>
    #[doc(hidden)]
    pub write_journey_request: ::std::option::Option<crate::types::WriteJourneyRequest>,
}
impl CreateJourneyInput {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>Specifies the configuration and other settings for a journey.</p>
    pub fn write_journey_request(
        &self,
    ) -> ::std::option::Option<&crate::types::WriteJourneyRequest> {
        self.write_journey_request.as_ref()
    }
}
impl CreateJourneyInput {
    /// Creates a new builder-style object to manufacture [`CreateJourneyInput`](crate::operation::create_journey::CreateJourneyInput).
    pub fn builder() -> crate::operation::create_journey::builders::CreateJourneyInputBuilder {
        crate::operation::create_journey::builders::CreateJourneyInputBuilder::default()
    }
}

/// A builder for [`CreateJourneyInput`](crate::operation::create_journey::CreateJourneyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateJourneyInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) write_journey_request: ::std::option::Option<crate::types::WriteJourneyRequest>,
}
impl CreateJourneyInputBuilder {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>Specifies the configuration and other settings for a journey.</p>
    pub fn write_journey_request(mut self, input: crate::types::WriteJourneyRequest) -> Self {
        self.write_journey_request = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the configuration and other settings for a journey.</p>
    pub fn set_write_journey_request(
        mut self,
        input: ::std::option::Option<crate::types::WriteJourneyRequest>,
    ) -> Self {
        self.write_journey_request = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateJourneyInput`](crate::operation::create_journey::CreateJourneyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_journey::CreateJourneyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_journey::CreateJourneyInput {
            application_id: self.application_id,
            write_journey_request: self.write_journey_request,
        })
    }
}
