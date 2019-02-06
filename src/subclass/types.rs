// Copyright 2017-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Module that contains the basic infrastructure for subclassing `GObject`.

use ffi;
use gobject_ffi;

use std::marker;
use std::mem;
use std::ptr;

use object::{ObjectExt, ObjectType};
use translate::*;
use {Closure, IsA, IsClassFor, SignalFlags, StaticType, Type, Value};

use super::object::ObjectImpl;

/// A newly registered `glib::Type` that is currently still being initialized.
///
/// This allows running additional type-setup functions, e.g. for implementing
/// interfaces on the type.
#[derive(Debug, PartialEq, Eq)]
pub struct InitializingType<T>(pub(crate) Type, pub(crate) marker::PhantomData<T>);

impl<T: ObjectSubclass> InitializingType<T> {
    /// Adds an interface implementation for `I` to the type.
    pub fn add_interface<I: IsImplementable<T>>(&mut self) {
        unsafe {
            let iface_info = gobject_ffi::GInterfaceInfo {
                interface_init: Some(I::interface_init),
                interface_finalize: None,
                interface_data: ptr::null_mut(),
            };
            gobject_ffi::g_type_add_interface_static(
                self.0.to_glib(),
                I::static_type().to_glib(),
                &iface_info,
            );
        }
    }
}

impl<T> ToGlib for InitializingType<T> {
    type GlibType = ffi::GType;

    fn to_glib(&self) -> ffi::GType {
        self.0.to_glib()
    }
}

/// Trait implemented by structs that implement a `GObject` C instance struct.
///
/// The struct must be `#[repr(C)]` and have the parent type's instance struct
/// as the first field.
///
/// See [`simple::InstanceStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the instance struct.
///
/// [`simple::InstanceStruct`]: ../simple/struct.InstanceStruct.html
pub unsafe trait InstanceStruct: Sized + 'static {
    /// Corresponding object subclass type for this instance struct.
    type Type: ObjectSubclass;

    /// Returns the implementation for from this instance struct, that
    /// is the implementor of [`ObjectImpl`] or subtraits.
    ///
    /// [`ObjectImpl`]: ../object/trait.ObjectImpl.html
    fn get_impl(&self) -> &Self::Type {
        unsafe {
            let data = Self::Type::type_data();
            let private_offset = data.as_ref().private_offset;
            let ptr: *const u8 = self as *const _ as *const u8;
            let priv_ptr = ptr.offset(private_offset);
            let imp = priv_ptr as *const Option<Self::Type>;

            (*imp).as_ref().expect("No private struct")
        }
    }

    /// Returns the class struct for this specific instance.
    fn get_class(&self) -> &<Self::Type as ObjectSubclass>::Class {
        unsafe { &**(self as *const _ as *const *const <Self::Type as ObjectSubclass>::Class) }
    }
}

/// Trait implemented by structs that implement a `GObject` C class struct.
///
/// The struct must be `#[repr(C)]` and have the parent type's class struct
/// as the first field.
///
/// See [`simple::ClassStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the class struct, e.g. for declaring new virtual methods.
///
/// [`simple::ClassStruct`]: ../simple/struct.ClassStruct.html
pub unsafe trait ClassStruct: Sized + 'static {
    /// Corresponding object subclass type for this class struct.
    type Type: ObjectSubclass;

    /// Override the vfuncs of all parent types.
    ///
    /// This is automatically called during type initialization.
    fn override_vfuncs(&mut self)
    where
        <<Self::Type as ObjectSubclass>::ParentType as ObjectType>::RustClassType:
            IsSubclassable<Self::Type>,
    {
        unsafe {
            let base = &mut *(self as *mut _
                as *mut <<Self::Type as ObjectSubclass>::ParentType as ObjectType>::RustClassType);
            base.override_vfuncs();
        }
    }
}

