
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{accessor, scene, traits, Extensions, Extras, Index};

/// [Joints and matrices defining a skin](https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/skin.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skin<E: traits::Extensions, X: traits::Extras> {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The index of the accessor containing the 4x4 inverse-bind matrices
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<Index<accessor::Accessor<E, X>>>,
    /// Indices of skeleton nodes used as joints in this skin
    pub joints: Vec<Index<scene::Node<E, X>>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The index of the node used as a skeleton root
    pub skeleton: Option<Index<scene::Node<E, X>>>,
}
