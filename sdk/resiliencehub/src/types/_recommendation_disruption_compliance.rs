// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines a disruption compliance recommendation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecommendationDisruptionCompliance {
    /// <p>The expected compliance status after applying the recommended configuration change.</p>
    #[doc(hidden)]
    pub expected_compliance_status: ::std::option::Option<crate::types::ComplianceStatus>,
    /// <p>The expected RTO after applying the recommended configuration change.</p>
    #[doc(hidden)]
    pub expected_rto_in_secs: i32,
    /// <p>The expected Recovery Time Objective (RTO) description after applying the recommended configuration change.</p>
    #[doc(hidden)]
    pub expected_rto_description: ::std::option::Option<::std::string::String>,
    /// <p>The expected RPO after applying the recommended configuration change.</p>
    #[doc(hidden)]
    pub expected_rpo_in_secs: i32,
    /// <p>The expected Recovery Point Objective (RPO) description after applying the recommended configuration change.</p>
    #[doc(hidden)]
    pub expected_rpo_description: ::std::option::Option<::std::string::String>,
}
impl RecommendationDisruptionCompliance {
    /// <p>The expected compliance status after applying the recommended configuration change.</p>
    pub fn expected_compliance_status(
        &self,
    ) -> ::std::option::Option<&crate::types::ComplianceStatus> {
        self.expected_compliance_status.as_ref()
    }
    /// <p>The expected RTO after applying the recommended configuration change.</p>
    pub fn expected_rto_in_secs(&self) -> i32 {
        self.expected_rto_in_secs
    }
    /// <p>The expected Recovery Time Objective (RTO) description after applying the recommended configuration change.</p>
    pub fn expected_rto_description(&self) -> ::std::option::Option<&str> {
        self.expected_rto_description.as_deref()
    }
    /// <p>The expected RPO after applying the recommended configuration change.</p>
    pub fn expected_rpo_in_secs(&self) -> i32 {
        self.expected_rpo_in_secs
    }
    /// <p>The expected Recovery Point Objective (RPO) description after applying the recommended configuration change.</p>
    pub fn expected_rpo_description(&self) -> ::std::option::Option<&str> {
        self.expected_rpo_description.as_deref()
    }
}
impl RecommendationDisruptionCompliance {
    /// Creates a new builder-style object to manufacture [`RecommendationDisruptionCompliance`](crate::types::RecommendationDisruptionCompliance).
    pub fn builder() -> crate::types::builders::RecommendationDisruptionComplianceBuilder {
        crate::types::builders::RecommendationDisruptionComplianceBuilder::default()
    }
}

/// A builder for [`RecommendationDisruptionCompliance`](crate::types::RecommendationDisruptionCompliance).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RecommendationDisruptionComplianceBuilder {
    pub(crate) expected_compliance_status: ::std::option::Option<crate::types::ComplianceStatus>,
    pub(crate) expected_rto_in_secs: ::std::option::Option<i32>,
    pub(crate) expected_rto_description: ::std::option::Option<::std::string::String>,
    pub(crate) expected_rpo_in_secs: ::std::option::Option<i32>,
    pub(crate) expected_rpo_description: ::std::option::Option<::std::string::String>,
}
impl RecommendationDisruptionComplianceBuilder {
    /// <p>The expected compliance status after applying the recommended configuration change.</p>
    pub fn expected_compliance_status(mut self, input: crate::types::ComplianceStatus) -> Self {
        self.expected_compliance_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The expected compliance status after applying the recommended configuration change.</p>
    pub fn set_expected_compliance_status(
        mut self,
        input: ::std::option::Option<crate::types::ComplianceStatus>,
    ) -> Self {
        self.expected_compliance_status = input;
        self
    }
    /// <p>The expected RTO after applying the recommended configuration change.</p>
    pub fn expected_rto_in_secs(mut self, input: i32) -> Self {
        self.expected_rto_in_secs = ::std::option::Option::Some(input);
        self
    }
    /// <p>The expected RTO after applying the recommended configuration change.</p>
    pub fn set_expected_rto_in_secs(mut self, input: ::std::option::Option<i32>) -> Self {
        self.expected_rto_in_secs = input;
        self
    }
    /// <p>The expected Recovery Time Objective (RTO) description after applying the recommended configuration change.</p>
    pub fn expected_rto_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.expected_rto_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The expected Recovery Time Objective (RTO) description after applying the recommended configuration change.</p>
    pub fn set_expected_rto_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.expected_rto_description = input;
        self
    }
    /// <p>The expected RPO after applying the recommended configuration change.</p>
    pub fn expected_rpo_in_secs(mut self, input: i32) -> Self {
        self.expected_rpo_in_secs = ::std::option::Option::Some(input);
        self
    }
    /// <p>The expected RPO after applying the recommended configuration change.</p>
    pub fn set_expected_rpo_in_secs(mut self, input: ::std::option::Option<i32>) -> Self {
        self.expected_rpo_in_secs = input;
        self
    }
    /// <p>The expected Recovery Point Objective (RPO) description after applying the recommended configuration change.</p>
    pub fn expected_rpo_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.expected_rpo_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The expected Recovery Point Objective (RPO) description after applying the recommended configuration change.</p>
    pub fn set_expected_rpo_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.expected_rpo_description = input;
        self
    }
    /// Consumes the builder and constructs a [`RecommendationDisruptionCompliance`](crate::types::RecommendationDisruptionCompliance).
    pub fn build(self) -> crate::types::RecommendationDisruptionCompliance {
        crate::types::RecommendationDisruptionCompliance {
            expected_compliance_status: self.expected_compliance_status,
            expected_rto_in_secs: self.expected_rto_in_secs.unwrap_or_default(),
            expected_rto_description: self.expected_rto_description,
            expected_rpo_in_secs: self.expected_rpo_in_secs.unwrap_or_default(),
            expected_rpo_description: self.expected_rpo_description,
        }
    }
}
