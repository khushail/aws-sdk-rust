// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of the Auto Scaling group capacity provider to update.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutoScalingGroupProviderUpdate {
    /// <p>The managed scaling settings for the Auto Scaling group capacity provider.</p>
    #[doc(hidden)]
    pub managed_scaling: ::std::option::Option<crate::types::ManagedScaling>,
    /// <p>The managed termination protection setting to use for the Auto Scaling group capacity provider. This determines whether the Auto Scaling group has managed termination protection.</p> <important>
    /// <p>When using managed termination protection, managed scaling must also be used otherwise managed termination protection doesn't work.</p>
    /// </important>
    /// <p>When managed termination protection is on, Amazon ECS prevents the Amazon EC2 instances in an Auto Scaling group that contain tasks from being terminated during a scale-in action. The Auto Scaling group and each instance in the Auto Scaling group must have instance protection from scale-in actions on. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p>
    /// <p>When managed termination protection is off, your Amazon EC2 instances aren't protected from termination when the Auto Scaling group scales in.</p>
    #[doc(hidden)]
    pub managed_termination_protection:
        ::std::option::Option<crate::types::ManagedTerminationProtection>,
}
impl AutoScalingGroupProviderUpdate {
    /// <p>The managed scaling settings for the Auto Scaling group capacity provider.</p>
    pub fn managed_scaling(&self) -> ::std::option::Option<&crate::types::ManagedScaling> {
        self.managed_scaling.as_ref()
    }
    /// <p>The managed termination protection setting to use for the Auto Scaling group capacity provider. This determines whether the Auto Scaling group has managed termination protection.</p> <important>
    /// <p>When using managed termination protection, managed scaling must also be used otherwise managed termination protection doesn't work.</p>
    /// </important>
    /// <p>When managed termination protection is on, Amazon ECS prevents the Amazon EC2 instances in an Auto Scaling group that contain tasks from being terminated during a scale-in action. The Auto Scaling group and each instance in the Auto Scaling group must have instance protection from scale-in actions on. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p>
    /// <p>When managed termination protection is off, your Amazon EC2 instances aren't protected from termination when the Auto Scaling group scales in.</p>
    pub fn managed_termination_protection(
        &self,
    ) -> ::std::option::Option<&crate::types::ManagedTerminationProtection> {
        self.managed_termination_protection.as_ref()
    }
}
impl AutoScalingGroupProviderUpdate {
    /// Creates a new builder-style object to manufacture [`AutoScalingGroupProviderUpdate`](crate::types::AutoScalingGroupProviderUpdate).
    pub fn builder() -> crate::types::builders::AutoScalingGroupProviderUpdateBuilder {
        crate::types::builders::AutoScalingGroupProviderUpdateBuilder::default()
    }
}

/// A builder for [`AutoScalingGroupProviderUpdate`](crate::types::AutoScalingGroupProviderUpdate).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AutoScalingGroupProviderUpdateBuilder {
    pub(crate) managed_scaling: ::std::option::Option<crate::types::ManagedScaling>,
    pub(crate) managed_termination_protection:
        ::std::option::Option<crate::types::ManagedTerminationProtection>,
}
impl AutoScalingGroupProviderUpdateBuilder {
    /// <p>The managed scaling settings for the Auto Scaling group capacity provider.</p>
    pub fn managed_scaling(mut self, input: crate::types::ManagedScaling) -> Self {
        self.managed_scaling = ::std::option::Option::Some(input);
        self
    }
    /// <p>The managed scaling settings for the Auto Scaling group capacity provider.</p>
    pub fn set_managed_scaling(
        mut self,
        input: ::std::option::Option<crate::types::ManagedScaling>,
    ) -> Self {
        self.managed_scaling = input;
        self
    }
    /// <p>The managed termination protection setting to use for the Auto Scaling group capacity provider. This determines whether the Auto Scaling group has managed termination protection.</p> <important>
    /// <p>When using managed termination protection, managed scaling must also be used otherwise managed termination protection doesn't work.</p>
    /// </important>
    /// <p>When managed termination protection is on, Amazon ECS prevents the Amazon EC2 instances in an Auto Scaling group that contain tasks from being terminated during a scale-in action. The Auto Scaling group and each instance in the Auto Scaling group must have instance protection from scale-in actions on. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p>
    /// <p>When managed termination protection is off, your Amazon EC2 instances aren't protected from termination when the Auto Scaling group scales in.</p>
    pub fn managed_termination_protection(
        mut self,
        input: crate::types::ManagedTerminationProtection,
    ) -> Self {
        self.managed_termination_protection = ::std::option::Option::Some(input);
        self
    }
    /// <p>The managed termination protection setting to use for the Auto Scaling group capacity provider. This determines whether the Auto Scaling group has managed termination protection.</p> <important>
    /// <p>When using managed termination protection, managed scaling must also be used otherwise managed termination protection doesn't work.</p>
    /// </important>
    /// <p>When managed termination protection is on, Amazon ECS prevents the Amazon EC2 instances in an Auto Scaling group that contain tasks from being terminated during a scale-in action. The Auto Scaling group and each instance in the Auto Scaling group must have instance protection from scale-in actions on. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p>
    /// <p>When managed termination protection is off, your Amazon EC2 instances aren't protected from termination when the Auto Scaling group scales in.</p>
    pub fn set_managed_termination_protection(
        mut self,
        input: ::std::option::Option<crate::types::ManagedTerminationProtection>,
    ) -> Self {
        self.managed_termination_protection = input;
        self
    }
    /// Consumes the builder and constructs a [`AutoScalingGroupProviderUpdate`](crate::types::AutoScalingGroupProviderUpdate).
    pub fn build(self) -> crate::types::AutoScalingGroupProviderUpdate {
        crate::types::AutoScalingGroupProviderUpdate {
            managed_scaling: self.managed_scaling,
            managed_termination_protection: self.managed_termination_protection,
        }
    }
}
