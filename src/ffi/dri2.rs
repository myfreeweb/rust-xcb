/*
 * This file generated automatically from dri2.xml by r_client.py.
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
use ffi::xproto;

pub static DRI2_MAJOR_VERSION : c_uint = 1;
pub static DRI2_MINOR_VERSION : c_uint = 4;

pub struct dri2_buffer {
    attachment :   u32,
    name :         u32,
    pitch :        u32,
    cpp :          u32,
    flags :        u32
}

/**
 * @brief dri2_buffer_iterator
 **/
pub struct dri2_buffer_iterator {
    data : *mut dri2_buffer,
    rem  : c_int,
    index: c_int
}


pub struct attach_format {
    attachment :   u32,
    format :       u32
}

/**
 * @brief attach_format_iterator
 **/
pub struct attach_format_iterator {
    data : *mut attach_format,
    rem  : c_int,
    index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32
}


pub struct connect_cookie {
    sequence : c_uint
}


pub struct connect_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    driver_type :    u32
}


pub struct connect_reply {
    response_type :        u8,
    pad0 :                 u8,
    sequence :             u16,
    length :               u32,
    driver_name_length :   u32,
    device_name_length :   u32,
    pad1 :                 [u8,..16]
}


pub struct authenticate_cookie {
    sequence : c_uint
}


pub struct authenticate_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    magic :          u32
}


pub struct authenticate_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    authenticated :   u32
}



pub struct create_drawable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable
}



pub struct destroy_drawable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable
}


pub struct get_buffers_cookie {
    sequence : c_uint
}


pub struct get_buffers_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    count :          u32
}


pub struct get_buffers_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    width :           u32,
    height :          u32,
    count :           u32,
    pad1 :            [u8,..12]
}


pub struct copy_region_cookie {
    sequence : c_uint
}


pub struct copy_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    region :         u32,
    dest :           u32,
    src :            u32
}


pub struct copy_region_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32
}


pub struct get_buffers_with_format_cookie {
    sequence : c_uint
}


pub struct get_buffers_with_format_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    count :          u32
}


pub struct get_buffers_with_format_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    width :           u32,
    height :          u32,
    count :           u32,
    pad1 :            [u8,..12]
}


pub struct swap_buffers_cookie {
    sequence : c_uint
}


pub struct swap_buffers_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    drawable :        ffi::xproto::drawable,
    target_msc_hi :   u32,
    target_msc_lo :   u32,
    divisor_hi :      u32,
    divisor_lo :      u32,
    remainder_hi :    u32,
    remainder_lo :    u32
}


pub struct swap_buffers_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    swap_hi :         u32,
    swap_lo :         u32
}


pub struct get_msc_cookie {
    sequence : c_uint
}


pub struct get_msc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable
}


pub struct get_msc_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc_hi :          u32,
    sbc_lo :          u32
}


pub struct wait_msc_cookie {
    sequence : c_uint
}


pub struct wait_msc_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    drawable :        ffi::xproto::drawable,
    target_msc_hi :   u32,
    target_msc_lo :   u32,
    divisor_hi :      u32,
    divisor_lo :      u32,
    remainder_hi :    u32,
    remainder_lo :    u32
}


pub struct wait_msc_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc_hi :          u32,
    sbc_lo :          u32
}


pub struct wait_sbc_cookie {
    sequence : c_uint
}


pub struct wait_sbc_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    drawable :        ffi::xproto::drawable,
    target_sbc_hi :   u32,
    target_sbc_lo :   u32
}


pub struct wait_sbc_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc_hi :          u32,
    sbc_lo :          u32
}



pub struct swap_interval_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    interval :       u32
}


pub struct get_param_cookie {
    sequence : c_uint
}


pub struct get_param_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    param :          u32
}


pub struct get_param_reply {
    response_type :         u8,
    is_param_recognized :   u8,
    sequence :              u16,
    length :                u32,
    value_hi :              u32,
    value_lo :              u32
}



pub struct buffer_swap_complete_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    event_type :      u16,
    pad1 :            [u8,..2],
    drawable :        ffi::xproto::drawable,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc :             u32
}



pub struct invalidate_buffers_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    drawable :        ffi::xproto::drawable
}

#[link(name="lxcb-dri2")]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a dri2_buffer_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(dri2_buffer)
 *
 *
 */
