// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartGeneratedCodeJobInput {
    /// <p>The name of the game.</p>
    #[doc(hidden)]
    pub game_name: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the snapshot for which to generate code.</p>
    #[doc(hidden)]
    pub snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>Properties of the generator to use for the job.</p>
    #[doc(hidden)]
    pub generator: ::std::option::Option<crate::types::Generator>,
}
impl StartGeneratedCodeJobInput {
    /// <p>The name of the game.</p>
    pub fn game_name(&self) -> ::std::option::Option<&str> {
        self.game_name.as_deref()
    }
    /// <p>The identifier of the snapshot for which to generate code.</p>
    pub fn snapshot_id(&self) -> ::std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
    /// <p>Properties of the generator to use for the job.</p>
    pub fn generator(&self) -> ::std::option::Option<&crate::types::Generator> {
        self.generator.as_ref()
    }
}
impl StartGeneratedCodeJobInput {
    /// Creates a new builder-style object to manufacture [`StartGeneratedCodeJobInput`](crate::operation::start_generated_code_job::StartGeneratedCodeJobInput).
    pub fn builder(
    ) -> crate::operation::start_generated_code_job::builders::StartGeneratedCodeJobInputBuilder
    {
        crate::operation::start_generated_code_job::builders::StartGeneratedCodeJobInputBuilder::default()
    }
}

/// A builder for [`StartGeneratedCodeJobInput`](crate::operation::start_generated_code_job::StartGeneratedCodeJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartGeneratedCodeJobInputBuilder {
    pub(crate) game_name: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) generator: ::std::option::Option<crate::types::Generator>,
}
impl StartGeneratedCodeJobInputBuilder {
    /// <p>The name of the game.</p>
    pub fn game_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.game_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the game.</p>
    pub fn set_game_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.game_name = input;
        self
    }
    /// <p>The identifier of the snapshot for which to generate code.</p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the snapshot for which to generate code.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_id = input;
        self
    }
    /// <p>Properties of the generator to use for the job.</p>
    pub fn generator(mut self, input: crate::types::Generator) -> Self {
        self.generator = ::std::option::Option::Some(input);
        self
    }
    /// <p>Properties of the generator to use for the job.</p>
    pub fn set_generator(mut self, input: ::std::option::Option<crate::types::Generator>) -> Self {
        self.generator = input;
        self
    }
    /// Consumes the builder and constructs a [`StartGeneratedCodeJobInput`](crate::operation::start_generated_code_job::StartGeneratedCodeJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_generated_code_job::StartGeneratedCodeJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::start_generated_code_job::StartGeneratedCodeJobInput {
                game_name: self.game_name,
                snapshot_id: self.snapshot_id,
                generator: self.generator,
            },
        )
    }
}
