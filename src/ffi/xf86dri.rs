/*
 * This file generated automatically from xf86dri.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use ffi;

pub static XF86DRI_MAJOR_VERSION : c_uint = 4;
pub static XF86DRI_MINOR_VERSION : c_uint = 1;

pub struct drm_clip_rect {
    x1 :   i16,
    y1 :   i16,
    x2 :   i16,
    x3 :   i16
}

/**
 * @brief drm_clip_rect_iterator
 **/
pub struct drm_clip_rect_iterator {
    data : *mut drm_clip_rect,
    rem  : c_int,
    index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_version_reply {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    length :              u32,
    dri_major_version :   u16,
    dri_minor_version :   u16,
    dri_minor_patch :     u32
}


pub struct query_direct_rendering_capable_cookie {
    sequence : c_uint
}


pub struct query_direct_rendering_capable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct query_direct_rendering_capable_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    is_capable :      u8
}


pub struct open_connection_cookie {
    sequence : c_uint
}


pub struct open_connection_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct open_connection_reply {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    length :              u32,
    sarea_handle_low :    u32,
    sarea_handle_high :   u32,
    bus_id_len :          u32,
    pad1 :                [u8,..12]
}



pub struct close_connection_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct get_client_driver_name_cookie {
    sequence : c_uint
}


pub struct get_client_driver_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct get_client_driver_name_reply {
    response_type :                 u8,
    pad0 :                          u8,
    sequence :                      u16,
    length :                        u32,
    client_driver_major_version :   u32,
    client_driver_minor_version :   u32,
    client_driver_patch_version :   u32,
    client_driver_name_len :        u32,
    pad1 :                          [u8,..8]
}


pub struct create_context_cookie {
    sequence : c_uint
}


pub struct create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    visual :         u32,
    context :        u32
}


pub struct create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    hw_context :      u32
}



pub struct destroy_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    context :        u32
}


pub struct create_drawable_cookie {
    sequence : c_uint
}


pub struct create_drawable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    drawable :       u32
}


pub struct create_drawable_reply {
    response_type :        u8,
    pad0 :                 u8,
    sequence :             u16,
    length :               u32,
    hw_drawable_handle :   u32
}



pub struct destroy_drawable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    drawable :       u32
}


pub struct get_drawable_info_cookie {
    sequence : c_uint
}


pub struct get_drawable_info_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    drawable :       u32
}


pub struct get_drawable_info_reply {
    response_type :          u8,
    pad0 :                   u8,
    sequence :               u16,
    length :                 u32,
    drawable_table_index :   u32,
    drawable_table_stamp :   u32,
    drawable_origin_X :      i16,
    drawable_origin_Y :      i16,
    drawable_size_W :        i16,
    drawable_size_H :        i16,
    num_clip_rects :         u32,
    back_x :                 i16,
    back_y :                 i16,
    num_back_clip_rects :    u32
}


pub struct get_device_info_cookie {
    sequence : c_uint
}


pub struct get_device_info_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct get_device_info_reply {
    response_type :               u8,
    pad0 :                        u8,
    sequence :                    u16,
    length :                      u32,
    framebuffer_handle_low :      u32,
    framebuffer_handle_high :     u32,
    framebuffer_origin_offset :   u32,
    framebuffer_size :            u32,
    framebuffer_stride :          u32,
    device_private_size :         u32
}


pub struct auth_connection_cookie {
    sequence : c_uint
}


pub struct auth_connection_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    magic :          u32
}


pub struct auth_connection_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    authenticated :   u32
}

#[link(name="lxcb-xf86dri")]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a drm_clip_rect_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(drm_clip_rect)
 *
 *
 */
