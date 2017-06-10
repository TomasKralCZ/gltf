// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::json::Extras;

/// Joints and matrices defining a skin.
#[derive(Debug, Deserialize, Serialize)]
pub struct Skin {
    /// 4x4 transformation matrix stored in column-major order.
    #[serde(default = "skin_bind_shape_matrix")]
    #[serde(rename = "bindShapeMatrix")]
    pub bind_shape_matrix: [f32; 16],

    /// The ID of the accessor containing the floating-point 4x4 inverse-bind
    /// matrices.
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<String>,

    /// Joint names of the joints (nodes with a joint_name property) in this
    /// skin.
    ///
    /// The array length is the same as the count property of the
    /// inverse_bind_matrices accessor, and the same as the total quantity of
    /// all skeleton nodes from node-trees referenced by the skinned mesh
    /// instance node's skeletons array.
    #[serde(default)]
    #[serde(rename = "jointNames")]
    pub join_names: Vec<String>,

    /// The user-defined name of this object.
    pub name: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: SkinExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

fn skin_bind_shape_matrix() -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

/// Extension specific data for `Skin`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SkinExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

