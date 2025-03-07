// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Settings requried for a slot type based on a grammar that you provide.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GrammarSlotTypeSetting {
    /// <p>The source of the grammar used to create the slot type.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<crate::types::GrammarSlotTypeSource>,
}
impl GrammarSlotTypeSetting {
    /// <p>The source of the grammar used to create the slot type.</p>
    pub fn source(&self) -> ::std::option::Option<&crate::types::GrammarSlotTypeSource> {
        self.source.as_ref()
    }
}
impl GrammarSlotTypeSetting {
    /// Creates a new builder-style object to manufacture [`GrammarSlotTypeSetting`](crate::types::GrammarSlotTypeSetting).
    pub fn builder() -> crate::types::builders::GrammarSlotTypeSettingBuilder {
        crate::types::builders::GrammarSlotTypeSettingBuilder::default()
    }
}

/// A builder for [`GrammarSlotTypeSetting`](crate::types::GrammarSlotTypeSetting).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GrammarSlotTypeSettingBuilder {
    pub(crate) source: ::std::option::Option<crate::types::GrammarSlotTypeSource>,
}
impl GrammarSlotTypeSettingBuilder {
    /// <p>The source of the grammar used to create the slot type.</p>
    pub fn source(mut self, input: crate::types::GrammarSlotTypeSource) -> Self {
        self.source = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source of the grammar used to create the slot type.</p>
    pub fn set_source(
        mut self,
        input: ::std::option::Option<crate::types::GrammarSlotTypeSource>,
    ) -> Self {
        self.source = input;
        self
    }
    /// Consumes the builder and constructs a [`GrammarSlotTypeSetting`](crate::types::GrammarSlotTypeSetting).
    pub fn build(self) -> crate::types::GrammarSlotTypeSetting {
        crate::types::GrammarSlotTypeSetting {
            source: self.source,
        }
    }
}
