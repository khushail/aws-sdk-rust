// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListRegexPatternSetsInput {
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexPatternSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RegexPatternSet</code> objects. For the second and subsequent <code>ListRegexPatternSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexPatternSet</code> objects.</p>
    #[doc(hidden)]
    pub next_marker: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the number of <code>RegexPatternSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexPatternSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexPatternSet</code> objects.</p>
    #[doc(hidden)]
    pub limit: i32,
}
impl ListRegexPatternSetsInput {
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexPatternSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RegexPatternSet</code> objects. For the second and subsequent <code>ListRegexPatternSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexPatternSet</code> objects.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>Specifies the number of <code>RegexPatternSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexPatternSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexPatternSet</code> objects.</p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
}
impl ListRegexPatternSetsInput {
    /// Creates a new builder-style object to manufacture [`ListRegexPatternSetsInput`](crate::operation::list_regex_pattern_sets::ListRegexPatternSetsInput).
    pub fn builder(
    ) -> crate::operation::list_regex_pattern_sets::builders::ListRegexPatternSetsInputBuilder {
        crate::operation::list_regex_pattern_sets::builders::ListRegexPatternSetsInputBuilder::default()
    }
}

/// A builder for [`ListRegexPatternSetsInput`](crate::operation::list_regex_pattern_sets::ListRegexPatternSetsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListRegexPatternSetsInputBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
    pub(crate) limit: ::std::option::Option<i32>,
}
impl ListRegexPatternSetsInputBuilder {
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexPatternSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RegexPatternSet</code> objects. For the second and subsequent <code>ListRegexPatternSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexPatternSet</code> objects.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexPatternSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RegexPatternSet</code> objects. For the second and subsequent <code>ListRegexPatternSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexPatternSet</code> objects.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>Specifies the number of <code>RegexPatternSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexPatternSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexPatternSet</code> objects.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the number of <code>RegexPatternSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexPatternSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexPatternSet</code> objects.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// Consumes the builder and constructs a [`ListRegexPatternSetsInput`](crate::operation::list_regex_pattern_sets::ListRegexPatternSetsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_regex_pattern_sets::ListRegexPatternSetsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_regex_pattern_sets::ListRegexPatternSetsInput {
                next_marker: self.next_marker,
                limit: self.limit.unwrap_or_default(),
            },
        )
    }
}
