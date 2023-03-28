use std::{ffi::CString, ptr};

use gtk::{ffi::GtkIMContextInfo, glib::gobject_ffi::{GTypeModule, G_TYPE_CHAR}};
use libc::{system, c_void};
use tracing::{debug, error, info, warn};

fn init_log() {
    let file_appender = tracing_appender::rolling::hourly("/tmp", "irms.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    debug!("test debug");
    info!("test info");
    warn!("test warn");
    error!("test error");
}

#[no_mangle]
pub unsafe extern "C" fn im_module_create(context_id: *const libc::c_char) {
    let s = CString::new("echo sanity check1 > /tmp/asdf.log").unwrap();
    system(s.as_ptr());
    init_log();
}

#[no_mangle]
pub unsafe extern "C" fn im_module_list(
    contexts: *mut *const *const GtkIMContextInfo,
    guint: *mut u32,
) {
    let imrs = CString::new("imrs".as_bytes()).unwrap();
    let locale = CString::new("/usr/share/locale".as_bytes()).unwrap();
    println!("asdf {:?}", imrs.as_ptr());
    let ctx_info = GtkIMContextInfo {
        context_id: imrs.as_ptr(),
        context_name: imrs.as_ptr(),
        domain: imrs.as_ptr(),
        domain_dirname: imrs.as_ptr(),
        default_locales: locale.as_ptr(),
    };
    let ptrs = vec![Box::into_raw(Box::new(ctx_info)) as *const GtkIMContextInfo];
    let ptr = Box::into_raw(ptrs.into_boxed_slice()) as *const *const GtkIMContextInfo;
    *contexts = ptr;
    *guint = 1;
}

#[no_mangle]
pub unsafe extern "C" fn im_module_init(module: *mut GTypeModule) {
    let s = CString::new("echo sanity check2 > /tmp/asdf.log").unwrap();
    system(s.as_ptr());
    init_log();
}

#[no_mangle]
pub unsafe extern "C" fn im_module_exit() {}
