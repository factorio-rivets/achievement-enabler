use rivets::defines;
use rivets::detour;
use rivets::Opaque;
use tracing::info;

#[detour(?run@LuaEventDispatcher@@AEAAXW4LuaEventType@@VMapTickType@@P8LuaGameScript@@EAA_NAEBVGameAction@@@Z2@Z)]
fn run(
    this: Opaque,
    lua_event_type: usize,
    map_tick_type: Opaque,
    lua_game_script: Opaque,
    game_action: Opaque,
) {
    let event = defines::events::try_from(&lua_event_type);
    match event {
        Ok(defines::events::on_tick) => {}
        Ok(event) => {
            info!("Rust event handler! {:?}", event);
        }
        Err(_) => {
            info!("Unknown event {:?}", lua_event_type);
        }
    }
    unsafe {
        back(
            this,
            lua_event_type,
            map_tick_type,
            lua_game_script,
            game_action,
        );
    }
}

#[detour(?valid@LuaSurface@@UEBA_NXZ)]
const fn valid(this: Opaque) -> bool {
    false
}

rivets::initialize!();
