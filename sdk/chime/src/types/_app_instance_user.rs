// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of an <code>AppInstanceUser</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AppInstanceUser {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    #[doc(hidden)]
    pub app_instance_user_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the <code>AppInstanceUser</code>.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The time at which the <code>AppInstanceUser</code> was created.</p>
    #[doc(hidden)]
    pub created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
    #[doc(hidden)]
    pub metadata: ::std::option::Option<::std::string::String>,
    /// <p>The time at which the <code>AppInstanceUser</code> was last updated.</p>
    #[doc(hidden)]
    pub last_updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl AppInstanceUser {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn app_instance_user_arn(&self) -> ::std::option::Option<&str> {
        self.app_instance_user_arn.as_deref()
    }
    /// <p>The name of the <code>AppInstanceUser</code>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The time at which the <code>AppInstanceUser</code> was created.</p>
    pub fn created_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_timestamp.as_ref()
    }
    /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
    pub fn metadata(&self) -> ::std::option::Option<&str> {
        self.metadata.as_deref()
    }
    /// <p>The time at which the <code>AppInstanceUser</code> was last updated.</p>
    pub fn last_updated_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_timestamp.as_ref()
    }
}
impl ::std::fmt::Debug for AppInstanceUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceUser");
        formatter.field("app_instance_user_arn", &self.app_instance_user_arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.field("last_updated_timestamp", &self.last_updated_timestamp);
        formatter.finish()
    }
}
impl AppInstanceUser {
    /// Creates a new builder-style object to manufacture [`AppInstanceUser`](crate::types::AppInstanceUser).
    pub fn builder() -> crate::types::builders::AppInstanceUserBuilder {
        crate::types::builders::AppInstanceUserBuilder::default()
    }
}

/// A builder for [`AppInstanceUser`](crate::types::AppInstanceUser).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AppInstanceUserBuilder {
    pub(crate) app_instance_user_arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) metadata: ::std::option::Option<::std::string::String>,
    pub(crate) last_updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl AppInstanceUserBuilder {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn app_instance_user_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_instance_user_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn set_app_instance_user_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_instance_user_arn = input;
        self
    }
    /// <p>The name of the <code>AppInstanceUser</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the <code>AppInstanceUser</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The time at which the <code>AppInstanceUser</code> was created.</p>
    pub fn created_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the <code>AppInstanceUser</code> was created.</p>
    pub fn set_created_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_timestamp = input;
        self
    }
    /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
    pub fn metadata(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metadata = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
    pub fn set_metadata(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metadata = input;
        self
    }
    /// <p>The time at which the <code>AppInstanceUser</code> was last updated.</p>
    pub fn last_updated_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the <code>AppInstanceUser</code> was last updated.</p>
    pub fn set_last_updated_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`AppInstanceUser`](crate::types::AppInstanceUser).
    pub fn build(self) -> crate::types::AppInstanceUser {
        crate::types::AppInstanceUser {
            app_instance_user_arn: self.app_instance_user_arn,
            name: self.name,
            created_timestamp: self.created_timestamp,
            metadata: self.metadata,
            last_updated_timestamp: self.last_updated_timestamp,
        }
    }
}
impl ::std::fmt::Debug for AppInstanceUserBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceUserBuilder");
        formatter.field("app_instance_user_arn", &self.app_instance_user_arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.field("last_updated_timestamp", &self.last_updated_timestamp);
        formatter.finish()
    }
}
