// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Plane;
use crate::Point3D;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

impl Plane {
    #[doc(alias = "graphene_plane_init")]
    pub fn new(normal: Option<&Vec3>, constant: f32) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let mut plane = Plane::uninitialized();
            ffi::graphene_plane_init(
                plane.to_glib_none_mut().0,
                normal.to_glib_none().0,
                constant,
            );
            plane
        }
    }

    #[doc(alias = "graphene_plane_init_from_plane")]
    #[doc(alias = "new_from_plane")]
    pub fn from_plane(src: &Plane) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let mut plane = Plane::uninitialized();
            ffi::graphene_plane_init_from_plane(plane.to_glib_none_mut().0, src.to_glib_none().0);
            plane
        }
    }

    #[doc(alias = "graphene_plane_init_from_point")]
    #[doc(alias = "new_from_point")]
    pub fn from_point(normal: &Vec3, point: &Point3D) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let mut plane = Plane::uninitialized();
            ffi::graphene_plane_init_from_point(
                plane.to_glib_none_mut().0,
                normal.to_glib_none().0,
                point.to_glib_none().0,
            );
            plane
        }
    }

    #[doc(alias = "graphene_plane_init_from_points")]
    #[doc(alias = "new_from_points")]
    pub fn from_points(a: &Point3D, b: &Point3D, c: &Point3D) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let mut plane = Plane::uninitialized();
            ffi::graphene_plane_init_from_points(
                plane.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            plane
        }
    }

    #[doc(alias = "graphene_plane_init_from_vec4")]
    #[doc(alias = "new_from_vec4")]
    pub fn from_vec4(src: &Vec4) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let mut plane = Plane::uninitialized();
            ffi::graphene_plane_init_from_vec4(plane.to_glib_none_mut().0, src.to_glib_none().0);
            plane
        }
    }
}
