// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Information about a reactive insight. This object is returned by <code>DescribeInsight.</code> </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReactiveInsightSummary {
    /// <p> The ID of a reactive summary. </p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p> The name of a reactive insight. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The severity of the insight. For more information, see <a href="https://docs.aws.amazon.com/devops-guru/latest/userguide/working-with-insights.html#understanding-insights-severities">Understanding insight severities</a> in the <i>Amazon DevOps Guru User Guide</i>.</p>
    #[doc(hidden)]
    pub severity: ::std::option::Option<crate::types::InsightSeverity>,
    /// <p> The status of a reactive insight. </p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::InsightStatus>,
    /// <p> A time ranged that specifies when the observed behavior in an insight started and ended. </p>
    #[doc(hidden)]
    pub insight_time_range: ::std::option::Option<crate::types::InsightTimeRange>,
    /// <p> A collection of Amazon Web Services resources supported by DevOps Guru. The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and Amazon Web Services resources that contain the same Amazon Web Services tag. DevOps Guru can be configured to analyze the Amazon Web Services resources that are defined in the stacks or that are tagged using the same tag <i>key</i>. You can specify up to 500 Amazon Web Services CloudFormation stacks. </p>
    #[doc(hidden)]
    pub resource_collection: ::std::option::Option<crate::types::ResourceCollection>,
    /// <p>A collection of the names of Amazon Web Services services.</p>
    #[doc(hidden)]
    pub service_collection: ::std::option::Option<crate::types::ServiceCollection>,
    /// <p>The Amazon Resource Names (ARNs) of the Amazon Web Services resources that generated this insight.</p>
    #[doc(hidden)]
    pub associated_resource_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ReactiveInsightSummary {
    /// <p> The ID of a reactive summary. </p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p> The name of a reactive insight. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The severity of the insight. For more information, see <a href="https://docs.aws.amazon.com/devops-guru/latest/userguide/working-with-insights.html#understanding-insights-severities">Understanding insight severities</a> in the <i>Amazon DevOps Guru User Guide</i>.</p>
    pub fn severity(&self) -> ::std::option::Option<&crate::types::InsightSeverity> {
        self.severity.as_ref()
    }
    /// <p> The status of a reactive insight. </p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::InsightStatus> {
        self.status.as_ref()
    }
    /// <p> A time ranged that specifies when the observed behavior in an insight started and ended. </p>
    pub fn insight_time_range(&self) -> ::std::option::Option<&crate::types::InsightTimeRange> {
        self.insight_time_range.as_ref()
    }
    /// <p> A collection of Amazon Web Services resources supported by DevOps Guru. The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and Amazon Web Services resources that contain the same Amazon Web Services tag. DevOps Guru can be configured to analyze the Amazon Web Services resources that are defined in the stacks or that are tagged using the same tag <i>key</i>. You can specify up to 500 Amazon Web Services CloudFormation stacks. </p>
    pub fn resource_collection(&self) -> ::std::option::Option<&crate::types::ResourceCollection> {
        self.resource_collection.as_ref()
    }
    /// <p>A collection of the names of Amazon Web Services services.</p>
    pub fn service_collection(&self) -> ::std::option::Option<&crate::types::ServiceCollection> {
        self.service_collection.as_ref()
    }
    /// <p>The Amazon Resource Names (ARNs) of the Amazon Web Services resources that generated this insight.</p>
    pub fn associated_resource_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.associated_resource_arns.as_deref()
    }
}
impl ReactiveInsightSummary {
    /// Creates a new builder-style object to manufacture [`ReactiveInsightSummary`](crate::types::ReactiveInsightSummary).
    pub fn builder() -> crate::types::builders::ReactiveInsightSummaryBuilder {
        crate::types::builders::ReactiveInsightSummaryBuilder::default()
    }
}

