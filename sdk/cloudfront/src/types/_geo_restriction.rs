// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that controls the countries in which your content is distributed. CloudFront determines the location of your users using <code>MaxMind</code> GeoIP databases. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GeoRestriction {
    /// <p>The method that you want to use to restrict distribution of your content by country:</p>
    /// <ul>
    /// <li> <p> <code>none</code>: No geo restriction is enabled, meaning access to content is not restricted by client geo location.</p> </li>
    /// <li> <p> <code>blacklist</code>: The <code>Location</code> elements specify the countries in which you don't want CloudFront to distribute your content.</p> </li>
    /// <li> <p> <code>whitelist</code>: The <code>Location</code> elements specify the countries in which you want CloudFront to distribute your content.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub restriction_type: ::std::option::Option<crate::types::GeoRestrictionType>,
    /// <p>When geo restriction is <code>enabled</code>, this is the number of countries in your <code>whitelist</code> or <code>blacklist</code>. Otherwise, when it is not enabled, <code>Quantity</code> is <code>0</code>, and you can omit <code>Items</code>.</p>
    #[doc(hidden)]
    pub quantity: ::std::option::Option<i32>,
    /// <p>A complex type that contains a <code>Location</code> element for each country in which you want CloudFront either to distribute your content (<code>whitelist</code>) or not distribute your content (<code>blacklist</code>).</p>
    /// <p>The <code>Location</code> element is a two-letter, uppercase country code for a country that you want to include in your <code>blacklist</code> or <code>whitelist</code>. Include one <code>Location</code> element for each country.</p>
    /// <p>CloudFront and <code>MaxMind</code> both use <code>ISO 3166</code> country codes. For the current list of countries and the corresponding codes, see <code>ISO 3166-1-alpha-2</code> code on the <i>International Organization for Standardization</i> website. You can also refer to the country list on the CloudFront console, which includes both country names and codes.</p>
    #[doc(hidden)]
    pub items: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GeoRestriction {
    /// <p>The method that you want to use to restrict distribution of your content by country:</p>
    /// <ul>
    /// <li> <p> <code>none</code>: No geo restriction is enabled, meaning access to content is not restricted by client geo location.</p> </li>
    /// <li> <p> <code>blacklist</code>: The <code>Location</code> elements specify the countries in which you don't want CloudFront to distribute your content.</p> </li>
    /// <li> <p> <code>whitelist</code>: The <code>Location</code> elements specify the countries in which you want CloudFront to distribute your content.</p> </li>
    /// </ul>
    pub fn restriction_type(&self) -> ::std::option::Option<&crate::types::GeoRestrictionType> {
        self.restriction_type.as_ref()
    }
    /// <p>When geo restriction is <code>enabled</code>, this is the number of countries in your <code>whitelist</code> or <code>blacklist</code>. Otherwise, when it is not enabled, <code>Quantity</code> is <code>0</code>, and you can omit <code>Items</code>.</p>
    pub fn quantity(&self) -> ::std::option::Option<i32> {
        self.quantity
    }
    /// <p>A complex type that contains a <code>Location</code> element for each country in which you want CloudFront either to distribute your content (<code>whitelist</code>) or not distribute your content (<code>blacklist</code>).</p>
    /// <p>The <code>Location</code> element is a two-letter, uppercase country code for a country that you want to include in your <code>blacklist</code> or <code>whitelist</code>. Include one <code>Location</code> element for each country.</p>
    /// <p>CloudFront and <code>MaxMind</code> both use <code>ISO 3166</code> country codes. For the current list of countries and the corresponding codes, see <code>ISO 3166-1-alpha-2</code> code on the <i>International Organization for Standardization</i> website. You can also refer to the country list on the CloudFront console, which includes both country names and codes.</p>
    pub fn items(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.items.as_deref()
    }
}
impl GeoRestriction {
    /// Creates a new builder-style object to manufacture [`GeoRestriction`](crate::types::GeoRestriction).
    pub fn builder() -> crate::types::builders::GeoRestrictionBuilder {
        crate::types::builders::GeoRestrictionBuilder::default()
    }
}

