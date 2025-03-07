// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDeleteConnectionInput {
    /// <p>The ID of the Data Catalog in which the connections reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    #[doc(hidden)]
    pub catalog_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of names of the connections to delete.</p>
    #[doc(hidden)]
    pub connection_name_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchDeleteConnectionInput {
    /// <p>The ID of the Data Catalog in which the connections reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn catalog_id(&self) -> ::std::option::Option<&str> {
        self.catalog_id.as_deref()
    }
    /// <p>A list of names of the connections to delete.</p>
    pub fn connection_name_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.connection_name_list.as_deref()
    }
}
impl BatchDeleteConnectionInput {
    /// Creates a new builder-style object to manufacture [`BatchDeleteConnectionInput`](crate::operation::batch_delete_connection::BatchDeleteConnectionInput).
    pub fn builder(
    ) -> crate::operation::batch_delete_connection::builders::BatchDeleteConnectionInputBuilder
    {
        crate::operation::batch_delete_connection::builders::BatchDeleteConnectionInputBuilder::default()
    }
}

/// A builder for [`BatchDeleteConnectionInput`](crate::operation::batch_delete_connection::BatchDeleteConnectionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchDeleteConnectionInputBuilder {
    pub(crate) catalog_id: ::std::option::Option<::std::string::String>,
    pub(crate) connection_name_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchDeleteConnectionInputBuilder {
    /// <p>The ID of the Data Catalog in which the connections reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.catalog_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Data Catalog in which the connections reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.catalog_id = input;
        self
    }
    /// Appends an item to `connection_name_list`.
    ///
    /// To override the contents of this collection use [`set_connection_name_list`](Self::set_connection_name_list).
    ///
    /// <p>A list of names of the connections to delete.</p>
    pub fn connection_name_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.connection_name_list.unwrap_or_default();
        v.push(input.into());
        self.connection_name_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of names of the connections to delete.</p>
    pub fn set_connection_name_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.connection_name_list = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchDeleteConnectionInput`](crate::operation::batch_delete_connection::BatchDeleteConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_delete_connection::BatchDeleteConnectionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::batch_delete_connection::BatchDeleteConnectionInput {
                catalog_id: self.catalog_id,
                connection_name_list: self.connection_name_list,
            },
        )
    }
}
