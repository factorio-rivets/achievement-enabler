use std::ffi::c_void;

use rivets::defines;
use rivets::detour;
use tracing::info;

#[detour(?run@LuaEventDispatcher@@AEAAXW4LuaEventType@@VMapTickType@@P8LuaGameScript@@EAA_NAEBVGameAction@@@Z2@Z)]
fn run(
    this: *const c_void,
    lua_event_type: i32,
    map_tick_type: *const c_void,
    lua_game_script: *const c_void,
    game_action: *const c_void,
) {
    if lua_event_type == 0 {
        unsafe {
            source(
                this,
                lua_event_type,
                map_tick_type,
                lua_game_script,
                game_action,
            );
        }
        return;
    }
    info!("Blocked event? {:?}", lua_event_type);
    let q: defines::events = (lua_event_type as u8).try_into().unwrap();
    info!("Blocked event? {:?}", q);
    unsafe {
        source(
            this,
            lua_event_type,
            map_tick_type,
            lua_game_script,
            game_action,
        );
    }
}
