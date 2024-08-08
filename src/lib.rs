use std::ffi::c_void;

use rivets::detour;
use tracing::info;

#[detour(?run@LuaEventDispatcher@@AEAAXW4LuaEventType@@VMapTickType@@P8LuaGameScript@@EAA_NAEBVGameAction@@@Z2@Z)]
fn run(this: *const c_void, lua_event_type: i32, map_tick_type: *const c_void, lua_game_script: *const c_void, game_action: *const c_void) {
    if lua_event_type == 0 {
        unsafe { source(this, lua_event_type, map_tick_type, lua_game_script, game_action); }
        return;
    }
    info!("Blocked event? {:?}", lua_event_type);
    unsafe { source(this, lua_event_type, map_tick_type, lua_game_script, game_action); }
}