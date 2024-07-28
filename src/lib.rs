use tracing::info;

#[no_mangle]
pub extern "C" fn main_detour() -> extern "C" fn() -> bool {
    extern "C" fn a() ->bool {
        false
    }
    a
}

#[no_mangle]
pub extern "C" fn mangled_name() -> String {
    "?valid@LuaSurface@@UEBA_NXZ".to_string()
}