/// Trait for subclassable class structs.
pub unsafe trait IsSubclassable<T: ObjectSubclass>: IsClassFor {
    /// Override the virtual methods of this class for the given subclass.
    ///
    /// This is automatically called during type initialization.
    fn override_vfuncs(&mut self);
}

/// Trait for implementable interfaces.
pub unsafe trait IsImplementable<T: ObjectSubclass>: StaticType {
    /// Initializes the interface's virtual methods.
    unsafe extern "C" fn interface_init(iface: ffi::gpointer, _iface_data: ffi::gpointer);
}

/// Type-specific data that is filled in during type creation.
pub struct TypeData {
    #[doc(hidden)]
    pub type_: Type,
    #[doc(hidden)]
    pub parent_class: ffi::gpointer,
    #[doc(hidden)]
    pub interface_data: *const Vec<(ffi::GType, ffi::gpointer)>,
    #[doc(hidden)]
    pub private_offset: isize,
}

unsafe impl Send for TypeData {}
unsafe impl Sync for TypeData {}

impl TypeData {
    /// Returns the type ID.
    pub fn get_type(&self) -> Type {
        self.type_
    }

    /// Returns a pointer to the native parent class.
    ///
    /// This is used for chaining up to the parent class' implementation
    /// of virtual methods.
    pub fn get_parent_class(&self) -> ffi::gpointer {
        self.parent_class
    }

    /// Returns a pointer to the interface implementation specific data.
    ///
    /// This is used for interface implementations to store additional data.
    pub fn get_interface_data(&self, type_: ffi::GType) -> ffi::gpointer {
        unsafe {
            if self.interface_data.is_null() {
                return ptr::null_mut();
            }

            for &(t, p) in &(*self.interface_data) {
                if t == type_ {
                    return p;
                }
            }

            ptr::null_mut()
        }
    }

    /// Returns the offset of the private struct in bytes relative to the
    /// beginning of the instance struct.
    pub fn get_private_offset(&self) -> isize {
        self.private_offset
    }
}

#[macro_export]
/// Macro for boilerplate of [`ObjectSubclass`] implementations.
///
/// [`ObjectSubclass`]: subclass/types/trait.ObjectSubclass.html
macro_rules! glib_object_subclass {
    () => {
        fn type_data() -> ::std::ptr::NonNull<$crate::subclass::TypeData> {
            static mut DATA: $crate::subclass::TypeData = $crate::subclass::TypeData {
                type_: $crate::Type::Invalid,
                parent_class: ::std::ptr::null_mut(),
                interface_data: ::std::ptr::null_mut(),
                private_offset: 0,
            };

            unsafe { ::std::ptr::NonNull::new_unchecked(&mut DATA) }
        }

        fn get_type() -> $crate::Type {
            static ONCE: ::std::sync::Once = ::std::sync::Once::new();

            ONCE.call_once(|| {
                $crate::subclass::register_type::<Self>();
            });

            unsafe {
                let data = Self::type_data();
                let type_ = data.as_ref().get_type();
                assert_ne!(type_, $crate::Type::Invalid);

                type_
            }
        }
    };
}

