// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Vec2;
use glib::translate::*;

impl Vec2 {
    #[doc(alias = "graphene_vec2_init")]
    pub fn new(x: f32, y: f32) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec2::uninitialized();
            ffi::graphene_vec2_init(vec.to_glib_none_mut().0, x, y);
            vec
        }
    }

    #[doc(alias = "graphene_vec2_init_from_vec2")]
    #[doc(alias = "new_from_vec2")]
    pub fn from_vec2(src: &Vec2) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec2::uninitialized();
            ffi::graphene_vec2_init_from_vec2(vec.to_glib_none_mut().0, src.to_glib_none().0);
            vec
        }
    }

    #[doc(alias = "graphene_vec2_init_from_float")]
    #[doc(alias = "new_from_float")]
    pub fn from_float(src: &[f32; 2]) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec2::uninitialized();
            ffi::graphene_vec2_init_from_float(vec.to_glib_none_mut().0, src.as_ptr() as *const _);
            vec
        }
    }

    #[doc(alias = "graphene_vec2_to_float")]
    pub fn to_float(&self) -> [f32; 2] {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            ffi::graphene_vec2_to_float(self.to_glib_none().0, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