pub fn xcb_dri2_dri2_buffer_next (i:*mut dri2_buffer_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An dri2_buffer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_dri2_dri2_buffer_end (i:dri2_buffer_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a attach_format_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(attach_format)
 *
 *
 */
pub fn xcb_dri2_attach_format_next (i:*mut attach_format_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An attach_format_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_dri2_attach_format_end (i:attach_format_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_query_version (c : *mut connection,
                                  major_version :  u32,
                                  minor_version :  u32) -> query_version_cookie;

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
pub fn xcb_dri2_query_version_unchecked (c : *mut connection,
                                            major_version :  u32,
                                            minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_query_version_reply (c : *mut connection,
                                        cookie : query_version_cookie,
                                        e : *mut *mut generic_error) -> *mut query_version_reply;

pub fn xcb_dri2_connect_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_connect (c : *mut connection,
                            window :  ffi::xproto::window,
                            driver_type :  u32) -> connect_cookie;

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
pub fn xcb_dri2_connect_unchecked (c : *mut connection,
                                      window :  ffi::xproto::window,
                                      driver_type :  u32) -> connect_cookie;

pub fn xcb_dri2_connect_driver_name (R : *mut connect_reply) -> *mut c_char;


pub fn xcb_dri2_connect_driver_name_length (R : *mut connect_reply) -> c_int;


pub fn xcb_dri2_connect_driver_name_end (R : *mut connect_reply) -> generic_iterator;

pub fn xcb_dri2_connect_alignment_pad (R : *mut connect_reply) -> *mut c_void;


pub fn xcb_dri2_connect_alignment_pad_length (R : *mut connect_reply) -> c_int;


pub fn xcb_dri2_connect_alignment_pad_end (R : *mut connect_reply) -> generic_iterator;

pub fn xcb_dri2_connect_device_name (R : *mut connect_reply) -> *mut c_char;


pub fn xcb_dri2_connect_device_name_length (R : *mut connect_reply) -> c_int;


pub fn xcb_dri2_connect_device_name_end (R : *mut connect_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_connect_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_connect_reply (c : *mut connection,
                                  cookie : connect_cookie,
                                  e : *mut *mut generic_error) -> *mut connect_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_authenticate (c : *mut connection,
                                 window :  ffi::xproto::window,
                                 magic :  u32) -> authenticate_cookie;

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
pub fn xcb_dri2_authenticate_unchecked (c : *mut connection,
                                           window :  ffi::xproto::window,
                                           magic :  u32) -> authenticate_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_authenticate_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_authenticate_reply (c : *mut connection,
                                       cookie : authenticate_cookie,
                                       e : *mut *mut generic_error) -> *mut authenticate_reply;

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
pub fn xcb_dri2_create_drawable_checked (c : *mut connection,
                                            drawable :  ffi::xproto::drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_create_drawable (c : *mut connection,
                                    drawable :  ffi::xproto::drawable) -> void_cookie;

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
pub fn xcb_dri2_destroy_drawable_checked (c : *mut connection,
                                             drawable :  ffi::xproto::drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_destroy_drawable (c : *mut connection,
                                     drawable :  ffi::xproto::drawable) -> void_cookie;

pub fn xcb_dri2_get_buffers_sizeof (_buffer :  *mut c_void,
                             attachments_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_get_buffers (c : *mut connection,
                                drawable :  ffi::xproto::drawable,
                                count :  u32,
                                attachments_len :  u32,
                                attachments : *mut u32) -> get_buffers_cookie;

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
pub fn xcb_dri2_get_buffers_unchecked (c : *mut connection,
                                          drawable :  ffi::xproto::drawable,
                                          count :  u32,
                                          attachments_len :  u32,
                                          attachments : *mut u32) -> get_buffers_cookie;

pub fn xcb_dri2_get_buffers_buffers (R : *mut get_buffers_reply) -> *mut dri2_buffer;


pub fn xcb_dri2_get_buffers_buffers_length (R : *mut get_buffers_reply) -> c_int;

pub fn xcb_dri2_get_buffers_buffers_iterator (R : *mut get_buffers_reply) -> dri2_buffer_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_buffers_reply (c : *mut connection,
                                      cookie : get_buffers_cookie,
                                      e : *mut *mut generic_error) -> *mut get_buffers_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_copy_region (c : *mut connection,
                                drawable :  ffi::xproto::drawable,
                                region :  u32,
                                dest :  u32,
                                src :  u32) -> copy_region_cookie;

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
pub fn xcb_dri2_copy_region_unchecked (c : *mut connection,
                                          drawable :  ffi::xproto::drawable,
                                          region :  u32,
                                          dest :  u32,
                                          src :  u32) -> copy_region_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_copy_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_copy_region_reply (c : *mut connection,
                                      cookie : copy_region_cookie,
                                      e : *mut *mut generic_error) -> *mut copy_region_reply;

pub fn xcb_dri2_get_buffers_with_format_sizeof (_buffer :  *mut c_void,
                                         attachments_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_get_buffers_with_format (c : *mut connection,
                                            drawable :  ffi::xproto::drawable,
                                            count :  u32,
                                            attachments_len :  u32,
                                            attachments : *mut attach_format) -> get_buffers_with_format_cookie;

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
pub fn xcb_dri2_get_buffers_with_format_unchecked (c : *mut connection,
                                                      drawable :  ffi::xproto::drawable,
                                                      count :  u32,
                                                      attachments_len :  u32,
                                                      attachments : *mut attach_format) -> get_buffers_with_format_cookie;

pub fn xcb_dri2_get_buffers_with_format_buffers (R : *mut get_buffers_with_format_reply) -> *mut dri2_buffer;


pub fn xcb_dri2_get_buffers_with_format_buffers_length (R : *mut get_buffers_with_format_reply) -> c_int;

pub fn xcb_dri2_get_buffers_with_format_buffers_iterator (R : *mut get_buffers_with_format_reply) -> dri2_buffer_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_with_format_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_buffers_with_format_reply (c : *mut connection,
                                                  cookie : get_buffers_with_format_cookie,
                                                  e : *mut *mut generic_error) -> *mut get_buffers_with_format_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_swap_buffers (c : *mut connection,
                                 drawable :  ffi::xproto::drawable,
                                 target_msc_hi :  u32,
                                 target_msc_lo :  u32,
                                 divisor_hi :  u32,
                                 divisor_lo :  u32,
                                 remainder_hi :  u32,
                                 remainder_lo :  u32) -> swap_buffers_cookie;

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
pub fn xcb_dri2_swap_buffers_unchecked (c : *mut connection,
                                           drawable :  ffi::xproto::drawable,
                                           target_msc_hi :  u32,
                                           target_msc_lo :  u32,
                                           divisor_hi :  u32,
                                           divisor_lo :  u32,
                                           remainder_hi :  u32,
                                           remainder_lo :  u32) -> swap_buffers_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_swap_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_swap_buffers_reply (c : *mut connection,
                                       cookie : swap_buffers_cookie,
                                       e : *mut *mut generic_error) -> *mut swap_buffers_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_get_msc (c : *mut connection,
                            drawable :  ffi::xproto::drawable) -> get_msc_cookie;

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
pub fn xcb_dri2_get_msc_unchecked (c : *mut connection,
                                      drawable :  ffi::xproto::drawable) -> get_msc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_msc_reply (c : *mut connection,
                                  cookie : get_msc_cookie,
                                  e : *mut *mut generic_error) -> *mut get_msc_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_wait_msc (c : *mut connection,
                             drawable :  ffi::xproto::drawable,
                             target_msc_hi :  u32,
                             target_msc_lo :  u32,
                             divisor_hi :  u32,
                             divisor_lo :  u32,
                             remainder_hi :  u32,
                             remainder_lo :  u32) -> wait_msc_cookie;

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
pub fn xcb_dri2_wait_msc_unchecked (c : *mut connection,
                                       drawable :  ffi::xproto::drawable,
                                       target_msc_hi :  u32,
                                       target_msc_lo :  u32,
                                       divisor_hi :  u32,
                                       divisor_lo :  u32,
                                       remainder_hi :  u32,
                                       remainder_lo :  u32) -> wait_msc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_wait_msc_reply (c : *mut connection,
                                   cookie : wait_msc_cookie,
                                   e : *mut *mut generic_error) -> *mut wait_msc_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_wait_sbc (c : *mut connection,
                             drawable :  ffi::xproto::drawable,
                             target_sbc_hi :  u32,
                             target_sbc_lo :  u32) -> wait_sbc_cookie;

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
pub fn xcb_dri2_wait_sbc_unchecked (c : *mut connection,
                                       drawable :  ffi::xproto::drawable,
                                       target_sbc_hi :  u32,
                                       target_sbc_lo :  u32) -> wait_sbc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_sbc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_wait_sbc_reply (c : *mut connection,
                                   cookie : wait_sbc_cookie,
                                   e : *mut *mut generic_error) -> *mut wait_sbc_reply;

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
pub fn xcb_dri2_swap_interval_checked (c : *mut connection,
                                          drawable :  ffi::xproto::drawable,
                                          interval :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_swap_interval (c : *mut connection,
                                  drawable :  ffi::xproto::drawable,
                                  interval :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_get_param (c : *mut connection,
                              drawable :  ffi::xproto::drawable,
                              param :  u32) -> get_param_cookie;

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
pub fn xcb_dri2_get_param_unchecked (c : *mut connection,
                                        drawable :  ffi::xproto::drawable,
                                        param :  u32) -> get_param_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_param_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_param_reply (c : *mut connection,
                                    cookie : get_param_cookie,
                                    e : *mut *mut generic_error) -> *mut get_param_reply;
}

