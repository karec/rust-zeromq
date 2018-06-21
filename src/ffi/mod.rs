//! # FFI binding from libzmq
#![allow(non_camel_case_types)]

use libc::{c_void, c_int, size_t, c_char};

#[repr(C)]
pub struct ZMQMsg {
    pub message: [u8; 8usize]
}

pub type zmq_msg_t = ZMQMsg;
pub type zmq_free_fn = unsafe extern "C" fn(data: *mut c_void, hint: *mut c_void);


#[link(name="zmq")]
extern "C" {
    // Context functions
    // Omit legacy API functions
    pub fn zmq_ctx_new() -> *mut c_void;
    pub fn zmq_ctx_term(context: *mut c_void) -> c_int;
    pub fn zmq_ctx_shutdown(ctx: *mut c_void) -> c_int;
    pub fn zmq_ctx_set(context: *mut c_void, option: c_int, optval: c_int) -> c_int;
    pub fn zmq_ctx_get(context: *mut c_void, option: c_int) -> c_int;

    // Messages functions
    pub fn zmq_msg_init(msg: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_init_size(msg: *mut zmq_msg_t, size: size_t) -> c_int;
    pub fn zmq_msg_init_data(
        msg: *mut zmq_msg_t, data: *mut c_void, size: size_t, ffn: *mut zmq_free_fn, hint: *mut c_void
    ) -> c_int;
    pub fn zmq_msg_send(msg: *mut zmq_msg_t, socket: *mut c_void, flags: c_int) -> c_int;
    pub fn zmq_msg_recv(msg: *mut zmq_msg_t, socket: *mut c_void, flags: c_int) -> c_int;
    pub fn zmq_msg_close(msg: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_move(dest: *mut zmq_msg_t, src: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_copy(dest: *mut zmq_msg_t, src: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_data(msg: *mut zmq_msg_t) -> *mut c_void;
    pub fn zmq_msg_size(msg: *mut zmq_msg_t) -> size_t;
    pub fn zmq_msg_more(msg: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_get(msg: *mut zmq_msg_t, property: c_int) -> c_int;
    pub fn zmq_msg_gets(msg: *mut zmq_msg_t, property: *const c_char) -> *const c_char;

    // Socket events and monitoring
    pub fn zmq_socket(ctx: *mut c_void, type_: c_int) -> *mut c_void;
    pub fn zmq_close(socket: *mut c_void) -> c_int;
    pub fn zmq_setsockopt(socket: *mut c_void, option: c_int, optval: *mut c_void, optvallen: size_t) -> c_int;
    pub fn zmq_getsockopt(socket: *mut c_void, option: c_int, optval: *mut c_void, optvallen: size_t) -> c_int;
    pub fn zmq_bind(sock: *mut c_void, addr: *const c_char) -> c_int;
    pub fn zmq_connect(sock: *mut c_void, addr: *const c_char) -> c_int;
    pub fn zmq_unbind(sock: *mut c_void, addr: *const c_char) -> c_int;
    pub fn zmq_disconnect(sock: *mut c_void, addr: *const c_char) -> c_int;
    pub fn zmq_send(sock: *mut c_void, buf: *const c_void, len: size_t, flags: c_int) -> c_int;
    pub fn zmq_send_const(sock: *mut c_void, buf: *const c_void, len: size_t, flags: c_int) -> c_int;
    pub fn zmq_recv(sock: *mut c_void, buf: *mut c_void, len: size_t, flags: c_int) -> c_int;
    pub fn zmq_socket_monitor(sock: *mut c_void, addr: *const c_char, events: c_int) -> c_int;

    // Message proxying
    pub fn zmq_proxy(frontend: *mut c_void, backend: *mut c_void, capture: *mut c_void) -> c_int;
    pub fn zmq_proxy_steerable(frontend: *mut c_void, backend: *mut c_void, capture: *mut c_void, control: *mut c_void) -> c_int;

    // Probe library capabilities
    pub fn zmq_has(capability: *const c_char) -> c_int;
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

