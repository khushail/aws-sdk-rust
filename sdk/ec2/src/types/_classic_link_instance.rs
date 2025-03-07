// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
/// <p>Describes a linked EC2-Classic instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ClassicLinkInstance {
    /// <p>A list of security groups.</p>
    #[doc(hidden)]
    pub groups: ::std::option::Option<::std::vec::Vec<crate::types::GroupIdentifier>>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>Any tags assigned to the instance.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: ::std::option::Option<::std::string::String>,
}
impl ClassicLinkInstance {
    /// <p>A list of security groups.</p>
    pub fn groups(&self) -> ::std::option::Option<&[crate::types::GroupIdentifier]> {
        self.groups.as_deref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>Any tags assigned to the instance.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl ClassicLinkInstance {
    /// Creates a new builder-style object to manufacture [`ClassicLinkInstance`](crate::types::ClassicLinkInstance).
    pub fn builder() -> crate::types::builders::ClassicLinkInstanceBuilder {
        crate::types::builders::ClassicLinkInstanceBuilder::default()
    }
}

/// A builder for [`ClassicLinkInstance`](crate::types::ClassicLinkInstance).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ClassicLinkInstanceBuilder {
    pub(crate) groups: ::std::option::Option<::std::vec::Vec<crate::types::GroupIdentifier>>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
}
impl ClassicLinkInstanceBuilder {
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>A list of security groups.</p>
    pub fn groups(mut self, input: crate::types::GroupIdentifier) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input);
        self.groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of security groups.</p>
    pub fn set_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::GroupIdentifier>>,
    ) -> Self {
        self.groups = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the instance.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any tags assigned to the instance.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ClassicLinkInstance`](crate::types::ClassicLinkInstance).
    pub fn build(self) -> crate::types::ClassicLinkInstance {
        crate::types::ClassicLinkInstance {
            groups: self.groups,
            instance_id: self.instance_id,
            tags: self.tags,
            vpc_id: self.vpc_id,
        }
    }
}
