// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// ResumeCampaignRequest
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResumeCampaignInput {
    /// Identifier representing a Campaign
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl ResumeCampaignInput {
    /// Identifier representing a Campaign
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl ResumeCampaignInput {
    /// Creates a new builder-style object to manufacture [`ResumeCampaignInput`](crate::operation::resume_campaign::ResumeCampaignInput).
    pub fn builder() -> crate::operation::resume_campaign::builders::ResumeCampaignInputBuilder {
        crate::operation::resume_campaign::builders::ResumeCampaignInputBuilder::default()
    }
}

/// A builder for [`ResumeCampaignInput`](crate::operation::resume_campaign::ResumeCampaignInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResumeCampaignInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl ResumeCampaignInputBuilder {
    /// Identifier representing a Campaign
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier representing a Campaign
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`ResumeCampaignInput`](crate::operation::resume_campaign::ResumeCampaignInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::resume_campaign::ResumeCampaignInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::resume_campaign::ResumeCampaignInput {
            id: self.id,
        })
    }
}
