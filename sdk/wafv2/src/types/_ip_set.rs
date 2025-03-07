// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains zero or more IP addresses or blocks of IP addresses specified in Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for /0. For information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>. </p>
/// <p>WAF assigns an ARN to each <code>IPSet</code> that you create. To use an IP set in a rule, you provide the ARN to the <code>Rule</code> statement <code>IPSetReferenceStatement</code>. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IpSet {
    /// <p>The name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>A description of the IP set that helps with identification. </p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The version of the IP addresses, either <code>IPV4</code> or <code>IPV6</code>. </p>
    #[doc(hidden)]
    pub ip_address_version: ::std::option::Option<crate::types::IpAddressVersion>,
    /// <p>Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for <code>/0</code>. </p>
    /// <p>Example address strings: </p>
    /// <ul>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li>
    /// </ul>
    /// <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
    /// <p>Example JSON <code>Addresses</code> specifications: </p>
    /// <ul>
    /// <li> <p>Empty array: <code>"Addresses": []</code> </p> </li>
    /// <li> <p>Array with one address: <code>"Addresses": ["192.0.2.44/32"]</code> </p> </li>
    /// <li> <p>Array with three addresses: <code>"Addresses": ["192.0.2.44/32", "192.0.2.0/24", "192.0.0.0/16"]</code> </p> </li>
    /// <li> <p>INVALID specification: <code>"Addresses": [""]</code> INVALID </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl IpSet {
    /// <p>The name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>A description of the IP set that helps with identification. </p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The version of the IP addresses, either <code>IPV4</code> or <code>IPV6</code>. </p>
    pub fn ip_address_version(&self) -> ::std::option::Option<&crate::types::IpAddressVersion> {
        self.ip_address_version.as_ref()
    }
    /// <p>Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for <code>/0</code>. </p>
    /// <p>Example address strings: </p>
    /// <ul>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li>
    /// </ul>
    /// <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
    /// <p>Example JSON <code>Addresses</code> specifications: </p>
    /// <ul>
    /// <li> <p>Empty array: <code>"Addresses": []</code> </p> </li>
    /// <li> <p>Array with one address: <code>"Addresses": ["192.0.2.44/32"]</code> </p> </li>
    /// <li> <p>Array with three addresses: <code>"Addresses": ["192.0.2.44/32", "192.0.2.0/24", "192.0.0.0/16"]</code> </p> </li>
    /// <li> <p>INVALID specification: <code>"Addresses": [""]</code> INVALID </p> </li>
    /// </ul>
    pub fn addresses(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.addresses.as_deref()
    }
}
impl IpSet {
    /// Creates a new builder-style object to manufacture [`IpSet`](crate::types::IpSet).
    pub fn builder() -> crate::types::builders::IpSetBuilder {
        crate::types::builders::IpSetBuilder::default()
    }
}

/// A builder for [`IpSet`](crate::types::IpSet).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IpSetBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) ip_address_version: ::std::option::Option<crate::types::IpAddressVersion>,
    pub(crate) addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl IpSetBuilder {
    /// <p>The name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>A description of the IP set that helps with identification. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the IP set that helps with identification. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The version of the IP addresses, either <code>IPV4</code> or <code>IPV6</code>. </p>
    pub fn ip_address_version(mut self, input: crate::types::IpAddressVersion) -> Self {
        self.ip_address_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version of the IP addresses, either <code>IPV4</code> or <code>IPV6</code>. </p>
    pub fn set_ip_address_version(
        mut self,
        input: ::std::option::Option<crate::types::IpAddressVersion>,
    ) -> Self {
        self.ip_address_version = input;
        self
    }
    /// Appends an item to `addresses`.
    ///
    /// To override the contents of this collection use [`set_addresses`](Self::set_addresses).
    ///
    /// <p>Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for <code>/0</code>. </p>
    /// <p>Example address strings: </p>
    /// <ul>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li>
    /// </ul>
    /// <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
    /// <p>Example JSON <code>Addresses</code> specifications: </p>
    /// <ul>
    /// <li> <p>Empty array: <code>"Addresses": []</code> </p> </li>
    /// <li> <p>Array with one address: <code>"Addresses": ["192.0.2.44/32"]</code> </p> </li>
    /// <li> <p>Array with three addresses: <code>"Addresses": ["192.0.2.44/32", "192.0.2.0/24", "192.0.0.0/16"]</code> </p> </li>
    /// <li> <p>INVALID specification: <code>"Addresses": [""]</code> INVALID </p> </li>
    /// </ul>
    pub fn addresses(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.addresses.unwrap_or_default();
        v.push(input.into());
        self.addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for <code>/0</code>. </p>
    /// <p>Example address strings: </p>
    /// <ul>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li>
    /// <li> <p>To configure WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li>
    /// </ul>
    /// <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
    /// <p>Example JSON <code>Addresses</code> specifications: </p>
    /// <ul>
    /// <li> <p>Empty array: <code>"Addresses": []</code> </p> </li>
    /// <li> <p>Array with one address: <code>"Addresses": ["192.0.2.44/32"]</code> </p> </li>
    /// <li> <p>Array with three addresses: <code>"Addresses": ["192.0.2.44/32", "192.0.2.0/24", "192.0.0.0/16"]</code> </p> </li>
    /// <li> <p>INVALID specification: <code>"Addresses": [""]</code> INVALID </p> </li>
    /// </ul>
    pub fn set_addresses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.addresses = input;
        self
    }
    /// Consumes the builder and constructs a [`IpSet`](crate::types::IpSet).
    pub fn build(self) -> crate::types::IpSet {
        crate::types::IpSet {
            name: self.name,
            id: self.id,
            arn: self.arn,
            description: self.description,
            ip_address_version: self.ip_address_version,
            addresses: self.addresses,
        }
    }
}