/// The central trait for subclassing a `GObject` type.
///
/// Links together the type name, parent type and the instance and
/// class structs for type registration and allows subclasses to
/// hook into various steps of the type registration and initialization.
///
/// See [`register_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_type`]: fn.register_type.html
pub trait ObjectSubclass: ObjectImpl + Sized + 'static {
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// If this subclass is an abstract class or not.
    ///
    /// By default all subclasses are non-abstract types but setting this to `true` will create an
    /// abstract class instead.
    ///
    /// Abstract classes can't be instantiated and require a non-abstract subclass.
    ///
    /// Optional.
    const ABSTRACT: bool = false;

    /// Parent Rust type to inherit from.
    type ParentType: ObjectType
        + FromGlibPtrBorrow<*mut <Self::ParentType as ObjectType>::GlibType>
        + FromGlibPtrNone<*mut <Self::ParentType as ObjectType>::GlibType>;

    /// The C instance struct.
    ///
    /// See [`simple::InstanceStruct`] for an basic instance struct that should be
    /// used in most cases.
    ///
    /// [`simple::InstanceStruct`]: ../simple/struct.InstanceStruct.html
    // TODO: Should default to simple::InstanceStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Instance: InstanceStruct<Type = Self>;

    /// The C class struct.
    ///
    /// See [`simple::ClassStruct`] for an basic instance struct that should be
    /// used in most cases.
    ///
    /// [`simple::ClassStruct`]: ../simple/struct.ClassStruct.html
    // TODO: Should default to simple::ClassStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Class: ClassStruct<Type = Self>;

    /// Storage for the type-specific data used during registration.
    ///
    /// This is usually generated by the [`glib_object_subclass!`] macro.
    ///
    /// [`glib_object_subclass!`]: ../../macro.glib_object_subclass.html
    fn type_data() -> ptr::NonNull<TypeData>;

    /// Returns the `glib::Type` ID of the subclass.
    ///
    /// This will register the type with the type system on the first call and is usually generated
    /// by the [`glib_object_subclass!`] macro.
    ///
    /// [`glib_object_subclass!`]: ../../macro.glib_object_subclass.html
    fn get_type() -> Type;

    /// Returns the corresponding object instance.
    fn get_instance(&self) -> Self::ParentType {
        unsafe {
            let data = Self::type_data();
            let type_ = data.as_ref().get_type();
            assert_ne!(type_, Type::Invalid);

            let offset = -data.as_ref().private_offset;
            assert_ne!(offset, 0);

            let ptr = self as *const Self as *const u8;
            let ptr = ptr.offset(offset);
            let ptr = ptr as *mut u8 as *mut <Self::ParentType as ObjectType>::GlibType;

            from_glib_none(ptr)
        }
    }

    /// Returns the implementation from an instance.
    ///
    /// Panics if called on an object of the wrong type.
    fn from_instance<T: IsA<Self::ParentType>>(obj: &T) -> &Self {
        unsafe {
            let data = Self::type_data();
            let type_ = data.as_ref().get_type();
            assert_ne!(type_, Type::Invalid);

            assert!(obj.get_type().is_a(&type_));

            let ptr = obj.as_ptr() as *const Self::Instance;
            (*ptr).get_impl()
        }
    }

    /// Additional type initialization.
    ///
    /// This is called right after the type was registered and allows
    /// subclasses to do additional type-specific initialization, e.g.
    /// for implementing `GObject` interfaces.
    ///
    /// Optional
    fn type_init(_type_: &mut InitializingType<Self>) {}

    /// Class initialization.
    ///
    /// This is called after `type_init` and before the first instance
    /// of the subclass is created. Subclasses can use this to do class-
    /// specific initialization, e.g. for installing properties or signals
    /// on the class or calling class methods.
    ///
    /// Optional
    fn class_init(_klass: &mut Self::Class) {}

    /// Constructor.
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    ///
    /// Optional, either implement this or `new_with_class()`.
    fn new() -> Self {
        unimplemented!();
    }

    /// Constructor.
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    ///
    /// Different to `new()` above it also gets the class of this type passed
    /// to itself for providing additional context.
    ///
    /// Optional, either implement this or `new()`.
    fn new_with_class(_klass: &Self::Class) -> Self {
        Self::new()
    }
}

