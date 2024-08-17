use factorio_mlua::lua_State;
use factorio_mlua::Lua;
use factorio_mlua::Value;
use rivets::defines;
use rivets::detour;
use rivets::Opaque;

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
            println!("Rust event handler! {event:?}");
        }
        Err(_) => {
            println!("Unknown event {lua_event_type:?}");
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
fn valid(this: Opaque) -> bool {
    println!("bbb!");
    unsafe { back(this) }
}

#[detour(?luaCountTilesFiltered@LuaSurface@@QEAAHPEAUlua_State@@@Z)]
fn lua_count_tiles_filtered(this: Opaque, lua_state: *mut lua_State) -> i64 {
    let res = unsafe { back(this, lua_state) };
    println!("lua_count_tiles_filtered!");
    let lua = unsafe { Lua::init_from_ptr(lua_state) };
    let globals = lua.globals();
    for (k, v) in globals.pairs::<Value, Value>().flatten() {
        println!("Global: {k:?} = {v:?}");
    }
    println!("Globals printed!");
    println!("res: {res}");
    res
}

rivets::finalize!();
