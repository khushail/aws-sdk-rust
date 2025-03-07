// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for the CreateThing operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateThingInput {
    /// <p>The name of the thing to create.</p>
    /// <p>You can't change a thing's name after you create it. To change a thing's name, you must create a new thing, give it the new name, and then delete the old thing.</p>
    #[doc(hidden)]
    pub thing_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the thing type associated with the new thing.</p>
    #[doc(hidden)]
    pub thing_type_name: ::std::option::Option<::std::string::String>,
    /// <p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p>
    /// <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    #[doc(hidden)]
    pub attribute_payload: ::std::option::Option<crate::types::AttributePayload>,
    /// <p>The name of the billing group the thing will be added to.</p>
    #[doc(hidden)]
    pub billing_group_name: ::std::option::Option<::std::string::String>,
}
impl CreateThingInput {
    /// <p>The name of the thing to create.</p>
    /// <p>You can't change a thing's name after you create it. To change a thing's name, you must create a new thing, give it the new name, and then delete the old thing.</p>
    pub fn thing_name(&self) -> ::std::option::Option<&str> {
        self.thing_name.as_deref()
    }
    /// <p>The name of the thing type associated with the new thing.</p>
    pub fn thing_type_name(&self) -> ::std::option::Option<&str> {
        self.thing_type_name.as_deref()
    }
    /// <p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p>
    /// <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    pub fn attribute_payload(&self) -> ::std::option::Option<&crate::types::AttributePayload> {
        self.attribute_payload.as_ref()
    }
    /// <p>The name of the billing group the thing will be added to.</p>
    pub fn billing_group_name(&self) -> ::std::option::Option<&str> {
        self.billing_group_name.as_deref()
    }
}
impl CreateThingInput {
    /// Creates a new builder-style object to manufacture [`CreateThingInput`](crate::operation::create_thing::CreateThingInput).
    pub fn builder() -> crate::operation::create_thing::builders::CreateThingInputBuilder {
        crate::operation::create_thing::builders::CreateThingInputBuilder::default()
    }
}

/// A builder for [`CreateThingInput`](crate::operation::create_thing::CreateThingInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateThingInputBuilder {
    pub(crate) thing_name: ::std::option::Option<::std::string::String>,
    pub(crate) thing_type_name: ::std::option::Option<::std::string::String>,
    pub(crate) attribute_payload: ::std::option::Option<crate::types::AttributePayload>,
    pub(crate) billing_group_name: ::std::option::Option<::std::string::String>,
}
impl CreateThingInputBuilder {
    /// <p>The name of the thing to create.</p>
    /// <p>You can't change a thing's name after you create it. To change a thing's name, you must create a new thing, give it the new name, and then delete the old thing.</p>
    pub fn thing_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.thing_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the thing to create.</p>
    /// <p>You can't change a thing's name after you create it. To change a thing's name, you must create a new thing, give it the new name, and then delete the old thing.</p>
    pub fn set_thing_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.thing_name = input;
        self
    }
    /// <p>The name of the thing type associated with the new thing.</p>
    pub fn thing_type_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.thing_type_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the thing type associated with the new thing.</p>
    pub fn set_thing_type_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.thing_type_name = input;
        self
    }
    /// <p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p>
    /// <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    pub fn attribute_payload(mut self, input: crate::types::AttributePayload) -> Self {
        self.attribute_payload = ::std::option::Option::Some(input);
        self
    }
    /// <p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p>
    /// <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    pub fn set_attribute_payload(
        mut self,
        input: ::std::option::Option<crate::types::AttributePayload>,
    ) -> Self {
        self.attribute_payload = input;
        self
    }
    /// <p>The name of the billing group the thing will be added to.</p>
    pub fn billing_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.billing_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the billing group the thing will be added to.</p>
    pub fn set_billing_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.billing_group_name = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateThingInput`](crate::operation::create_thing::CreateThingInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_thing::CreateThingInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_thing::CreateThingInput {
            thing_name: self.thing_name,
            thing_type_name: self.thing_type_name,
            attribute_payload: self.attribute_payload,
            billing_group_name: self.billing_group_name,
        })
    }
}
