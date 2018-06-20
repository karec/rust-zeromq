//! # FFI binding from libzmq
#![allow(non_camel_case_types)]

#[repr(C)]
pub struct ZMQMsg {
    pub message: [u8; 8usize]
}

pub type zmq_msg_t = ZMQMsg;
pub type zmq_free_fn = unsafe extern "C" fn(data: *mut ::libc::c_void, hint: *mut ::libc::c_void);


#[link(name="zmq")]
extern "C" {
    // Context functions
    // Omit legacy API functions
    pub fn zmq_ctx_new() -> *mut ::libc::c_void;
    pub fn zmq_ctx_term(context: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_ctx_shutdown(ctx: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_ctx_set(context: *mut ::libc::c_void, option: ::libc::c_int, optval: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_ctx_get(context: *mut ::libc::c_void, option: ::libc::c_int) -> ::libc::c_int;

    // Messages functions
    pub fn zmq_msg_init(msg: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_init_size(msg: *mut zmq_msg_t, size: ::libc::size_t) -> ::libc::c_int;
    pub fn zmq_msg_init_data(
        msg: *mut zmq_msg_t, data: *mut ::libc::c_void, size: ::libc::size_t, ffn: *mut zmq_free_fn, hint: *mut ::libc::c_void
    ) -> ::libc::c_int;
    pub fn zmq_msg_send(msg: *mut zmq_msg_t, socket: *mut ::libc::c_void, flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_msg_recv(msg: *mut zmq_msg_t, socket: *mut ::libc::c_void, flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_msg_close(msg: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_move(dest: *mut zmq_msg_t, src: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_copy(dest: *mut zmq_msg_t, src: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_data(msg: *mut zmq_msg_t) -> *mut ::libc::c_void;
    pub fn zmq_msg_size(msg: *mut zmq_msg_t) -> ::libc::size_t;
    pub fn zmq_msg_more(msg: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_get(msg: *mut zmq_msg_t, property: ::libc::c_int) -> ::libc::c_int;
}


#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test_ctx() {
        unsafe {
            let ctx = zmq_ctx_new();

            assert_eq!(zmq_ctx_set(ctx, 2, 2), 0);
            assert_eq!(zmq_ctx_set(ctx, 255, 2), -1);
            assert_eq!(zmq_ctx_get(ctx, 2), 2);
            assert_eq!(zmq_ctx_get(ctx, 1), 1);

            assert_eq!(zmq_ctx_shutdown(ctx), 0);
            assert_eq!(zmq_ctx_term(ctx), 0);
        };
    }
}

