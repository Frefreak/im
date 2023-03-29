use std::ffi::CString;

use gtk::{ffi::GtkIMContextInfo, glib::gobject_ffi::GTypeModule};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

fn init_log() {
    let file_appender = tracing_appender::rolling::hourly("./log/", "irms.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(file_appender)
        .init();
    debug!("init_log debug");
    info!("init_log info");
    warn!("init_log warn");
    error!("init_log error");
}

#[no_mangle]
pub unsafe extern "C" fn im_module_create(context_id: *const libc::c_char) {
    init_log();
}

#[no_mangle]
pub unsafe extern "C" fn im_module_list(
    contexts: *mut *const *const GtkIMContextInfo,
    guint: *mut u32,
) {
    let ctx_id = CString::new("imrs".as_bytes()).unwrap().into_raw();
    let ctx_name = CString::new("IRMS".as_bytes()).unwrap().into_raw();
    let domain = CString::new("gtk30".as_bytes()).unwrap().into_raw();
    let domain_dirname = CString::new("/usr/share/locale".as_bytes()).unwrap().into_raw();
    let locale = CString::new("zh".as_bytes()).unwrap().into_raw();
    let ctx_info = GtkIMContextInfo {
        context_id: ctx_id,
        context_name: ctx_name,
        domain,
        domain_dirname,
        default_locales: locale,
    };
    let ptrs = vec![Box::into_raw(Box::new(ctx_info)) as *const GtkIMContextInfo];
    let ptr = Box::into_raw(ptrs.into_boxed_slice()) as *const *const GtkIMContextInfo;
    *contexts = ptr;
    *guint = 1;
}

#[no_mangle]
pub unsafe extern "C" fn im_module_init(module: *mut GTypeModule) {
    info!("init");
}

#[no_mangle]
pub unsafe extern "C" fn im_module_exit() {
    info!("exit");
}
