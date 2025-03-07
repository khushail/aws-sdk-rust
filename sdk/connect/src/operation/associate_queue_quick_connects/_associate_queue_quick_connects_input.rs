// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateQueueQuickConnectsInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for the queue.</p>
    #[doc(hidden)]
    pub queue_id: ::std::option::Option<::std::string::String>,
    /// <p>The quick connects to associate with this queue.</p>
    #[doc(hidden)]
    pub quick_connect_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AssociateQueueQuickConnectsInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The identifier for the queue.</p>
    pub fn queue_id(&self) -> ::std::option::Option<&str> {
        self.queue_id.as_deref()
    }
    /// <p>The quick connects to associate with this queue.</p>
    pub fn quick_connect_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.quick_connect_ids.as_deref()
    }
}
impl AssociateQueueQuickConnectsInput {
    /// Creates a new builder-style object to manufacture [`AssociateQueueQuickConnectsInput`](crate::operation::associate_queue_quick_connects::AssociateQueueQuickConnectsInput).
    pub fn builder() -> crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsInputBuilder{
        crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsInputBuilder::default()
    }
}

/// A builder for [`AssociateQueueQuickConnectsInput`](crate::operation::associate_queue_quick_connects::AssociateQueueQuickConnectsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateQueueQuickConnectsInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) queue_id: ::std::option::Option<::std::string::String>,
    pub(crate) quick_connect_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AssociateQueueQuickConnectsInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The identifier for the queue.</p>
    pub fn queue_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the queue.</p>
    pub fn set_queue_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_id = input;
        self
    }
    /// Appends an item to `quick_connect_ids`.
    ///
    /// To override the contents of this collection use [`set_quick_connect_ids`](Self::set_quick_connect_ids).
    ///
    /// <p>The quick connects to associate with this queue.</p>
    pub fn quick_connect_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.quick_connect_ids.unwrap_or_default();
        v.push(input.into());
        self.quick_connect_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The quick connects to associate with this queue.</p>
    pub fn set_quick_connect_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.quick_connect_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateQueueQuickConnectsInput`](crate::operation::associate_queue_quick_connects::AssociateQueueQuickConnectsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_queue_quick_connects::AssociateQueueQuickConnectsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_queue_quick_connects::AssociateQueueQuickConnectsInput {
                instance_id: self.instance_id,
                queue_id: self.queue_id,
                quick_connect_ids: self.quick_connect_ids,
            },
        )
    }
}
