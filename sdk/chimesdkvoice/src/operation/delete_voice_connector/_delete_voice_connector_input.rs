// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVoiceConnectorInput {
    /// <p>The Voice Connector ID.</p>
    #[doc(hidden)]
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceConnectorInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
}
impl DeleteVoiceConnectorInput {
    /// Creates a new builder-style object to manufacture [`DeleteVoiceConnectorInput`](crate::operation::delete_voice_connector::DeleteVoiceConnectorInput).
    pub fn builder(
    ) -> crate::operation::delete_voice_connector::builders::DeleteVoiceConnectorInputBuilder {
        crate::operation::delete_voice_connector::builders::DeleteVoiceConnectorInputBuilder::default()
    }
}

/// A builder for [`DeleteVoiceConnectorInput`](crate::operation::delete_voice_connector::DeleteVoiceConnectorInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteVoiceConnectorInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceConnectorInputBuilder {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn set_voice_connector_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVoiceConnectorInput`](crate::operation::delete_voice_connector::DeleteVoiceConnectorInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_voice_connector::DeleteVoiceConnectorInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_voice_connector::DeleteVoiceConnectorInput {
                voice_connector_id: self.voice_connector_id,
            },
        )
    }
}