unsafe extern "C" fn class_init<T: ObjectSubclass>(klass: ffi::gpointer, _klass_data: ffi::gpointer)
where
    <<T as ObjectSubclass>::ParentType as ObjectType>::RustClassType: IsSubclassable<T>,
{
    let mut data = T::type_data();

    // We have to update the private struct offset once the class is actually
    // being initialized.
    {
        let mut private_offset = data.as_ref().private_offset as i32;
        gobject_ffi::g_type_class_adjust_private_offset(klass, &mut private_offset);
        (*data.as_mut()).private_offset = private_offset as isize;
    }

    // Set trampolines for the basic GObject virtual methods.
    {
        let gobject_klass = &mut *(klass as *mut gobject_ffi::GObjectClass);

        gobject_klass.finalize = Some(finalize::<T>);
    }

    // And finally peek the parent class struct (containing the parent class'
    // implementations of virtual methods for chaining up), and call the subclass'
    // class initialization function.
    {
        let klass = &mut *(klass as *mut T::Class);
        let parent_class = gobject_ffi::g_type_class_peek_parent(klass as *mut _ as ffi::gpointer)
            as *mut <T::ParentType as ObjectType>::GlibClassType;
        assert!(!parent_class.is_null());

        (*data.as_mut()).parent_class = parent_class as ffi::gpointer;

        klass.override_vfuncs();
        T::class_init(klass);
    }
}

unsafe extern "C" fn instance_init<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GTypeInstance,
    klass: ffi::gpointer,
) {
    glib_floating_reference_guard!(obj);

    // Get offset to the storage of our private struct, create it
    // and actually store it in that place.
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr: *mut u8 = obj as *mut _ as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let imp_storage = priv_ptr as *mut Option<T>;

    let klass = &*(klass as *const T::Class);

    let imp = T::new_with_class(klass);

    ptr::write(imp_storage, Some(imp));
}

unsafe extern "C" fn finalize<T: ObjectSubclass>(obj: *mut gobject_ffi::GObject) {
    // Retrieve the private struct, take it out of its storage and
    // drop it for freeing all associated memory.
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr: *mut u8 = obj as *mut _ as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let imp_storage = priv_ptr as *mut Option<T>;

    let imp = (*imp_storage).take().expect("No private struct");
    drop(imp);

    // Chain up to the parent class' finalize implementation, if any.
    let parent_class = &*(data.as_ref().get_parent_class() as *const gobject_ffi::GObjectClass);
    if let Some(ref func) = parent_class.finalize {
        func(obj);
    }
}

/// Register a `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// The [`glib_object_subclass!`] macro will create a `get_type()` function around this, which will
/// ensure that it's only ever called once.
///
/// [`glib_object_subclass!`]: ../../macro.glib_object_subclass.html
pub fn register_type<T: ObjectSubclass>() -> Type
where
    <<T as ObjectSubclass>::ParentType as ObjectType>::RustClassType: IsSubclassable<T>,
{
    unsafe {
        use std::ffi::CString;

        let type_info = gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<T::Class>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(class_init::<T>),
            class_finalize: None,
            class_data: ptr::null_mut(),
            instance_size: mem::size_of::<T::Instance>() as u16,
            n_preallocs: 0,
            instance_init: Some(instance_init::<T>),
            value_table: ptr::null(),
        };

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_ffi::g_type_from_name(type_name.as_ptr()),
            gobject_ffi::G_TYPE_INVALID
        );

        let type_ = from_glib(gobject_ffi::g_type_register_static(
            <T::ParentType as StaticType>::static_type().to_glib(),
            type_name.as_ptr(),
            &type_info,
            if T::ABSTRACT {
                gobject_ffi::G_TYPE_FLAG_ABSTRACT
            } else {
                0
            },
        ));

        let mut data = T::type_data();
        (*data.as_mut()).type_ = type_;
        let private_offset =
            gobject_ffi::g_type_add_instance_private(type_.to_glib(), mem::size_of::<Option<T>>());
        (*data.as_mut()).private_offset = private_offset as isize;

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}

pub(crate) unsafe fn add_signal(
    type_: ffi::GType,
    name: &str,
    flags: SignalFlags,
    arg_types: &[Type],
    ret_type: Type,
) {
    let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();

    gobject_ffi::g_signal_newv(
        name.to_glib_none().0,
        type_,
        flags.to_glib(),
        ptr::null_mut(),
        None,
        ptr::null_mut(),
        None,
        ret_type.to_glib(),
        arg_types.len() as u32,
        arg_types.as_ptr() as *mut _,
    );
}

