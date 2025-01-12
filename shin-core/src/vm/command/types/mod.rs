//! Types used in commands.

mod flags;
mod id;
mod property;

pub use flags::{AudioWaitStatus, LayerCtrlFlags, MaskFlags};
pub use id::{
    LayerId, LayerIdOpt, VLayerId, VLayerIdRepr, LAYERBANKS_COUNT, LAYERS_COUNT, PLANES_COUNT,
};
pub use property::{LayerProperty, LayerPropertySmallList};

use crate::format::scenario::instructions::NumberSpec;
use crate::vm::{FromVmCtx, VmCtx};
use num_derive::FromPrimitive;

#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayerType {
    Null = 0,
    Tile = 1,
    Picture = 2,
    Bustup = 3,
    Animation = 4,
    Effect = 5,
    Movie = 6,
    FocusLine = 7,
    Rain = 8,
    Quiz = 9,
}

impl FromVmCtx<NumberSpec> for LayerType {
    fn from_vm_ctx(ctx: &VmCtx, input: NumberSpec) -> Self {
        let num = ctx.get_number(input);
        num_traits::FromPrimitive::from_i32(num)
            .unwrap_or_else(|| panic!("LayerType::from_vm_ctx: invalid layer type: {}", num))
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum MessageboxType {
    Neutral = 0,
    WitchSpace = 1,
    Ushiromiya = 2,
    Transparent = 3,
    Novel = 4,
    NoText = 5,
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum MessageTextLayout {
    Left = 0,
    /// I _think_ this is the same as Left
    Layout1 = 1,
    Center = 2,
    Right = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct MessageboxStyle {
    pub messagebox_type: MessageboxType,
    pub text_layout: MessageTextLayout,
}

impl Default for MessageboxStyle {
    fn default() -> Self {
        Self {
            messagebox_type: MessageboxType::Neutral,
            text_layout: MessageTextLayout::Left,
        }
    }
}

impl FromVmCtx<NumberSpec> for MessageboxStyle {
    fn from_vm_ctx(ctx: &VmCtx, input: NumberSpec) -> Self {
        let v = ctx.get_number(input);
        assert!(v >= 0);
        let msgbox_type = v & 0xf;
        let text_layout = (v >> 4) & 0xf;
        Self {
            messagebox_type: num_traits::FromPrimitive::from_i32(msgbox_type).unwrap_or_else(
                || panic!("MsgInit::from: unknown messagebox type: {}", msgbox_type),
            ),
            text_layout: num_traits::FromPrimitive::from_i32(text_layout)
                .unwrap_or_else(|| panic!("MsgInit::from: unknown text layout: {}", text_layout)),
        }
    }
}

/// A volume value, in the range [0.0, 1.0].
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Volume(pub f32);

impl Default for Volume {
    fn default() -> Self {
        Self(1.0)
    }
}

impl FromVmCtx<NumberSpec> for Volume {
    fn from_vm_ctx(ctx: &VmCtx, input: NumberSpec) -> Self {
        Self((ctx.get_number(input) as f32 / 1000.0).clamp(0.0, 1.0)) // TODO: warn if out of range
    }
}

/// Defines a pan value in the range [-1.0, 1.0], where `0.0` is the center and `-1.0` is the hard left and `1.0` is the hard right.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pan(pub f32);

impl Default for Pan {
    fn default() -> Self {
        Self(0.0)
    }
}

impl FromVmCtx<NumberSpec> for Pan {
    fn from_vm_ctx(ctx: &VmCtx, input: NumberSpec) -> Self {
        Self((ctx.get_number(input) as f32 / 1000.0).clamp(-1.0, 1.0)) // TODO: warn if out of range
    }
}
