// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A metric for a particular pillar in a lens.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PillarMetric {
    /// <p>The ID used to identify a pillar, for example, <code>security</code>.</p>
    /// <p>A pillar is identified by its <code>PillarReviewSummary$PillarId</code>.</p>
    #[doc(hidden)]
    pub pillar_id: ::std::option::Option<::std::string::String>,
    /// <p>A map from risk names to the count of how many questions have that rating.</p>
    #[doc(hidden)]
    pub risk_counts: ::std::option::Option<::std::collections::HashMap<crate::types::Risk, i32>>,
    /// <p>The questions that have been identified as risks in the pillar.</p>
    #[doc(hidden)]
    pub questions: ::std::option::Option<::std::vec::Vec<crate::types::QuestionMetric>>,
}
impl PillarMetric {
    /// <p>The ID used to identify a pillar, for example, <code>security</code>.</p>
    /// <p>A pillar is identified by its <code>PillarReviewSummary$PillarId</code>.</p>
    pub fn pillar_id(&self) -> ::std::option::Option<&str> {
        self.pillar_id.as_deref()
    }
    /// <p>A map from risk names to the count of how many questions have that rating.</p>
    pub fn risk_counts(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<crate::types::Risk, i32>> {
        self.risk_counts.as_ref()
    }
    /// <p>The questions that have been identified as risks in the pillar.</p>
    pub fn questions(&self) -> ::std::option::Option<&[crate::types::QuestionMetric]> {
        self.questions.as_deref()
    }
}
impl PillarMetric {
    /// Creates a new builder-style object to manufacture [`PillarMetric`](crate::types::PillarMetric).
    pub fn builder() -> crate::types::builders::PillarMetricBuilder {
        crate::types::builders::PillarMetricBuilder::default()
    }
}

/// A builder for [`PillarMetric`](crate::types::PillarMetric).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PillarMetricBuilder {
    pub(crate) pillar_id: ::std::option::Option<::std::string::String>,
    pub(crate) risk_counts:
        ::std::option::Option<::std::collections::HashMap<crate::types::Risk, i32>>,
    pub(crate) questions: ::std::option::Option<::std::vec::Vec<crate::types::QuestionMetric>>,
}
impl PillarMetricBuilder {
    /// <p>The ID used to identify a pillar, for example, <code>security</code>.</p>
    /// <p>A pillar is identified by its <code>PillarReviewSummary$PillarId</code>.</p>
    pub fn pillar_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pillar_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID used to identify a pillar, for example, <code>security</code>.</p>
    /// <p>A pillar is identified by its <code>PillarReviewSummary$PillarId</code>.</p>
    pub fn set_pillar_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pillar_id = input;
        self
    }
    /// Adds a key-value pair to `risk_counts`.
    ///
    /// To override the contents of this collection use [`set_risk_counts`](Self::set_risk_counts).
    ///
    /// <p>A map from risk names to the count of how many questions have that rating.</p>
    pub fn risk_counts(mut self, k: crate::types::Risk, v: i32) -> Self {
        let mut hash_map = self.risk_counts.unwrap_or_default();
        hash_map.insert(k, v);
        self.risk_counts = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map from risk names to the count of how many questions have that rating.</p>
    pub fn set_risk_counts(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::Risk, i32>>,
    ) -> Self {
        self.risk_counts = input;
        self
    }
    /// Appends an item to `questions`.
    ///
    /// To override the contents of this collection use [`set_questions`](Self::set_questions).
    ///
    /// <p>The questions that have been identified as risks in the pillar.</p>
    pub fn questions(mut self, input: crate::types::QuestionMetric) -> Self {
        let mut v = self.questions.unwrap_or_default();
        v.push(input);
        self.questions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The questions that have been identified as risks in the pillar.</p>
    pub fn set_questions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::QuestionMetric>>,
    ) -> Self {
        self.questions = input;
        self
    }
    /// Consumes the builder and constructs a [`PillarMetric`](crate::types::PillarMetric).
    pub fn build(self) -> crate::types::PillarMetric {
        crate::types::PillarMetric {
            pillar_id: self.pillar_id,
            risk_counts: self.risk_counts,
            questions: self.questions,
        }
    }
}
