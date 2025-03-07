// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBuiltinSlotTypesInput {
    /// <p>A list of locales that the slot type supports.</p>
    #[doc(hidden)]
    pub locale: ::std::option::Option<crate::types::Locale>,
    /// <p>Substring to match in built-in slot type signatures. A slot type will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    #[doc(hidden)]
    pub signature_contains: ::std::option::Option<::std::string::String>,
    /// <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of slot types, specify the pagination token in the next request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of slot types to return in the response. The default is 10.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl GetBuiltinSlotTypesInput {
    /// <p>A list of locales that the slot type supports.</p>
    pub fn locale(&self) -> ::std::option::Option<&crate::types::Locale> {
        self.locale.as_ref()
    }
    /// <p>Substring to match in built-in slot type signatures. A slot type will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    pub fn signature_contains(&self) -> ::std::option::Option<&str> {
        self.signature_contains.as_deref()
    }
    /// <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of slot types, specify the pagination token in the next request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of slot types to return in the response. The default is 10.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl GetBuiltinSlotTypesInput {
    /// Creates a new builder-style object to manufacture [`GetBuiltinSlotTypesInput`](crate::operation::get_builtin_slot_types::GetBuiltinSlotTypesInput).
    pub fn builder(
    ) -> crate::operation::get_builtin_slot_types::builders::GetBuiltinSlotTypesInputBuilder {
        crate::operation::get_builtin_slot_types::builders::GetBuiltinSlotTypesInputBuilder::default(
        )
    }
}

/// A builder for [`GetBuiltinSlotTypesInput`](crate::operation::get_builtin_slot_types::GetBuiltinSlotTypesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBuiltinSlotTypesInputBuilder {
    pub(crate) locale: ::std::option::Option<crate::types::Locale>,
    pub(crate) signature_contains: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl GetBuiltinSlotTypesInputBuilder {
    /// <p>A list of locales that the slot type supports.</p>
    pub fn locale(mut self, input: crate::types::Locale) -> Self {
        self.locale = ::std::option::Option::Some(input);
        self
    }
    /// <p>A list of locales that the slot type supports.</p>
    pub fn set_locale(mut self, input: ::std::option::Option<crate::types::Locale>) -> Self {
        self.locale = input;
        self
    }
    /// <p>Substring to match in built-in slot type signatures. A slot type will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    pub fn signature_contains(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.signature_contains = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Substring to match in built-in slot type signatures. A slot type will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    pub fn set_signature_contains(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.signature_contains = input;
        self
    }
    /// <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of slot types, specify the pagination token in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of slot types, specify the pagination token in the next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of slot types to return in the response. The default is 10.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of slot types to return in the response. The default is 10.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`GetBuiltinSlotTypesInput`](crate::operation::get_builtin_slot_types::GetBuiltinSlotTypesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_builtin_slot_types::GetBuiltinSlotTypesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_builtin_slot_types::GetBuiltinSlotTypesInput {
                locale: self.locale,
                signature_contains: self.signature_contains,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
