use tracing::{error, info};
use rivets::{inject, start_stream};
use retour::static_detour;
use anyhow::Result;

static_detour! {
    static MainHook: unsafe extern "C" fn() -> bool;
}

type FnMain = unsafe extern "C" fn() -> bool;

fn main_detour() -> bool {
    info!("Detoured into main!");
    //unsafe { MessageBoxWHook.call(hwnd, text, replaced_caption, msgbox_style) }
    false
}

unsafe fn hook(address: u64) -> Result<()> {
    let fnmain: FnMain = std::mem::transmute(address);
    MainHook.initialize(fnmain, main_detour)?.enable()?;
    Ok(())
}

#[ctor::ctor]
fn ctor() {
    start_stream();

    if let Err(e) = inject("?valid@LuaSurface@@UEBA_NXZ", hook) {
        error!("{e}");
    }
}
