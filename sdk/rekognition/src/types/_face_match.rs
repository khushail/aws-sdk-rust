// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides face metadata. In addition, it also provides the confidence in the match of this face with the input face.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FaceMatch {
    /// <p>Confidence in the match of this face with the input face.</p>
    #[doc(hidden)]
    pub similarity: ::std::option::Option<f32>,
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>
    #[doc(hidden)]
    pub face: ::std::option::Option<crate::types::Face>,
}
impl FaceMatch {
    /// <p>Confidence in the match of this face with the input face.</p>
    pub fn similarity(&self) -> ::std::option::Option<f32> {
        self.similarity
    }
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>
    pub fn face(&self) -> ::std::option::Option<&crate::types::Face> {
        self.face.as_ref()
    }
}
impl FaceMatch {
    /// Creates a new builder-style object to manufacture [`FaceMatch`](crate::types::FaceMatch).
    pub fn builder() -> crate::types::builders::FaceMatchBuilder {
        crate::types::builders::FaceMatchBuilder::default()
    }
}

/// A builder for [`FaceMatch`](crate::types::FaceMatch).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FaceMatchBuilder {
    pub(crate) similarity: ::std::option::Option<f32>,
    pub(crate) face: ::std::option::Option<crate::types::Face>,
}
impl FaceMatchBuilder {
    /// <p>Confidence in the match of this face with the input face.</p>
    pub fn similarity(mut self, input: f32) -> Self {
        self.similarity = ::std::option::Option::Some(input);
        self
    }
    /// <p>Confidence in the match of this face with the input face.</p>
    pub fn set_similarity(mut self, input: ::std::option::Option<f32>) -> Self {
        self.similarity = input;
        self
    }
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>
    pub fn face(mut self, input: crate::types::Face) -> Self {
        self.face = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>
    pub fn set_face(mut self, input: ::std::option::Option<crate::types::Face>) -> Self {
        self.face = input;
        self
    }
    /// Consumes the builder and constructs a [`FaceMatch`](crate::types::FaceMatch).
    pub fn build(self) -> crate::types::FaceMatch {
        crate::types::FaceMatch {
            similarity: self.similarity,
            face: self.face,
        }
    }
}
