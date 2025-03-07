// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A summary of a field-level encryption item.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FieldLevelEncryptionSummary {
    /// <p>The unique ID of a field-level encryption item.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The last time that the summary of field-level encryption items was modified.</p>
    #[doc(hidden)]
    pub last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>An optional comment about the field-level encryption item. The comment cannot be longer than 128 characters.</p>
    #[doc(hidden)]
    pub comment: ::std::option::Option<::std::string::String>,
    /// <p>A summary of a query argument-profile mapping.</p>
    #[doc(hidden)]
    pub query_arg_profile_config: ::std::option::Option<crate::types::QueryArgProfileConfig>,
    /// <p>A summary of a content type-profile mapping.</p>
    #[doc(hidden)]
    pub content_type_profile_config: ::std::option::Option<crate::types::ContentTypeProfileConfig>,
}
impl FieldLevelEncryptionSummary {
    /// <p>The unique ID of a field-level encryption item.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The last time that the summary of field-level encryption items was modified.</p>
    pub fn last_modified_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_time.as_ref()
    }
    /// <p>An optional comment about the field-level encryption item. The comment cannot be longer than 128 characters.</p>
    pub fn comment(&self) -> ::std::option::Option<&str> {
        self.comment.as_deref()
    }
    /// <p>A summary of a query argument-profile mapping.</p>
    pub fn query_arg_profile_config(
        &self,
    ) -> ::std::option::Option<&crate::types::QueryArgProfileConfig> {
        self.query_arg_profile_config.as_ref()
    }
    /// <p>A summary of a content type-profile mapping.</p>
    pub fn content_type_profile_config(
        &self,
    ) -> ::std::option::Option<&crate::types::ContentTypeProfileConfig> {
        self.content_type_profile_config.as_ref()
    }
}
impl FieldLevelEncryptionSummary {
    /// Creates a new builder-style object to manufacture [`FieldLevelEncryptionSummary`](crate::types::FieldLevelEncryptionSummary).
    pub fn builder() -> crate::types::builders::FieldLevelEncryptionSummaryBuilder {
        crate::types::builders::FieldLevelEncryptionSummaryBuilder::default()
    }
}

/// A builder for [`FieldLevelEncryptionSummary`](crate::types::FieldLevelEncryptionSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FieldLevelEncryptionSummaryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) comment: ::std::option::Option<::std::string::String>,
    pub(crate) query_arg_profile_config: ::std::option::Option<crate::types::QueryArgProfileConfig>,
    pub(crate) content_type_profile_config:
        ::std::option::Option<crate::types::ContentTypeProfileConfig>,
}
impl FieldLevelEncryptionSummaryBuilder {
    /// <p>The unique ID of a field-level encryption item.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID of a field-level encryption item.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The last time that the summary of field-level encryption items was modified.</p>
    pub fn last_modified_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last time that the summary of field-level encryption items was modified.</p>
    pub fn set_last_modified_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_time = input;
        self
    }
    /// <p>An optional comment about the field-level encryption item. The comment cannot be longer than 128 characters.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional comment about the field-level encryption item. The comment cannot be longer than 128 characters.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comment = input;
        self
    }
    /// <p>A summary of a query argument-profile mapping.</p>
    pub fn query_arg_profile_config(mut self, input: crate::types::QueryArgProfileConfig) -> Self {
        self.query_arg_profile_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>A summary of a query argument-profile mapping.</p>
    pub fn set_query_arg_profile_config(
        mut self,
        input: ::std::option::Option<crate::types::QueryArgProfileConfig>,
    ) -> Self {
        self.query_arg_profile_config = input;
        self
    }
    /// <p>A summary of a content type-profile mapping.</p>
    pub fn content_type_profile_config(
        mut self,
        input: crate::types::ContentTypeProfileConfig,
    ) -> Self {
        self.content_type_profile_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>A summary of a content type-profile mapping.</p>
    pub fn set_content_type_profile_config(
        mut self,
        input: ::std::option::Option<crate::types::ContentTypeProfileConfig>,
    ) -> Self {
        self.content_type_profile_config = input;
        self
    }
    /// Consumes the builder and constructs a [`FieldLevelEncryptionSummary`](crate::types::FieldLevelEncryptionSummary).
    pub fn build(self) -> crate::types::FieldLevelEncryptionSummary {
        crate::types::FieldLevelEncryptionSummary {
            id: self.id,
            last_modified_time: self.last_modified_time,
            comment: self.comment,
            query_arg_profile_config: self.query_arg_profile_config,
            content_type_profile_config: self.content_type_profile_config,
        }
    }
}
