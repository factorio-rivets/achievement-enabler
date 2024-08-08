use std::ffi::c_void;

use rivets::defines;
use rivets::detour;
use rivets::Opaque;
use tracing::info;

#[detour(?run@LuaEventDispatcher@@AEAAXW4LuaEventType@@VMapTickType@@P8LuaGameScript@@EAA_NAEBVGameAction@@@Z2@Z)]
fn run(
    this: Opaque,
    lua_event_type: i32,
    map_tick_type: Opaque,
    lua_game_script: Opaque,
    game_action: Opaque,
) {
    if lua_event_type == 0 {
        return unsafe { back(this, lua_event_type, map_tick_type, lua_game_script, game_action); }
    }
    let q: Result<defines::events, _> = (lua_event_type as u8).try_into();
    if let Ok(q) = q {
        info!("Rust event handler! {:?}", q);
    }
    unsafe { back(this, lua_event_type, map_tick_type, lua_game_script, game_action) }
}
