//! Live Blends (Object → Blend), Envelope Distort (Object → Envelope Distort) and Gradient Mesh
//! (Object → Create Gradient Mesh, Mesh tool edits).
//!
//! The objects are live document nodes (`NodeKind::Blend`, `NodeKind::Envelope`,
//! `NodeKind::Mesh`); evaluation lives in `vectorz_doc::live` and the renderer. **Expand**
//! replaces a live object by its evaluated geometry (`vectorz_render::expand_live`).

use std::sync::Arc;

use serde_json::{Value, json};
use vectorz_color::{Color, Paint};
use vectorz_doc::live::{
    self, BlendDefaults, BlendEase, BlendEaseType, BlendOrientation, BlendSpacing, BlendSpec, EnvelopeKind, EnvelopeMap, EnvelopeOptions,
    GradientMesh, MeshAppearance, PreserveShape, Spine,
};
use vectorz_doc::{Appearance, Document, Node, NodeId, NodeKind, Selection};
use vectorz_geom::{Affine, Anchor, PathData, Point, Vec2};

use super::edit::selected_roots;
use super::expand::gradient_sampler;
use super::pathops::Beside;
use super::*;

mod blend;
mod commands;
mod common;
mod envelope;
mod mesh;

pub(crate) use blend::expand_blends;
pub use commands::specs;
pub(super) use common::fix_ids;
pub(super) use envelope::expand_envelopes_under;
