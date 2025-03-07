// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains a DNS record value that you can use to validate ownership or control of a domain. This is used by the <code>DescribeCertificate</code> action. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourceRecord {
    /// <p>The name of the DNS record to create in your domain. This is supplied by ACM.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The type of DNS record. Currently this can be <code>CNAME</code>.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::RecordType>,
    /// <p>The value of the CNAME record to add to your DNS database. This is supplied by ACM.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ResourceRecord {
    /// <p>The name of the DNS record to create in your domain. This is supplied by ACM.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The type of DNS record. Currently this can be <code>CNAME</code>.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::RecordType> {
        self.r#type.as_ref()
    }
    /// <p>The value of the CNAME record to add to your DNS database. This is supplied by ACM.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl ResourceRecord {
    /// Creates a new builder-style object to manufacture [`ResourceRecord`](crate::types::ResourceRecord).
    pub fn builder() -> crate::types::builders::ResourceRecordBuilder {
        crate::types::builders::ResourceRecordBuilder::default()
    }
}

/// A builder for [`ResourceRecord`](crate::types::ResourceRecord).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResourceRecordBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::RecordType>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl ResourceRecordBuilder {
    /// <p>The name of the DNS record to create in your domain. This is supplied by ACM.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the DNS record to create in your domain. This is supplied by ACM.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The type of DNS record. Currently this can be <code>CNAME</code>.</p>
    pub fn r#type(mut self, input: crate::types::RecordType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of DNS record. Currently this can be <code>CNAME</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::RecordType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The value of the CNAME record to add to your DNS database. This is supplied by ACM.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value of the CNAME record to add to your DNS database. This is supplied by ACM.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`ResourceRecord`](crate::types::ResourceRecord).
    pub fn build(self) -> crate::types::ResourceRecord {
        crate::types::ResourceRecord {
            name: self.name,
            r#type: self.r#type,
            value: self.value,
        }
    }
}
