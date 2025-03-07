// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides summary information about a built-in intent for the <a href="https://docs.aws.amazon.com/lexv2/latest/APIReference/API_ListBuiltInIntents.html"> ListBuiltInIntents </a> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BuiltInIntentSummary {
    /// <p>The signature of the built-in intent. Use this to specify the parent intent of a derived intent.</p>
    #[doc(hidden)]
    pub intent_signature: ::std::option::Option<::std::string::String>,
    /// <p>The description of the intent.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl BuiltInIntentSummary {
    /// <p>The signature of the built-in intent. Use this to specify the parent intent of a derived intent.</p>
    pub fn intent_signature(&self) -> ::std::option::Option<&str> {
        self.intent_signature.as_deref()
    }
    /// <p>The description of the intent.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl BuiltInIntentSummary {
    /// Creates a new builder-style object to manufacture [`BuiltInIntentSummary`](crate::types::BuiltInIntentSummary).
    pub fn builder() -> crate::types::builders::BuiltInIntentSummaryBuilder {
        crate::types::builders::BuiltInIntentSummaryBuilder::default()
    }
}

/// A builder for [`BuiltInIntentSummary`](crate::types::BuiltInIntentSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BuiltInIntentSummaryBuilder {
    pub(crate) intent_signature: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl BuiltInIntentSummaryBuilder {
    /// <p>The signature of the built-in intent. Use this to specify the parent intent of a derived intent.</p>
    pub fn intent_signature(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.intent_signature = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The signature of the built-in intent. Use this to specify the parent intent of a derived intent.</p>
    pub fn set_intent_signature(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.intent_signature = input;
        self
    }
    /// <p>The description of the intent.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the intent.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`BuiltInIntentSummary`](crate::types::BuiltInIntentSummary).
    pub fn build(self) -> crate::types::BuiltInIntentSummary {
        crate::types::BuiltInIntentSummary {
            intent_signature: self.intent_signature,
            description: self.description,
        }
    }
}
