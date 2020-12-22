use wasm_sc_guest::*;

#[no_mangle]
pub static PROTOCOL: &str = "tls";

#[no_mangle]
pub extern "C" fn tx_log(_tx_id: u64) -> i32 {
    SCLogInfo!("WASM: TLS tx log");
    let serial = tls_get_cert_serial();
    SCLogInfo!("serial: {:?}", serial);
    let sni = tls_get_sni();
    SCLogInfo!("sni: {:?}", sni);
    0
}

#[no_mangle]
pub extern "C" fn init(_major: u32, _minor: u32) -> i32 {
    SCLogInfo!("WASM: TLS tx logger init");
    0
}

