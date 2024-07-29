use rivets::detour;
use tracing::info;

#[detour(?valid@LuaSurface@@UEBA_NXZ)]
fn valid(ggg: bool) -> bool {
    info!("Detoured into LuaSurface::valid! {ggg}");
    false
}
