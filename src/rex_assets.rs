use rltk::rex::XpFile;
rltk::embedded_resource!(SMALL_DUNGEON, "../resources/temple.xp");

pub struct RexAssets {
    pub menu: XpFile,
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(SMALL_DUNGEON, "../resources/temple.xp");

        RexAssets {
            menu: XpFile::from_resource("../resources/temple.xp").unwrap(),
        }
    }
}
