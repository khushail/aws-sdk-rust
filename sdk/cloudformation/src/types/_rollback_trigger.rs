// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A rollback trigger CloudFormation monitors during creation and updating of stacks. If any of the alarms you specify goes to ALARM state during the stack operation or within the specified monitoring period afterwards, CloudFormation rolls back the entire stack operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RollbackTrigger {
    /// <p>The Amazon Resource Name (ARN) of the rollback trigger.</p>
    /// <p>If a specified trigger is missing, the entire stack operation fails and is rolled back.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The resource type of the rollback trigger. Specify either <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html">AWS::CloudWatch::Alarm</a> or <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html">AWS::CloudWatch::CompositeAlarm</a> resource types.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::string::String>,
}
impl RollbackTrigger {
    /// <p>The Amazon Resource Name (ARN) of the rollback trigger.</p>
    /// <p>If a specified trigger is missing, the entire stack operation fails and is rolled back.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The resource type of the rollback trigger. Specify either <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html">AWS::CloudWatch::Alarm</a> or <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html">AWS::CloudWatch::CompositeAlarm</a> resource types.</p>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
}
impl RollbackTrigger {
    /// Creates a new builder-style object to manufacture [`RollbackTrigger`](crate::types::RollbackTrigger).
    pub fn builder() -> crate::types::builders::RollbackTriggerBuilder {
        crate::types::builders::RollbackTriggerBuilder::default()
    }
}

/// A builder for [`RollbackTrigger`](crate::types::RollbackTrigger).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RollbackTriggerBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
}
impl RollbackTriggerBuilder {
    /// <p>The Amazon Resource Name (ARN) of the rollback trigger.</p>
    /// <p>If a specified trigger is missing, the entire stack operation fails and is rolled back.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the rollback trigger.</p>
    /// <p>If a specified trigger is missing, the entire stack operation fails and is rolled back.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The resource type of the rollback trigger. Specify either <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html">AWS::CloudWatch::Alarm</a> or <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html">AWS::CloudWatch::CompositeAlarm</a> resource types.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The resource type of the rollback trigger. Specify either <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html">AWS::CloudWatch::Alarm</a> or <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html">AWS::CloudWatch::CompositeAlarm</a> resource types.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// Consumes the builder and constructs a [`RollbackTrigger`](crate::types::RollbackTrigger).
    pub fn build(self) -> crate::types::RollbackTrigger {
        crate::types::RollbackTrigger {
            arn: self.arn,
            r#type: self.r#type,
        }
    }
}
