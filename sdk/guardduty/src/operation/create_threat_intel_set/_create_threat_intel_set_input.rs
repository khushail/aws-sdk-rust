// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateThreatIntelSetInput {
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create a threatIntelSet for.</p>
    #[doc(hidden)]
    pub detector_id: ::std::option::Option<::std::string::String>,
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    #[doc(hidden)]
    pub format: ::std::option::Option<crate::types::ThreatIntelSetFormat>,
    /// <p>The URI of the file that contains the ThreatIntelSet. </p>
    #[doc(hidden)]
    pub location: ::std::option::Option<::std::string::String>,
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    #[doc(hidden)]
    pub activate: ::std::option::Option<bool>,
    /// <p>The idempotency token for the create request.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The tags to be added to a new threat list resource.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateThreatIntelSetInput {
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create a threatIntelSet for.</p>
    pub fn detector_id(&self) -> ::std::option::Option<&str> {
        self.detector_id.as_deref()
    }
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    pub fn format(&self) -> ::std::option::Option<&crate::types::ThreatIntelSetFormat> {
        self.format.as_ref()
    }
    /// <p>The URI of the file that contains the ThreatIntelSet. </p>
    pub fn location(&self) -> ::std::option::Option<&str> {
        self.location.as_deref()
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    pub fn activate(&self) -> ::std::option::Option<bool> {
        self.activate
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The tags to be added to a new threat list resource.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl CreateThreatIntelSetInput {
    /// Creates a new builder-style object to manufacture [`CreateThreatIntelSetInput`](crate::operation::create_threat_intel_set::CreateThreatIntelSetInput).
    pub fn builder(
    ) -> crate::operation::create_threat_intel_set::builders::CreateThreatIntelSetInputBuilder {
        crate::operation::create_threat_intel_set::builders::CreateThreatIntelSetInputBuilder::default()
    }
}

/// A builder for [`CreateThreatIntelSetInput`](crate::operation::create_threat_intel_set::CreateThreatIntelSetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateThreatIntelSetInputBuilder {
    pub(crate) detector_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) format: ::std::option::Option<crate::types::ThreatIntelSetFormat>,
    pub(crate) location: ::std::option::Option<::std::string::String>,
    pub(crate) activate: ::std::option::Option<bool>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateThreatIntelSetInputBuilder {
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create a threatIntelSet for.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.detector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create a threatIntelSet for.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.detector_id = input;
        self
    }
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    pub fn format(mut self, input: crate::types::ThreatIntelSetFormat) -> Self {
        self.format = ::std::option::Option::Some(input);
        self
    }
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    pub fn set_format(
        mut self,
        input: ::std::option::Option<crate::types::ThreatIntelSetFormat>,
    ) -> Self {
        self.format = input;
        self
    }
    /// <p>The URI of the file that contains the ThreatIntelSet. </p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URI of the file that contains the ThreatIntelSet. </p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    pub fn activate(mut self, input: bool) -> Self {
        self.activate = ::std::option::Option::Some(input);
        self
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    pub fn set_activate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.activate = input;
        self
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to be added to a new threat list resource.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags to be added to a new threat list resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateThreatIntelSetInput`](crate::operation::create_threat_intel_set::CreateThreatIntelSetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_threat_intel_set::CreateThreatIntelSetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_threat_intel_set::CreateThreatIntelSetInput {
                detector_id: self.detector_id,
                name: self.name,
                format: self.format,
                location: self.location,
                activate: self.activate,
                client_token: self.client_token,
                tags: self.tags,
            },
        )
    }
}