/// A builder for [`ReactiveInsightSummary`](crate::types::ReactiveInsightSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReactiveInsightSummaryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) severity: ::std::option::Option<crate::types::InsightSeverity>,
    pub(crate) status: ::std::option::Option<crate::types::InsightStatus>,
    pub(crate) insight_time_range: ::std::option::Option<crate::types::InsightTimeRange>,
    pub(crate) resource_collection: ::std::option::Option<crate::types::ResourceCollection>,
    pub(crate) service_collection: ::std::option::Option<crate::types::ServiceCollection>,
    pub(crate) associated_resource_arns:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ReactiveInsightSummaryBuilder {
    /// <p> The ID of a reactive summary. </p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID of a reactive summary. </p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p> The name of a reactive insight. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of a reactive insight. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The severity of the insight. For more information, see <a href="https://docs.aws.amazon.com/devops-guru/latest/userguide/working-with-insights.html#understanding-insights-severities">Understanding insight severities</a> in the <i>Amazon DevOps Guru User Guide</i>.</p>
    pub fn severity(mut self, input: crate::types::InsightSeverity) -> Self {
        self.severity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The severity of the insight. For more information, see <a href="https://docs.aws.amazon.com/devops-guru/latest/userguide/working-with-insights.html#understanding-insights-severities">Understanding insight severities</a> in the <i>Amazon DevOps Guru User Guide</i>.</p>
    pub fn set_severity(
        mut self,
        input: ::std::option::Option<crate::types::InsightSeverity>,
    ) -> Self {
        self.severity = input;
        self
    }
    /// <p> The status of a reactive insight. </p>
    pub fn status(mut self, input: crate::types::InsightStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p> The status of a reactive insight. </p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::InsightStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p> A time ranged that specifies when the observed behavior in an insight started and ended. </p>
    pub fn insight_time_range(mut self, input: crate::types::InsightTimeRange) -> Self {
        self.insight_time_range = ::std::option::Option::Some(input);
        self
    }
    /// <p> A time ranged that specifies when the observed behavior in an insight started and ended. </p>
    pub fn set_insight_time_range(
        mut self,
        input: ::std::option::Option<crate::types::InsightTimeRange>,
    ) -> Self {
        self.insight_time_range = input;
        self
    }
    /// <p> A collection of Amazon Web Services resources supported by DevOps Guru. The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and Amazon Web Services resources that contain the same Amazon Web Services tag. DevOps Guru can be configured to analyze the Amazon Web Services resources that are defined in the stacks or that are tagged using the same tag <i>key</i>. You can specify up to 500 Amazon Web Services CloudFormation stacks. </p>
    pub fn resource_collection(mut self, input: crate::types::ResourceCollection) -> Self {
        self.resource_collection = ::std::option::Option::Some(input);
        self
    }
    /// <p> A collection of Amazon Web Services resources supported by DevOps Guru. The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and Amazon Web Services resources that contain the same Amazon Web Services tag. DevOps Guru can be configured to analyze the Amazon Web Services resources that are defined in the stacks or that are tagged using the same tag <i>key</i>. You can specify up to 500 Amazon Web Services CloudFormation stacks. </p>
    pub fn set_resource_collection(
        mut self,
        input: ::std::option::Option<crate::types::ResourceCollection>,
    ) -> Self {
        self.resource_collection = input;
        self
    }
    /// <p>A collection of the names of Amazon Web Services services.</p>
    pub fn service_collection(mut self, input: crate::types::ServiceCollection) -> Self {
        self.service_collection = ::std::option::Option::Some(input);
        self
    }
    /// <p>A collection of the names of Amazon Web Services services.</p>
    pub fn set_service_collection(
        mut self,
        input: ::std::option::Option<crate::types::ServiceCollection>,
    ) -> Self {
        self.service_collection = input;
        self
    }
    /// Appends an item to `associated_resource_arns`.
    ///
    /// To override the contents of this collection use [`set_associated_resource_arns`](Self::set_associated_resource_arns).
    ///
    /// <p>The Amazon Resource Names (ARNs) of the Amazon Web Services resources that generated this insight.</p>
    pub fn associated_resource_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.associated_resource_arns.unwrap_or_default();
        v.push(input.into());
        self.associated_resource_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Resource Names (ARNs) of the Amazon Web Services resources that generated this insight.</p>
    pub fn set_associated_resource_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.associated_resource_arns = input;
        self
    }
    /// Consumes the builder and constructs a [`ReactiveInsightSummary`](crate::types::ReactiveInsightSummary).
    pub fn build(self) -> crate::types::ReactiveInsightSummary {
        crate::types::ReactiveInsightSummary {
            id: self.id,
            name: self.name,
            severity: self.severity,
            status: self.status,
            insight_time_range: self.insight_time_range,
            resource_collection: self.resource_collection,
            service_collection: self.service_collection,
            associated_resource_arns: self.associated_resource_arns,
        }
    }
}
