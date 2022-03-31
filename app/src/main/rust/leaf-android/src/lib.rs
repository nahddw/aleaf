use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_hk_nahdd_telev_SimpleVpnService_runLeaf(
    env: JNIEnv,
    _: JClass,
    config_path: JString,
    protect_path: JString,
) {
    let config_path = env
        .get_string(config_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let protect_path = env
        .get_string(protect_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let opts = leaf::StartOptions {
        config: leaf::Config::File(config_path),
        socket_protect_path: Some(protect_path),
        runtime_opt: leaf::RuntimeOption::SingleThread,
    };
    leaf::start(0, opts).unwrap();
}

//test
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_hk_nahdd_telev_SimpleVpnService_stopLeaf(
    _: JNIEnv,
    _: JClass,
) {
    leaf::shutdown(0);
}

//add
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_hk_nahdd_telev_SimpleVpnService_isRunning(
    _: JNIEnv,
    _: JClass,
) -> sys::jboolean {
    if leaf::is_running(0) {
        sys::JNI_TRUE
    } else {
        sys::JNI_FALSE
    }
}