/// A builder for [`GeoRestriction`](crate::types::GeoRestriction).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GeoRestrictionBuilder {
    pub(crate) restriction_type: ::std::option::Option<crate::types::GeoRestrictionType>,
    pub(crate) quantity: ::std::option::Option<i32>,
    pub(crate) items: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GeoRestrictionBuilder {
    /// <p>The method that you want to use to restrict distribution of your content by country:</p>
    /// <ul>
    /// <li> <p> <code>none</code>: No geo restriction is enabled, meaning access to content is not restricted by client geo location.</p> </li>
    /// <li> <p> <code>blacklist</code>: The <code>Location</code> elements specify the countries in which you don't want CloudFront to distribute your content.</p> </li>
    /// <li> <p> <code>whitelist</code>: The <code>Location</code> elements specify the countries in which you want CloudFront to distribute your content.</p> </li>
    /// </ul>
    pub fn restriction_type(mut self, input: crate::types::GeoRestrictionType) -> Self {
        self.restriction_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The method that you want to use to restrict distribution of your content by country:</p>
    /// <ul>
    /// <li> <p> <code>none</code>: No geo restriction is enabled, meaning access to content is not restricted by client geo location.</p> </li>
    /// <li> <p> <code>blacklist</code>: The <code>Location</code> elements specify the countries in which you don't want CloudFront to distribute your content.</p> </li>
    /// <li> <p> <code>whitelist</code>: The <code>Location</code> elements specify the countries in which you want CloudFront to distribute your content.</p> </li>
    /// </ul>
    pub fn set_restriction_type(
        mut self,
        input: ::std::option::Option<crate::types::GeoRestrictionType>,
    ) -> Self {
        self.restriction_type = input;
        self
    }
    /// <p>When geo restriction is <code>enabled</code>, this is the number of countries in your <code>whitelist</code> or <code>blacklist</code>. Otherwise, when it is not enabled, <code>Quantity</code> is <code>0</code>, and you can omit <code>Items</code>.</p>
    pub fn quantity(mut self, input: i32) -> Self {
        self.quantity = ::std::option::Option::Some(input);
        self
    }
    /// <p>When geo restriction is <code>enabled</code>, this is the number of countries in your <code>whitelist</code> or <code>blacklist</code>. Otherwise, when it is not enabled, <code>Quantity</code> is <code>0</code>, and you can omit <code>Items</code>.</p>
    pub fn set_quantity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.quantity = input;
        self
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>A complex type that contains a <code>Location</code> element for each country in which you want CloudFront either to distribute your content (<code>whitelist</code>) or not distribute your content (<code>blacklist</code>).</p>
    /// <p>The <code>Location</code> element is a two-letter, uppercase country code for a country that you want to include in your <code>blacklist</code> or <code>whitelist</code>. Include one <code>Location</code> element for each country.</p>
    /// <p>CloudFront and <code>MaxMind</code> both use <code>ISO 3166</code> country codes. For the current list of countries and the corresponding codes, see <code>ISO 3166-1-alpha-2</code> code on the <i>International Organization for Standardization</i> website. You can also refer to the country list on the CloudFront console, which includes both country names and codes.</p>
    pub fn items(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input.into());
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>A complex type that contains a <code>Location</code> element for each country in which you want CloudFront either to distribute your content (<code>whitelist</code>) or not distribute your content (<code>blacklist</code>).</p>
    /// <p>The <code>Location</code> element is a two-letter, uppercase country code for a country that you want to include in your <code>blacklist</code> or <code>whitelist</code>. Include one <code>Location</code> element for each country.</p>
    /// <p>CloudFront and <code>MaxMind</code> both use <code>ISO 3166</code> country codes. For the current list of countries and the corresponding codes, see <code>ISO 3166-1-alpha-2</code> code on the <i>International Organization for Standardization</i> website. You can also refer to the country list on the CloudFront console, which includes both country names and codes.</p>
    pub fn set_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// Consumes the builder and constructs a [`GeoRestriction`](crate::types::GeoRestriction).
    pub fn build(self) -> crate::types::GeoRestriction {
        crate::types::GeoRestriction {
            restriction_type: self.restriction_type,
            quantity: self.quantity,
            items: self.items,
        }
    }
}
