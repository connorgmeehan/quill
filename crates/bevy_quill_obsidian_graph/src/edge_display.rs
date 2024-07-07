use std::ops::Mul;

use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderPointerEvents};
use bevy_quill::prelude::*;

use crate::materials::{DrawPathMaterial, DrawablePath};

/// Displays a stroked path between two nodes.
#[derive(Clone, PartialEq)]
pub struct EdgeDisplay {
    /// Pixel position of the source terminal.
    pub src_pos: IVec2,

    /// Color of the edge at the source terminal
    pub src_color: Srgba,

    /// Pixel position of the destination terminal.
    pub dst_pos: IVec2,

    /// Color of the edge at the destination terminal
    pub dst_color: Srgba,
}

impl ViewTemplate for EdgeDisplay {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let material = cx.create_memo(
            |world, _| {
                let mut ui_materials = world
                    .get_resource_mut::<Assets<DrawPathMaterial>>()
                    .unwrap();
                ui_materials.add(DrawPathMaterial::default())
            },
            (),
        );
        let material_id = material.id();

        Element::<MaterialNodeBundle<DrawPathMaterial>>::new()
            .named("NodeGraph::Edge")
            .insert(material)
            .style(|sb: &mut StyleBuilder| {
                sb.pointer_events(false);
            })
            .effect(
                move |cx, ent, (src, dst, src_color, dst_color)| {
                    let mut path = DrawablePath::new(1.7);
                    let dx = (dst.x - src.x).abs().mul(0.3).min(20.);
                    let src1 = src + Vec2::new(dx, 0.);
                    let dst1 = dst - Vec2::new(dx, 0.);
                    path.move_to(src);
                    let mlen = src1.distance(dst1);
                    if mlen > 40. {
                        let src2 = src1.lerp(dst1, 20. / mlen);
                        let dst2 = src1.lerp(dst1, (mlen - 20.) / mlen);
                        path.quadratic_to(src1, src2);
                        path.line_to(dst2);
                        path.quadratic_to(dst1, dst);
                    } else {
                        let mid = src1.lerp(dst1, 0.5);
                        path.quadratic_to(src1, mid);
                        path.quadratic_to(dst1, dst);
                    }
                    let bounds = path.bounds();

                    let mut style = cx.world_mut().get_mut::<Style>(ent).unwrap();
                    style.left = ui::Val::Px(bounds.min.x);
                    style.top = ui::Val::Px(bounds.min.y);
                    style.width = ui::Val::Px(bounds.width());
                    style.height = ui::Val::Px(bounds.height());
                    style.position_type = ui::PositionType::Absolute;

                    let mut materials = cx
                        .world_mut()
                        .get_resource_mut::<Assets<DrawPathMaterial>>()
                        .unwrap();
                    let material = materials.get_mut(material_id).unwrap();
                    material.update_path(&path);
                    material.update_color(src_color, src - bounds.min, dst_color, dst - bounds.min);
                },
                (
                    self.src_pos.as_vec2(),
                    self.dst_pos.as_vec2(),
                    self.src_color,
                    self.dst_color,
                ),
            )
    }
}
