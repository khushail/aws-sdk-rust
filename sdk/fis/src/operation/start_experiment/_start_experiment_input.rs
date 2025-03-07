// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartExperimentInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the experiment template.</p>
    #[doc(hidden)]
    pub experiment_template_id: ::std::option::Option<::std::string::String>,
    /// <p>The tags to apply to the experiment.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl StartExperimentInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The ID of the experiment template.</p>
    pub fn experiment_template_id(&self) -> ::std::option::Option<&str> {
        self.experiment_template_id.as_deref()
    }
    /// <p>The tags to apply to the experiment.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl StartExperimentInput {
    /// Creates a new builder-style object to manufacture [`StartExperimentInput`](crate::operation::start_experiment::StartExperimentInput).
    pub fn builder() -> crate::operation::start_experiment::builders::StartExperimentInputBuilder {
        crate::operation::start_experiment::builders::StartExperimentInputBuilder::default()
    }
}

/// A builder for [`StartExperimentInput`](crate::operation::start_experiment::StartExperimentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartExperimentInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) experiment_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl StartExperimentInputBuilder {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The ID of the experiment template.</p>
    pub fn experiment_template_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.experiment_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the experiment template.</p>
    pub fn set_experiment_template_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.experiment_template_id = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the experiment.</p>
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
    /// <p>The tags to apply to the experiment.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`StartExperimentInput`](crate::operation::start_experiment::StartExperimentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_experiment::StartExperimentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_experiment::StartExperimentInput {
            client_token: self.client_token,
            experiment_template_id: self.experiment_template_id,
            tags: self.tags,
        })
    }
}
