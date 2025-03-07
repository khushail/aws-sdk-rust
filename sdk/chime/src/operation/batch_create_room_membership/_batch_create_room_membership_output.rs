// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchCreateRoomMembershipOutput {
    /// <p>If the action fails for one or more of the member IDs in the request, a list of the member IDs is returned, along with error codes and error messages.</p>
    #[doc(hidden)]
    pub errors: ::std::option::Option<::std::vec::Vec<crate::types::MemberError>>,
    _request_id: Option<String>,
}
impl BatchCreateRoomMembershipOutput {
    /// <p>If the action fails for one or more of the member IDs in the request, a list of the member IDs is returned, along with error codes and error messages.</p>
    pub fn errors(&self) -> ::std::option::Option<&[crate::types::MemberError]> {
        self.errors.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchCreateRoomMembershipOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchCreateRoomMembershipOutput {
    /// Creates a new builder-style object to manufacture [`BatchCreateRoomMembershipOutput`](crate::operation::batch_create_room_membership::BatchCreateRoomMembershipOutput).
    pub fn builder() -> crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipOutputBuilder{
        crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipOutputBuilder::default()
    }
}

/// A builder for [`BatchCreateRoomMembershipOutput`](crate::operation::batch_create_room_membership::BatchCreateRoomMembershipOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchCreateRoomMembershipOutputBuilder {
    pub(crate) errors: ::std::option::Option<::std::vec::Vec<crate::types::MemberError>>,
    _request_id: Option<String>,
}
impl BatchCreateRoomMembershipOutputBuilder {
    /// Appends an item to `errors`.
    ///
    /// To override the contents of this collection use [`set_errors`](Self::set_errors).
    ///
    /// <p>If the action fails for one or more of the member IDs in the request, a list of the member IDs is returned, along with error codes and error messages.</p>
    pub fn errors(mut self, input: crate::types::MemberError) -> Self {
        let mut v = self.errors.unwrap_or_default();
        v.push(input);
        self.errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>If the action fails for one or more of the member IDs in the request, a list of the member IDs is returned, along with error codes and error messages.</p>
    pub fn set_errors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MemberError>>,
    ) -> Self {
        self.errors = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`BatchCreateRoomMembershipOutput`](crate::operation::batch_create_room_membership::BatchCreateRoomMembershipOutput).
    pub fn build(
        self,
    ) -> crate::operation::batch_create_room_membership::BatchCreateRoomMembershipOutput {
        crate::operation::batch_create_room_membership::BatchCreateRoomMembershipOutput {
            errors: self.errors,
            _request_id: self._request_id,
        }
    }
}