#[repr(C)]
pub struct SignalInvocationHint(gobject_ffi::GSignalInvocationHint);

impl SignalInvocationHint {
    pub fn detail(&self) -> ::Quark {
        from_glib(self.0.detail)
    }

    pub fn run_type(&self) -> SignalFlags {
        from_glib(self.0.run_type)
    }
}

pub(crate) unsafe fn add_signal_with_accumulator<F>(
    type_: ffi::GType,
    name: &str,
    flags: SignalFlags,
    arg_types: &[Type],
    ret_type: Type,
    accumulator: F,
) where
    F: Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
{
    let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();

    let accumulator: Box<F> = Box::new(accumulator);

    unsafe extern "C" fn accumulator_trampoline<
        F: Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
    >(
        ihint: *mut gobject_ffi::GSignalInvocationHint,
        return_accu: *mut gobject_ffi::GValue,
        handler_return: *const gobject_ffi::GValue,
        data: ffi::gpointer,
    ) -> ffi::gboolean {
        let accumulator: &F = &*(data as *const &F);
        accumulator(
            &SignalInvocationHint(*ihint),
            &mut *(return_accu as *mut Value),
            &*(handler_return as *const Value),
        )
        .to_glib()
    }

    gobject_ffi::g_signal_newv(
        name.to_glib_none().0,
        type_,
        flags.to_glib(),
        ptr::null_mut(),
        Some(accumulator_trampoline::<F>),
        Box::into_raw(accumulator) as ffi::gpointer,
        None,
        ret_type.to_glib(),
        arg_types.len() as u32,
        arg_types.as_ptr() as *mut _,
    );
}

pub(crate) unsafe fn add_signal_with_class_handler<F>(
    type_: ffi::GType,
    name: &str,
    flags: SignalFlags,
    arg_types: &[Type],
    ret_type: Type,
    class_handler: F,
) where
    F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static,
{
    let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
    let class_handler = Closure::new(class_handler);

    gobject_ffi::g_signal_newv(
        name.to_glib_none().0,
        type_,
        flags.to_glib(),
        class_handler.to_glib_none().0,
        None,
        ptr::null_mut(),
        None,
        ret_type.to_glib(),
        arg_types.len() as u32,
        arg_types.as_ptr() as *mut _,
    );
}

pub(crate) unsafe fn add_signal_with_class_handler_and_accumulator<F, G>(
    type_: ffi::GType,
    name: &str,
    flags: SignalFlags,
    arg_types: &[Type],
    ret_type: Type,
    class_handler: F,
    accumulator: G,
) where
    F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static,
    G: Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
{
    let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();

    let class_handler = Closure::new(class_handler);
    let accumulator: Box<G> = Box::new(accumulator);

    unsafe extern "C" fn accumulator_trampoline<
        G: Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
    >(
        ihint: *mut gobject_ffi::GSignalInvocationHint,
        return_accu: *mut gobject_ffi::GValue,
        handler_return: *const gobject_ffi::GValue,
        data: ffi::gpointer,
    ) -> ffi::gboolean {
        let accumulator: &G = &*(data as *const &G);
        accumulator(
            &SignalInvocationHint(*ihint),
            &mut *(return_accu as *mut Value),
            &*(handler_return as *const Value),
        )
        .to_glib()
    }

    gobject_ffi::g_signal_newv(
        name.to_glib_none().0,
        type_,
        flags.to_glib(),
        class_handler.to_glib_none().0,
        Some(accumulator_trampoline::<G>),
        Box::into_raw(accumulator) as ffi::gpointer,
        None,
        ret_type.to_glib(),
        arg_types.len() as u32,
        arg_types.as_ptr() as *mut _,
    );
}
