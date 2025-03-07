// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides information about the target engine for the specified source database.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecommendationData {
    /// <p>The recommendation of a target Amazon RDS database engine.</p>
    #[doc(hidden)]
    pub rds_engine: ::std::option::Option<crate::types::RdsRecommendation>,
}
impl RecommendationData {
    /// <p>The recommendation of a target Amazon RDS database engine.</p>
    pub fn rds_engine(&self) -> ::std::option::Option<&crate::types::RdsRecommendation> {
        self.rds_engine.as_ref()
    }
}
impl RecommendationData {
    /// Creates a new builder-style object to manufacture [`RecommendationData`](crate::types::RecommendationData).
    pub fn builder() -> crate::types::builders::RecommendationDataBuilder {
        crate::types::builders::RecommendationDataBuilder::default()
    }
}

/// A builder for [`RecommendationData`](crate::types::RecommendationData).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RecommendationDataBuilder {
    pub(crate) rds_engine: ::std::option::Option<crate::types::RdsRecommendation>,
}
impl RecommendationDataBuilder {
    /// <p>The recommendation of a target Amazon RDS database engine.</p>
    pub fn rds_engine(mut self, input: crate::types::RdsRecommendation) -> Self {
        self.rds_engine = ::std::option::Option::Some(input);
        self
    }
    /// <p>The recommendation of a target Amazon RDS database engine.</p>
    pub fn set_rds_engine(
        mut self,
        input: ::std::option::Option<crate::types::RdsRecommendation>,
    ) -> Self {
        self.rds_engine = input;
        self
    }
    /// Consumes the builder and constructs a [`RecommendationData`](crate::types::RecommendationData).
    pub fn build(self) -> crate::types::RecommendationData {
        crate::types::RecommendationData {
            rds_engine: self.rds_engine,
        }
    }
}
