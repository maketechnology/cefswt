/* automatically generated by rust-bindgen */

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

///
/// Structure representing CefExecuteProcess arguments.
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cef_main_args_t {
    pub argc: ::std::os::raw::c_int,
    pub argv: *mut *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout__cef_main_args_t() {
    assert_eq!(
        ::std::mem::size_of::<_cef_main_args_t>(),
        16usize,
        concat!("Size of: ", stringify!(_cef_main_args_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_cef_main_args_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_cef_main_args_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_cef_main_args_t>())).argc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_main_args_t),
            "::",
            stringify!(argc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_cef_main_args_t>())).argv as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_main_args_t),
            "::",
            stringify!(argv)
        )
    );
}
///
/// Class representing window information.
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cef_window_info_t {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
    pub width: ::std::os::raw::c_uint,
    pub height: ::std::os::raw::c_uint,
    ///
    /// // Pointer for the parent window.
    /// ///
    pub parent_window: ::std::os::raw::c_ulong,
    ///
    /// // Set to true (1) to create the browser using windowless (off-screen)
    /// // rendering. No window will be created for the browser and all rendering will
    /// // occur via the CefRenderHandler interface. The |parent_window| value will be
    /// // used to identify monitor info and to act as the parent window for dialogs,
    /// // context menus, etc. If |parent_window| is not provided then the main screen
    /// // monitor will be used and some functionality that requires a parent window
    /// // may not function correctly. In order to create windowless browsers the
    /// // CefSettings.windowless_rendering_enabled value must be set to true.
    /// // Transparent painting is enabled by default but can be disabled by setting
    /// // CefBrowserSettings.background_color to an opaque value.
    /// ///
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    ///
    /// // Pointer for the new browser window. Only used with windowed rendering.
    /// ///
    pub window: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout__cef_window_info_t() {
    assert_eq!(
        ::std::mem::size_of::<_cef_window_info_t>(),
        40usize,
        concat!("Size of: ", stringify!(_cef_window_info_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_cef_window_info_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_cef_window_info_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_cef_window_info_t>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_window_info_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_cef_window_info_t>())).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_window_info_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_cef_window_info_t>())).width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_window_info_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_cef_window_info_t>())).height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_window_info_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_cef_window_info_t>())).parent_window as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_window_info_t),
            "::",
            stringify!(parent_window)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_cef_window_info_t>())).windowless_rendering_enabled as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_window_info_t),
            "::",
            stringify!(windowless_rendering_enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_cef_window_info_t>())).window as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_cef_window_info_t),
            "::",
            stringify!(window)
        )
    );
}