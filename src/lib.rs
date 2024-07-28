use anyhow::Result;
use retour::static_detour;
use rivets::{detour, inject, start_stream};
use tracing::{error, info};

#[detour("?valid@LuaSurface@@UEBA_NXZ")]
fn valid() -> bool {
    info!("Detoured into LuaSurface::valid!");
    false
}