pub fn xcb_xf86dri_drm_clip_rect_next (i:*mut drm_clip_rect_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An drm_clip_rect_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86dri_drm_clip_rect_end (i:drm_clip_rect_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_query_version (c : *mut connection) -> query_version_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_query_version_unchecked (c : *mut connection) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_query_version_reply (c : *mut connection,
                                           cookie : query_version_cookie,
                                           e : *mut *mut generic_error) -> *mut query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_query_direct_rendering_capable (c : *mut connection,
                                                      screen :  u32) -> query_direct_rendering_capable_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_query_direct_rendering_capable_unchecked (c : *mut connection,
                                                                screen :  u32) -> query_direct_rendering_capable_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_query_direct_rendering_capable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_query_direct_rendering_capable_reply (c : *mut connection,
                                                            cookie : query_direct_rendering_capable_cookie,
                                                            e : *mut *mut generic_error) -> *mut query_direct_rendering_capable_reply;

pub fn xcb_xf86dri_open_connection_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_open_connection (c : *mut connection,
                                       screen :  u32) -> open_connection_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_open_connection_unchecked (c : *mut connection,
                                                 screen :  u32) -> open_connection_cookie;

pub fn xcb_xf86dri_open_connection_bus_id (R : *mut open_connection_reply) -> *mut c_char;


pub fn xcb_xf86dri_open_connection_bus_id_length (R : *mut open_connection_reply) -> c_int;


pub fn xcb_xf86dri_open_connection_bus_id_end (R : *mut open_connection_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_open_connection_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_open_connection_reply (c : *mut connection,
                                             cookie : open_connection_cookie,
                                             e : *mut *mut generic_error) -> *mut open_connection_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xf86dri_close_connection_checked (c : *mut connection,
                                                screen :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_close_connection (c : *mut connection,
                                        screen :  u32) -> void_cookie;

pub fn xcb_xf86dri_get_client_driver_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_get_client_driver_name (c : *mut connection,
                                              screen :  u32) -> get_client_driver_name_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_get_client_driver_name_unchecked (c : *mut connection,
                                                        screen :  u32) -> get_client_driver_name_cookie;

pub fn xcb_xf86dri_get_client_driver_name_client_driver_name (R : *mut get_client_driver_name_reply) -> *mut c_char;


pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_length (R : *mut get_client_driver_name_reply) -> c_int;


pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_end (R : *mut get_client_driver_name_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_get_client_driver_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_get_client_driver_name_reply (c : *mut connection,
                                                    cookie : get_client_driver_name_cookie,
                                                    e : *mut *mut generic_error) -> *mut get_client_driver_name_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_create_context (c : *mut connection,
                                      screen :  u32,
                                      visual :  u32,
                                      context :  u32) -> create_context_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_create_context_unchecked (c : *mut connection,
                                                screen :  u32,
                                                visual :  u32,
                                                context :  u32) -> create_context_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_create_context_reply (c : *mut connection,
                                            cookie : create_context_cookie,
                                            e : *mut *mut generic_error) -> *mut create_context_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xf86dri_destroy_context_checked (c : *mut connection,
                                               screen :  u32,
                                               context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_destroy_context (c : *mut connection,
                                       screen :  u32,
                                       context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_create_drawable (c : *mut connection,
                                       screen :  u32,
                                       drawable :  u32) -> create_drawable_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_create_drawable_unchecked (c : *mut connection,
                                                 screen :  u32,
                                                 drawable :  u32) -> create_drawable_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_create_drawable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_create_drawable_reply (c : *mut connection,
                                             cookie : create_drawable_cookie,
                                             e : *mut *mut generic_error) -> *mut create_drawable_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xf86dri_destroy_drawable_checked (c : *mut connection,
                                                screen :  u32,
                                                drawable :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_destroy_drawable (c : *mut connection,
                                        screen :  u32,
                                        drawable :  u32) -> void_cookie;

pub fn xcb_xf86dri_get_drawable_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_get_drawable_info (c : *mut connection,
                                         screen :  u32,
                                         drawable :  u32) -> get_drawable_info_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_get_drawable_info_unchecked (c : *mut connection,
                                                   screen :  u32,
                                                   drawable :  u32) -> get_drawable_info_cookie;

pub fn xcb_xf86dri_get_drawable_info_clip_rects (R : *mut get_drawable_info_reply) -> *mut drm_clip_rect;


pub fn xcb_xf86dri_get_drawable_info_clip_rects_length (R : *mut get_drawable_info_reply) -> c_int;

pub fn xcb_xf86dri_get_drawable_info_clip_rects_iterator (R : *mut get_drawable_info_reply) -> drm_clip_rect_iterator;

pub fn xcb_xf86dri_get_drawable_info_back_clip_rects (R : *mut get_drawable_info_reply) -> *mut drm_clip_rect;


pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_length (R : *mut get_drawable_info_reply) -> c_int;

pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_iterator (R : *mut get_drawable_info_reply) -> drm_clip_rect_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_get_drawable_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_get_drawable_info_reply (c : *mut connection,
                                               cookie : get_drawable_info_cookie,
                                               e : *mut *mut generic_error) -> *mut get_drawable_info_reply;

pub fn xcb_xf86dri_get_device_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_get_device_info (c : *mut connection,
                                       screen :  u32) -> get_device_info_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_get_device_info_unchecked (c : *mut connection,
                                                 screen :  u32) -> get_device_info_cookie;

pub fn xcb_xf86dri_get_device_info_device_private (R : *mut get_device_info_reply) -> *mut u32;


pub fn xcb_xf86dri_get_device_info_device_private_length (R : *mut get_device_info_reply) -> c_int;


pub fn xcb_xf86dri_get_device_info_device_private_end (R : *mut get_device_info_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_get_device_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_get_device_info_reply (c : *mut connection,
                                             cookie : get_device_info_cookie,
                                             e : *mut *mut generic_error) -> *mut get_device_info_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_auth_connection (c : *mut connection,
                                       screen :  u32,
                                       magic :  u32) -> auth_connection_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xf86dri_auth_connection_unchecked (c : *mut connection,
                                                 screen :  u32,
                                                 magic :  u32) -> auth_connection_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_auth_connection_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_auth_connection_reply (c : *mut connection,
                                             cookie : auth_connection_cookie,
                                             e : *mut *mut generic_error) -> *mut auth_connection_reply;
}

