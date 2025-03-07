// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBrowserSettingsInput {
    /// <p>The ARN of the browser settings.</p>
    #[doc(hidden)]
    pub browser_settings_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteBrowserSettingsInput {
    /// <p>The ARN of the browser settings.</p>
    pub fn browser_settings_arn(&self) -> ::std::option::Option<&str> {
        self.browser_settings_arn.as_deref()
    }
}
impl DeleteBrowserSettingsInput {
    /// Creates a new builder-style object to manufacture [`DeleteBrowserSettingsInput`](crate::operation::delete_browser_settings::DeleteBrowserSettingsInput).
    pub fn builder(
    ) -> crate::operation::delete_browser_settings::builders::DeleteBrowserSettingsInputBuilder
    {
        crate::operation::delete_browser_settings::builders::DeleteBrowserSettingsInputBuilder::default()
    }
}

/// A builder for [`DeleteBrowserSettingsInput`](crate::operation::delete_browser_settings::DeleteBrowserSettingsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteBrowserSettingsInputBuilder {
    pub(crate) browser_settings_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteBrowserSettingsInputBuilder {
    /// <p>The ARN of the browser settings.</p>
    pub fn browser_settings_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.browser_settings_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the browser settings.</p>
    pub fn set_browser_settings_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.browser_settings_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteBrowserSettingsInput`](crate::operation::delete_browser_settings::DeleteBrowserSettingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_browser_settings::DeleteBrowserSettingsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_browser_settings::DeleteBrowserSettingsInput {
                browser_settings_arn: self.browser_settings_arn,
            },
        )
    }
}
