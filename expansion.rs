#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate image;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;
use glutin::GlContext;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;








// left, right, bottom, top, near, far
// having trouble with this z stuff







// we need window dimensions


// fetch events

// draw a frame


#[allow(missing_docs)]
#[rustc_copy_clone_marker]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::clone::Clone for Vertex {
    #[inline]
    fn clone(&self) -> Vertex {
        {
            let _: ::std::clone::AssertParamIsClone<[f32; 3]>;
            let _: ::std::clone::AssertParamIsClone<[f32; 3]>;
            let _: ::std::clone::AssertParamIsClone<[f32; 4]>;
            let _: ::std::clone::AssertParamIsClone<[f32; 3]>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::marker::Copy for Vertex {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::fmt::Debug for Vertex {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Vertex {
                position: ref __self_0_0,
                tex_coord: ref __self_0_1,
                color: ref __self_0_2,
                normal: ref __self_0_3,
            } => {
                let mut builder = __arg_0.debug_struct("Vertex");
                let _ = builder.field("position", &&(*__self_0_0));
                let _ = builder.field("tex_coord", &&(*__self_0_1));
                let _ = builder.field("color", &&(*__self_0_2));
                let _ = builder.field("normal", &&(*__self_0_3));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::cmp::PartialEq for Vertex {
    #[inline]
    fn eq(&self, __arg_0: &Vertex) -> bool {
        match *__arg_0 {
            Vertex {
                position: ref __self_1_0,
                tex_coord: ref __self_1_1,
                color: ref __self_1_2,
                normal: ref __self_1_3,
            } => {
                match *self {
                    Vertex {
                        position: ref __self_0_0,
                        tex_coord: ref __self_0_1,
                        color: ref __self_0_2,
                        normal: ref __self_0_3,
                    } => {
                        true && (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3)
                    }
                }
            }
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Vertex) -> bool {
        match *__arg_0 {
            Vertex {
                position: ref __self_1_0,
                tex_coord: ref __self_1_1,
                color: ref __self_1_2,
                normal: ref __self_1_3,
            } => {
                match *self {
                    Vertex {
                        position: ref __self_0_0,
                        tex_coord: ref __self_0_1,
                        color: ref __self_0_2,
                        normal: ref __self_0_3,
                    } => {
                        false || (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3)
                    }
                }
            }
        }
    }
}
unsafe impl ::traits::Pod for Vertex {}
impl ::pso::buffer::Structure<::format::Format> for Vertex {
    fn query(name: &str) -> ::std::option::Option<::pso::buffer::Element<::format::Format>> {
        use std::mem::{size_of, transmute};
        use pso::buffer::{Element, ElemOffset};
        let tmp: &Vertex = unsafe { transmute(1usize) };
        let base = tmp as *const _ as usize;
        let (sub_name, big_offset) = {
            let mut split = name.split(|c| c == '[' || c == ']');
            let _ = split.next().unwrap();
            match split.next() {
                Some(s) => {
                    let array_id: ElemOffset = s.parse().unwrap();
                    let sub_name = match split.next() {
                        Some(s) if s.starts_with('.') => &s[1..],
                        _ => name,
                    };
                    (sub_name, array_id * (size_of::<Vertex>() as ElemOffset))
                }
                None => (name, 0),
            }
        };
        match sub_name {
            "position" => {
                Some(Element {
                         format: <[f32; 3] as ::format::Formatted>::get_format(),
                         offset: (((&tmp.position as *const _ as usize) - base) as ElemOffset) +
                                 big_offset,
                     })
            }
            "tex_coord" => {
                Some(Element {
                         format: <[f32; 3] as ::format::Formatted>::get_format(),
                         offset: (((&tmp.tex_coord as *const _ as usize) - base) as ElemOffset) +
                                 big_offset,
                     })
            }
            "color" => {
                Some(Element {
                         format: <[f32; 4] as ::format::Formatted>::get_format(),
                         offset: (((&tmp.color as *const _ as usize) - base) as ElemOffset) +
                                 big_offset,
                     })
            }
            "normal" => {
                Some(Element {
                         format: <[f32; 3] as ::format::Formatted>::get_format(),
                         offset: (((&tmp.normal as *const _ as usize) - base) as ElemOffset) +
                                 big_offset,
                     })
            }
            _ => None,
        }
    }
}
#[allow(missing_docs)]
#[rustc_copy_clone_marker]
pub struct Locals {
    pub u_transform: [[f32; 4]; 4],
    pub u_alpha_minimum: f32,
    pub u_color: [f32; 4],
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::clone::Clone for Locals {
    #[inline]
    fn clone(&self) -> Locals {
        {
            let _: ::std::clone::AssertParamIsClone<[[f32; 4]; 4]>;
            let _: ::std::clone::AssertParamIsClone<f32>;
            let _: ::std::clone::AssertParamIsClone<[f32; 4]>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::marker::Copy for Locals {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::fmt::Debug for Locals {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Locals {
                u_transform: ref __self_0_0,
                u_alpha_minimum: ref __self_0_1,
                u_color: ref __self_0_2,
            } => {
                let mut builder = __arg_0.debug_struct("Locals");
                let _ = builder.field("u_transform", &&(*__self_0_0));
                let _ = builder.field("u_alpha_minimum", &&(*__self_0_1));
                let _ = builder.field("u_color", &&(*__self_0_2));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::cmp::PartialEq for Locals {
    #[inline]
    fn eq(&self, __arg_0: &Locals) -> bool {
        match *__arg_0 {
            Locals {
                u_transform: ref __self_1_0,
                u_alpha_minimum: ref __self_1_1,
                u_color: ref __self_1_2,
            } => {
                match *self {
                    Locals {
                        u_transform: ref __self_0_0,
                        u_alpha_minimum: ref __self_0_1,
                        u_color: ref __self_0_2,
                    } => {
                        true && (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2)
                    }
                }
            }
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Locals) -> bool {
        match *__arg_0 {
            Locals {
                u_transform: ref __self_1_0,
                u_alpha_minimum: ref __self_1_1,
                u_color: ref __self_1_2,
            } => {
                match *self {
                    Locals {
                        u_transform: ref __self_0_0,
                        u_alpha_minimum: ref __self_0_1,
                        u_color: ref __self_0_2,
                    } => {
                        false || (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2)
                    }
                }
            }
        }
    }
}
unsafe impl ::traits::Pod for Locals {}
impl ::pso::buffer::Structure<::shade::ConstFormat> for Locals {
    fn query(name: &str) -> ::std::option::Option<::pso::buffer::Element<::shade::ConstFormat>> {
        use std::mem::{size_of, transmute};
        use pso::buffer::{Element, ElemOffset};
        let tmp: &Locals = unsafe { transmute(1usize) };
        let base = tmp as *const _ as usize;
        let (sub_name, big_offset) = {
            let mut split = name.split(|c| c == '[' || c == ']');
            let _ = split.next().unwrap();
            match split.next() {
                Some(s) => {
                    let array_id: ElemOffset = s.parse().unwrap();
                    let sub_name = match split.next() {
                        Some(s) if s.starts_with('.') => &s[1..],
                        _ => name,
                    };
                    (sub_name, array_id * (size_of::<Locals>() as ElemOffset))
                }
                None => (name, 0),
            }
        };
        match sub_name {
            "u_transform" => {
                Some(Element {
                         format: <[[f32; 4]; 4] as ::shade::Formatted>::get_format(),
                         offset: (((&tmp.u_transform as *const _ as usize) - base) as
                                  ElemOffset) + big_offset,
                     })
            }
            "u_alpha_minimum" => {
                Some(Element {
                         format: <f32 as ::shade::Formatted>::get_format(),
                         offset: (((&tmp.u_alpha_minimum as *const _ as usize) - base) as
                                  ElemOffset) + big_offset,
                     })
            }
            "u_color" => {
                Some(Element {
                         format: <[f32; 4] as ::shade::Formatted>::get_format(),
                         offset: (((&tmp.u_color as *const _ as usize) - base) as ElemOffset) +
                                 big_offset,
                     })
            }
            _ => None,
        }
    }
}
#[allow(missing_docs)]
pub mod pipe_no_blend {
    #[allow(unused_imports)]
    use super::*;
    use super::gfx;
    use pso::{DataLink, DataBind, Descriptor, InitError, RawDataSet, AccessInfo};
    pub struct Data<R: ::Resources> {
        pub vbuf: <gfx::VertexBuffer<Vertex> as DataBind<R>>::Data,
        pub locals: <gfx::ConstantBuffer<Locals> as DataBind<R>>::Data,
        pub out_color:
            <gfx::RenderTarget<ColorFormat> as DataBind<R>>::Data,
        pub out_depth:
            <gfx::DepthTarget<DepthFormat> as DataBind<R>>::Data,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::std::clone::Clone + ::Resources> ::std::clone::Clone for Data<R> {
        #[inline]
        fn clone(&self) -> Data<R> {
            match *self {
                Data {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    Data {
                        vbuf: ::std::clone::Clone::clone(&(*__self_0_0)),
                        locals: ::std::clone::Clone::clone(&(*__self_0_1)),
                        out_color: ::std::clone::Clone::clone(&(*__self_0_2)),
                        out_depth: ::std::clone::Clone::clone(&(*__self_0_3)),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::std::fmt::Debug + ::Resources> ::std::fmt::Debug for Data<R> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Data {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    let mut builder = __arg_0.debug_struct("Data");
                    let _ = builder.field("vbuf", &&(*__self_0_0));
                    let _ = builder.field("locals", &&(*__self_0_1));
                    let _ = builder.field("out_color", &&(*__self_0_2));
                    let _ = builder.field("out_depth", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::std::cmp::PartialEq + ::Resources> ::std::cmp::PartialEq for Data<R> {
        #[inline]
        fn eq(&self, __arg_0: &Data<R>) -> bool {
            match *__arg_0 {
                Data {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Data {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3)
                        }
                    }
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Data<R>) -> bool {
            match *__arg_0 {
                Data {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Data {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3)
                        }
                    }
                }
            }
        }
    }
    pub struct Meta {
        vbuf: gfx::VertexBuffer<Vertex>,
        locals: gfx::ConstantBuffer<Locals>,
        out_color: gfx::RenderTarget<ColorFormat>,
        out_depth: gfx::DepthTarget<DepthFormat>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Meta {
        #[inline]
        fn clone(&self) -> Meta {
            match *self {
                Meta {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    Meta {
                        vbuf: ::std::clone::Clone::clone(&(*__self_0_0)),
                        locals: ::std::clone::Clone::clone(&(*__self_0_1)),
                        out_color: ::std::clone::Clone::clone(&(*__self_0_2)),
                        out_depth: ::std::clone::Clone::clone(&(*__self_0_3)),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Meta {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Meta {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    let mut builder = __arg_0.debug_struct("Meta");
                    let _ = builder.field("vbuf", &&(*__self_0_0));
                    let _ = builder.field("locals", &&(*__self_0_1));
                    let _ = builder.field("out_color", &&(*__self_0_2));
                    let _ = builder.field("out_depth", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Meta {
        #[inline]
        fn eq(&self, __arg_0: &Meta) -> bool {
            match *__arg_0 {
                Meta {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Meta {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3)
                        }
                    }
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Meta) -> bool {
            match *__arg_0 {
                Meta {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Meta {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3)
                        }
                    }
                }
            }
        }
    }
    pub struct Init<'a> {
        pub vbuf: <gfx::VertexBuffer<Vertex> as DataLink<'a>>::Init,
        pub locals: <gfx::ConstantBuffer<Locals> as DataLink<'a>>::Init,
        pub out_color:
            <gfx::RenderTarget<ColorFormat> as DataLink<'a>>::Init,
        pub out_depth:
            <gfx::DepthTarget<DepthFormat> as DataLink<'a>>::Init,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::clone::Clone for Init<'a> {
        #[inline]
        fn clone(&self) -> Init<'a> {
            match *self {
                Init {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    Init {
                        vbuf: ::std::clone::Clone::clone(&(*__self_0_0)),
                        locals: ::std::clone::Clone::clone(&(*__self_0_1)),
                        out_color: ::std::clone::Clone::clone(&(*__self_0_2)),
                        out_depth: ::std::clone::Clone::clone(&(*__self_0_3)),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::fmt::Debug for Init<'a> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Init {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    let mut builder = __arg_0.debug_struct("Init");
                    let _ = builder.field("vbuf", &&(*__self_0_0));
                    let _ = builder.field("locals", &&(*__self_0_1));
                    let _ = builder.field("out_color", &&(*__self_0_2));
                    let _ = builder.field("out_depth", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::cmp::PartialEq for Init<'a> {
        #[inline]
        fn eq(&self, __arg_0: &Init<'a>) -> bool {
            match *__arg_0 {
                Init {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Init {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3)
                        }
                    }
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Init<'a>) -> bool {
            match *__arg_0 {
                Init {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Init {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3)
                        }
                    }
                }
            }
        }
    }
    impl<'a> ::pso::PipelineInit for Init<'a> {
        type Meta = Meta;
        fn link_to<'s>(&self,
                       desc: &mut Descriptor,
                       info: &'s ::ProgramInfo)
                       -> ::std::result::Result<Self::Meta, InitError<&'s str>> {
            let mut meta =
                Meta{vbuf: <gfx::VertexBuffer<Vertex> as DataLink<'a>>::new(),
                     locals:
                         <gfx::ConstantBuffer<Locals> as DataLink<'a>>::new(),
                     out_color:
                         <gfx::RenderTarget<ColorFormat> as
                             DataLink<'a>>::new(),
                     out_depth:
                         <gfx::DepthTarget<DepthFormat> as
                             DataLink<'a>>::new(),};
            let mut _num_vb = 0;
            if let Some(d) = meta.vbuf.link_vertex_buffer(_num_vb, &self.vbuf) {
                if !meta.vbuf.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            if let Some(d) = meta.locals.link_vertex_buffer(_num_vb, &self.locals) {
                if !meta.locals.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            if let Some(d) = meta.out_color.link_vertex_buffer(_num_vb, &self.out_color) {
                if !meta.out_color.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            if let Some(d) = meta.out_depth.link_vertex_buffer(_num_vb, &self.out_depth) {
                if !meta.out_depth.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            for at in &info.vertex_attributes {
                match meta.vbuf.link_input(at, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                match meta.locals.link_input(at, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                match meta.out_color.link_input(at, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                match meta.out_depth.link_input(at, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                return Err(InitError::VertexImport(&at.name, None));
            }
            for cb in &info.constant_buffers {
                match meta.vbuf.link_constant_buffer(cb, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                match meta.locals.link_constant_buffer(cb, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                match meta.out_color.link_constant_buffer(cb, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                match meta.out_depth.link_constant_buffer(cb, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                return Err(InitError::ConstantBuffer(&cb.name, None));
            }
            for gc in &info.globals {
                match meta.vbuf.link_global_constant(gc, &self.vbuf) {
                    Some(Ok(())) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                match meta.locals.link_global_constant(gc, &self.locals) {
                    Some(Ok(())) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                match meta.out_color.link_global_constant(gc, &self.out_color) {
                    Some(Ok(())) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                match meta.out_depth.link_global_constant(gc, &self.out_depth) {
                    Some(Ok(())) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                return Err(InitError::GlobalConstant(&gc.name, None));
            }
            for srv in &info.textures {
                match meta.vbuf.link_resource_view(srv, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                match meta.locals.link_resource_view(srv, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                match meta.out_color.link_resource_view(srv, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                match meta.out_depth.link_resource_view(srv, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                return Err(InitError::ResourceView(&srv.name, None));
            }
            for uav in &info.unordereds {
                match meta.vbuf.link_unordered_view(uav, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                match meta.locals.link_unordered_view(uav, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                match meta.out_color.link_unordered_view(uav, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                match meta.out_depth.link_unordered_view(uav, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                return Err(InitError::UnorderedView(&uav.name, None));
            }
            for sm in &info.samplers {
                match meta.vbuf.link_sampler(sm, &self.vbuf) {
                    Some(d) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                match meta.locals.link_sampler(sm, &self.locals) {
                    Some(d) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                match meta.out_color.link_sampler(sm, &self.out_color) {
                    Some(d) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                match meta.out_depth.link_sampler(sm, &self.out_depth) {
                    Some(d) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                return Err(InitError::Sampler(&sm.name, None));
            }
            for out in &info.outputs {
                match meta.vbuf.link_output(out, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                match meta.locals.link_output(out, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                match meta.out_color.link_output(out, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                match meta.out_depth.link_output(out, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                return Err(InitError::PixelExport(&out.name, None));
            }
            if !info.knows_outputs {
                use shade::core as s;
                let mut out = s::OutputVar {
                    name: String::new(),
                    slot: 0,
                    base_type: s::BaseType::F32,
                    container: s::ContainerType::Vector(4),
                };
                match meta.vbuf.link_output(&out, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
                match meta.locals.link_output(&out, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
                match meta.out_color.link_output(&out, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
                match meta.out_depth.link_output(&out, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
            }
            for _ in 0..1 {
                if let Some(d) = meta.vbuf.link_depth_stencil(&self.vbuf) {
                    if !meta.vbuf.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.vbuf.link_scissor() {
                    if !meta.vbuf.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
                if let Some(d) = meta.locals.link_depth_stencil(&self.locals) {
                    if !meta.locals.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.locals.link_scissor() {
                    if !meta.locals.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
                if let Some(d) = meta.out_color.link_depth_stencil(&self.out_color) {
                    if !meta.out_color.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.out_color.link_scissor() {
                    if !meta.out_color.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
                if let Some(d) = meta.out_depth.link_depth_stencil(&self.out_depth) {
                    if !meta.out_depth.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.out_depth.link_scissor() {
                    if !meta.out_depth.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
            }
            Ok(meta)
        }
    }
    impl<R: ::Resources> ::pso::PipelineData<R> for Data<R> {
        type Meta = Meta;
        fn bake_to(&self,
                   out: &mut RawDataSet<R>,
                   meta: &Self::Meta,
                   man: &mut ::handle::Manager<R>,
                   access: &mut AccessInfo<R>) {
            meta.vbuf.bind_to(out, &self.vbuf, man, access);
            meta.locals.bind_to(out, &self.locals, man, access);
            meta.out_color.bind_to(out, &self.out_color, man, access);
            meta.out_depth.bind_to(out, &self.out_depth, man, access);
        }
    }
    pub fn new() -> Init<'static> {
        Init {
            vbuf: (),
            locals: "Locals",
            out_color: "Target0",
            out_depth: gfx::preset::depth::LESS_EQUAL_WRITE,
        }
    }
}
#[allow(missing_docs)]
pub mod pipe_blend {
    #[allow(unused_imports)]
    use super::*;
    use super::gfx;
    use pso::{DataLink, DataBind, Descriptor, InitError, RawDataSet, AccessInfo};
    pub struct Data<R: ::Resources> {
        pub vbuf: <gfx::VertexBuffer<Vertex> as DataBind<R>>::Data,
        pub locals: <gfx::ConstantBuffer<Locals> as DataBind<R>>::Data,
        pub out_color:
            <gfx::BlendTarget<ColorFormat> as DataBind<R>>::Data,
        pub out_depth:
            <gfx::DepthTarget<DepthFormat> as DataBind<R>>::Data,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::std::clone::Clone + ::Resources> ::std::clone::Clone for Data<R> {
        #[inline]
        fn clone(&self) -> Data<R> {
            match *self {
                Data {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    Data {
                        vbuf: ::std::clone::Clone::clone(&(*__self_0_0)),
                        locals: ::std::clone::Clone::clone(&(*__self_0_1)),
                        out_color: ::std::clone::Clone::clone(&(*__self_0_2)),
                        out_depth: ::std::clone::Clone::clone(&(*__self_0_3)),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::std::fmt::Debug + ::Resources> ::std::fmt::Debug for Data<R> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Data {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    let mut builder = __arg_0.debug_struct("Data");
                    let _ = builder.field("vbuf", &&(*__self_0_0));
                    let _ = builder.field("locals", &&(*__self_0_1));
                    let _ = builder.field("out_color", &&(*__self_0_2));
                    let _ = builder.field("out_depth", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::std::cmp::PartialEq + ::Resources> ::std::cmp::PartialEq for Data<R> {
        #[inline]
        fn eq(&self, __arg_0: &Data<R>) -> bool {
            match *__arg_0 {
                Data {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Data {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3)
                        }
                    }
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Data<R>) -> bool {
            match *__arg_0 {
                Data {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Data {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3)
                        }
                    }
                }
            }
        }
    }
    pub struct Meta {
        vbuf: gfx::VertexBuffer<Vertex>,
        locals: gfx::ConstantBuffer<Locals>,
        out_color: gfx::BlendTarget<ColorFormat>,
        out_depth: gfx::DepthTarget<DepthFormat>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Meta {
        #[inline]
        fn clone(&self) -> Meta {
            match *self {
                Meta {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    Meta {
                        vbuf: ::std::clone::Clone::clone(&(*__self_0_0)),
                        locals: ::std::clone::Clone::clone(&(*__self_0_1)),
                        out_color: ::std::clone::Clone::clone(&(*__self_0_2)),
                        out_depth: ::std::clone::Clone::clone(&(*__self_0_3)),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Meta {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Meta {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    let mut builder = __arg_0.debug_struct("Meta");
                    let _ = builder.field("vbuf", &&(*__self_0_0));
                    let _ = builder.field("locals", &&(*__self_0_1));
                    let _ = builder.field("out_color", &&(*__self_0_2));
                    let _ = builder.field("out_depth", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Meta {
        #[inline]
        fn eq(&self, __arg_0: &Meta) -> bool {
            match *__arg_0 {
                Meta {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Meta {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3)
                        }
                    }
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Meta) -> bool {
            match *__arg_0 {
                Meta {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Meta {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3)
                        }
                    }
                }
            }
        }
    }
    pub struct Init<'a> {
        pub vbuf: <gfx::VertexBuffer<Vertex> as DataLink<'a>>::Init,
        pub locals: <gfx::ConstantBuffer<Locals> as DataLink<'a>>::Init,
        pub out_color:
            <gfx::BlendTarget<ColorFormat> as DataLink<'a>>::Init,
        pub out_depth:
            <gfx::DepthTarget<DepthFormat> as DataLink<'a>>::Init,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::clone::Clone for Init<'a> {
        #[inline]
        fn clone(&self) -> Init<'a> {
            match *self {
                Init {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    Init {
                        vbuf: ::std::clone::Clone::clone(&(*__self_0_0)),
                        locals: ::std::clone::Clone::clone(&(*__self_0_1)),
                        out_color: ::std::clone::Clone::clone(&(*__self_0_2)),
                        out_depth: ::std::clone::Clone::clone(&(*__self_0_3)),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::fmt::Debug for Init<'a> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Init {
                    vbuf: ref __self_0_0,
                    locals: ref __self_0_1,
                    out_color: ref __self_0_2,
                    out_depth: ref __self_0_3,
                } => {
                    let mut builder = __arg_0.debug_struct("Init");
                    let _ = builder.field("vbuf", &&(*__self_0_0));
                    let _ = builder.field("locals", &&(*__self_0_1));
                    let _ = builder.field("out_color", &&(*__self_0_2));
                    let _ = builder.field("out_depth", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::cmp::PartialEq for Init<'a> {
        #[inline]
        fn eq(&self, __arg_0: &Init<'a>) -> bool {
            match *__arg_0 {
                Init {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Init {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3)
                        }
                    }
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Init<'a>) -> bool {
            match *__arg_0 {
                Init {
                    vbuf: ref __self_1_0,
                    locals: ref __self_1_1,
                    out_color: ref __self_1_2,
                    out_depth: ref __self_1_3,
                } => {
                    match *self {
                        Init {
                            vbuf: ref __self_0_0,
                            locals: ref __self_0_1,
                            out_color: ref __self_0_2,
                            out_depth: ref __self_0_3,
                        } => {
                            false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3)
                        }
                    }
                }
            }
        }
    }
    impl<'a> ::pso::PipelineInit for Init<'a> {
        type Meta = Meta;
        fn link_to<'s>(&self,
                       desc: &mut Descriptor,
                       info: &'s ::ProgramInfo)
                       -> ::std::result::Result<Self::Meta, InitError<&'s str>> {
            let mut meta =
                Meta{vbuf: <gfx::VertexBuffer<Vertex> as DataLink<'a>>::new(),
                     locals:
                         <gfx::ConstantBuffer<Locals> as DataLink<'a>>::new(),
                     out_color:
                         <gfx::BlendTarget<ColorFormat> as
                             DataLink<'a>>::new(),
                     out_depth:
                         <gfx::DepthTarget<DepthFormat> as
                             DataLink<'a>>::new(),};
            let mut _num_vb = 0;
            if let Some(d) = meta.vbuf.link_vertex_buffer(_num_vb, &self.vbuf) {
                if !meta.vbuf.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            if let Some(d) = meta.locals.link_vertex_buffer(_num_vb, &self.locals) {
                if !meta.locals.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            if let Some(d) = meta.out_color.link_vertex_buffer(_num_vb, &self.out_color) {
                if !meta.out_color.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            if let Some(d) = meta.out_depth.link_vertex_buffer(_num_vb, &self.out_depth) {
                if !meta.out_depth.is_active() {
                    {
                        ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                            static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                            &_FILE_LINE
                        })
                    }
                };
                desc.vertex_buffers[_num_vb as usize] = Some(d);
                _num_vb += 1;
            }
            for at in &info.vertex_attributes {
                match meta.vbuf.link_input(at, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                match meta.locals.link_input(at, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                match meta.out_color.link_input(at, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                match meta.out_depth.link_input(at, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => (),
                }
                return Err(InitError::VertexImport(&at.name, None));
            }
            for cb in &info.constant_buffers {
                match meta.vbuf.link_constant_buffer(cb, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                match meta.locals.link_constant_buffer(cb, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                match meta.out_color.link_constant_buffer(cb, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                match meta.out_depth.link_constant_buffer(cb, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.constant_buffers[cb.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(e)) => return Err(InitError::ConstantBuffer(&cb.name, Some(e))),
                    None => (),
                }
                return Err(InitError::ConstantBuffer(&cb.name, None));
            }
            for gc in &info.globals {
                match meta.vbuf.link_global_constant(gc, &self.vbuf) {
                    Some(Ok(())) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                match meta.locals.link_global_constant(gc, &self.locals) {
                    Some(Ok(())) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                match meta.out_color.link_global_constant(gc, &self.out_color) {
                    Some(Ok(())) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                match meta.out_depth.link_global_constant(gc, &self.out_depth) {
                    Some(Ok(())) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()",
                                                  {
                                                      static _FILE_LINE:
                                                             (&'static str,
                                                              u32) =
                                                          ("src/lib.rs",
                                                           16u32);
                                                      &_FILE_LINE
                                                  })
                            }
                        };
                        continue ;
                    }
                    Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                    None => (),
                }
                return Err(InitError::GlobalConstant(&gc.name, None));
            }
            for srv in &info.textures {
                match meta.vbuf.link_resource_view(srv, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                match meta.locals.link_resource_view(srv, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                match meta.out_color.link_resource_view(srv, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                match meta.out_depth.link_resource_view(srv, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.resource_views[srv.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => (),
                }
                return Err(InitError::ResourceView(&srv.name, None));
            }
            for uav in &info.unordereds {
                match meta.vbuf.link_unordered_view(uav, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                match meta.locals.link_unordered_view(uav, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                match meta.out_color.link_unordered_view(uav, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                match meta.out_depth.link_unordered_view(uav, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.unordered_views[uav.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(_)) => return Err(InitError::UnorderedView(&uav.name, Some(()))),
                    None => (),
                }
                return Err(InitError::UnorderedView(&uav.name, None));
            }
            for sm in &info.samplers {
                match meta.vbuf.link_sampler(sm, &self.vbuf) {
                    Some(d) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                match meta.locals.link_sampler(sm, &self.locals) {
                    Some(d) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                match meta.out_color.link_sampler(sm, &self.out_color) {
                    Some(d) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                match meta.out_depth.link_sampler(sm, &self.out_depth) {
                    Some(d) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.samplers[sm.slot as usize] = Some(d);
                        continue;
                    }
                    None => (),
                }
                return Err(InitError::Sampler(&sm.name, None));
            }
            for out in &info.outputs {
                match meta.vbuf.link_output(out, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                match meta.locals.link_output(out, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                match meta.out_color.link_output(out, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                match meta.out_depth.link_output(out, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => (),
                }
                return Err(InitError::PixelExport(&out.name, None));
            }
            if !info.knows_outputs {
                use shade::core as s;
                let mut out = s::OutputVar {
                    name: String::new(),
                    slot: 0,
                    base_type: s::BaseType::F32,
                    container: s::ContainerType::Vector(4),
                };
                match meta.vbuf.link_output(&out, &self.vbuf) {
                    Some(Ok(d)) => {
                        if !meta.vbuf.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
                match meta.locals.link_output(&out, &self.locals) {
                    Some(Ok(d)) => {
                        if !meta.locals.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
                match meta.out_color.link_output(&out, &self.out_color) {
                    Some(Ok(d)) => {
                        if !meta.out_color.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
                match meta.out_depth.link_output(&out, &self.out_depth) {
                    Some(Ok(d)) => {
                        if !meta.out_depth.is_active() {
                            {
                                ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                    static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                    &_FILE_LINE
                                })
                            }
                        };
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
            }
            for _ in 0..1 {
                if let Some(d) = meta.vbuf.link_depth_stencil(&self.vbuf) {
                    if !meta.vbuf.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.vbuf.link_scissor() {
                    if !meta.vbuf.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.vbuf.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
                if let Some(d) = meta.locals.link_depth_stencil(&self.locals) {
                    if !meta.locals.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.locals.link_scissor() {
                    if !meta.locals.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.locals.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
                if let Some(d) = meta.out_color.link_depth_stencil(&self.out_color) {
                    if !meta.out_color.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.out_color.link_scissor() {
                    if !meta.out_color.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_color.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
                if let Some(d) = meta.out_depth.link_depth_stencil(&self.out_depth) {
                    if !meta.out_depth.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.depth_stencil = Some(d);
                }
                if meta.out_depth.link_scissor() {
                    if !meta.out_depth.is_active() {
                        {
                            ::rt::begin_panic("assertion failed: meta.out_depth.is_active()", {
                                static _FILE_LINE: (&'static str, u32) = ("src/lib.rs", 16u32);
                                &_FILE_LINE
                            })
                        }
                    };
                    desc.scissor = true;
                }
            }
            Ok(meta)
        }
    }
    impl<R: ::Resources> ::pso::PipelineData<R> for Data<R> {
        type Meta = Meta;
        fn bake_to(&self,
                   out: &mut RawDataSet<R>,
                   meta: &Self::Meta,
                   man: &mut ::handle::Manager<R>,
                   access: &mut AccessInfo<R>) {
            meta.vbuf.bind_to(out, &self.vbuf, man, access);
            meta.locals.bind_to(out, &self.locals, man, access);
            meta.out_color.bind_to(out, &self.out_color, man, access);
            meta.out_depth.bind_to(out, &self.out_depth, man, access);
        }
    }
    pub fn new() -> Init<'static> {
        Init {
            vbuf: (),
            locals: "Locals",
            out_color: ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
            out_depth: gfx::preset::depth::LESS_EQUAL_WRITE,
        }
    }
}
const TEX_COORD: [f32; 3] = [0.0, 0.0, 0.0];
const NORMAL: [f32; 3] = [1.0, 0.0, 0.0];
const TRIANGLE: [Vertex; 3] = [Vertex {
                                   position: [-0.5, -0.5, 0.0],
                                   tex_coord: TEX_COORD,
                                   color: [1.0, 0.0, 0.0, 1.0],
                                   normal: NORMAL,
                               },
                               Vertex {
                                   position: [0.5, -0.5, 0.0],
                                   tex_coord: TEX_COORD,
                                   color: [0.0, 1.0, 0.0, 1.0],
                                   normal: NORMAL,
                               },
                               Vertex {
                                   position: [0.0, 0.5, 0.0],
                                   tex_coord: TEX_COORD,
                                   color: [0.0, 0.0, 1.0, 1.0],
                                   normal: NORMAL,
                               }];
pub fn ui_projection(width: f32, height: f32) -> cgmath::Matrix4<f32> {
    cgmath::ortho(0.0, width, 0.0, height, -100.0, 100.0)
}
const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
pub fn run_example() {
    ::io::_print(::std::fmt::Arguments::new_v1({
        static __STATIC_FMTSTR: &'static [&'static str] = &["booting up\n"];
        __STATIC_FMTSTR
    },
                                               &match () {
                                                   () => [],
                                               }));
    let width = 1024;
    let height = 768;
    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(width, height);
    use glutin::{GlRequest, Api};
    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true);
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    ::io::_print(::std::fmt::Arguments::new_v1({
        static __STATIC_FMTSTR: &'static [&'static str] = &["pre pso\n"];
        __STATIC_FMTSTR
    },
                                               &match () {
                                                   () => [],
                                               }));
    let m_pso =
        factory.create_pipeline_simple(b"#version 330\n\nlayout (std140)\nuniform Locals {\n\tmat4 u_transform;\n\tfloat u_alpha_minimum;\n\tvec4 u_color;\n};\n\nin vec3 position;\nin vec3 tex_coord;\nin vec4 color;\nin vec3 normal;\n\nout vec4 v_color;\nout vec3 v_tex_coord;\nout vec3 v_normal;\n\nvoid main() {\n    gl_Position = u_transform * vec4(position, 1.0);\n    v_color = color * u_color;\n    v_tex_coord = tex_coord;\n    v_normal = normal; \n}\n",
                                       b"#version 330\n\nlayout (std140)\nuniform Locals {\n\tmat4 u_transform;\n\tfloat u_alpha_minimum;\n\tvec4 u_color;\n};\n\nin vec4 v_color;\nin vec3 v_tex_coord;\nin vec3 v_normal;\n\nout vec4 f_color;\n\nvoid main() {\n    vec4 albedo_colour = v_color;\n\n    vec4 final_colour = albedo_colour; // * light;\n    final_colour.a = albedo_colour.a; // ignore light\'s alpha\n\n    if(final_colour.a < u_alpha_minimum) {\n        discard;\n    }\n    f_color = final_colour;\n}\n\n\n",
                                       pipe_no_blend::new());
    ::io::_print(::std::fmt::Arguments::new_v1({
        static __STATIC_FMTSTR: &'static [&'static str] = &["pso res -> ", "\n"];
        __STATIC_FMTSTR
    },
                                               &match (&m_pso,) {
                                                   (__arg0,) =>
                                                    [::std::fmt::ArgumentV1::new(__arg0,
                                                                                 ::std::fmt::Debug::fmt)],
                                               }));
    ::io::_print(::std::fmt::Arguments::new_v1({
        static __STATIC_FMTSTR: &'static [&'static str] = &["pso done\n"];
        __STATIC_FMTSTR
    },
                                               &match () {
                                                   () => [],
                                               }));
    let pso = m_pso.unwrap();
    let transform = ui_projection(width as f32, height as f32);
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut data = pipe_no_blend::Data {
        vbuf: vertex_buffer,
        locals: factory.create_constant_buffer(1),
        out_color: main_color,
        out_depth: main_depth,
    };
    let mut running = true;
    while running {
        events_loop.poll_events(|event|
                                    {
                                        if let glutin::Event::WindowEvent {
                                               event, .. } = event {
                                            match event {
                                                glutin::WindowEvent::KeyboardInput {
                                                input: glutin::KeyboardInput {
                                                    virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                                    ..
                                                    }, .. } |
                                                glutin::WindowEvent::Closed =>
                                                {
                                                    ::io::_print(::std::fmt::Arguments::new_v1({
                                                                                                   static __STATIC_FMTSTR:
                                                                                                          &'static [&'static str]
                                                                                                          =
                                                                                                       &["closing, time to stop running\n"];
                                                                                                   __STATIC_FMTSTR
                                                                                               },
                                                                                               &match ()
                                                                                                    {
                                                                                                    ()
                                                                                                    =>
                                                                                                    [],
                                                                                                }));
                                                    running = false
                                                }
                                                glutin::WindowEvent::Resized(width,
                                                                             height)
                                                => {
                                                    window.resize(width,
                                                                  height);
                                                    gfx_window_glutin::update_views(&window,
                                                                                    &mut data.out_color,
                                                                                    &mut data.out_depth);
                                                }
                                                _ => (),
                                            }
                                        }
                                    });
        let locals = Locals {
            u_transform: transform.into(),
            u_alpha_minimum: 0.1,
            u_color: [1.0, 0.0, 0.0, 1.0],
        };
        encoder.update_constant_buffer(&data.locals, &locals);
        encoder.clear(&data.out_color, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
