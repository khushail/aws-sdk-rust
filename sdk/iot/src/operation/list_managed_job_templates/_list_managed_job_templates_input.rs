// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListManagedJobTemplatesInput {
    /// <p>An optional parameter for template name. If specified, only the versions of the managed job templates that have the specified template name will be returned.</p>
    #[doc(hidden)]
    pub template_name: ::std::option::Option<::std::string::String>,
    /// <p>Maximum number of entries that can be returned.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token to retrieve the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListManagedJobTemplatesInput {
    /// <p>An optional parameter for template name. If specified, only the versions of the managed job templates that have the specified template name will be returned.</p>
    pub fn template_name(&self) -> ::std::option::Option<&str> {
        self.template_name.as_deref()
    }
    /// <p>Maximum number of entries that can be returned.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListManagedJobTemplatesInput {
    /// Creates a new builder-style object to manufacture [`ListManagedJobTemplatesInput`](crate::operation::list_managed_job_templates::ListManagedJobTemplatesInput).
    pub fn builder(
    ) -> crate::operation::list_managed_job_templates::builders::ListManagedJobTemplatesInputBuilder
    {
        crate::operation::list_managed_job_templates::builders::ListManagedJobTemplatesInputBuilder::default()
    }
}

/// A builder for [`ListManagedJobTemplatesInput`](crate::operation::list_managed_job_templates::ListManagedJobTemplatesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListManagedJobTemplatesInputBuilder {
    pub(crate) template_name: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListManagedJobTemplatesInputBuilder {
    /// <p>An optional parameter for template name. If specified, only the versions of the managed job templates that have the specified template name will be returned.</p>
    pub fn template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional parameter for template name. If specified, only the versions of the managed job templates that have the specified template name will be returned.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.template_name = input;
        self
    }
    /// <p>Maximum number of entries that can be returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of entries that can be returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListManagedJobTemplatesInput`](crate::operation::list_managed_job_templates::ListManagedJobTemplatesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_managed_job_templates::ListManagedJobTemplatesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_managed_job_templates::ListManagedJobTemplatesInput {
                template_name: self.template_name,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
