use rltk::rex::XpFile;
rltk::embedded_resource!(SPLASH_SCREEN, "../resources/temple.xp");
rltk::embedded_resource!(GAME_OVER, "../resources/coffin2.xp");
rltk::embedded_resource!(WFC_DEMO_IMAGE1, "../resources/wfc-demo1.xp");

pub struct RexAssets {
    pub main_menu: XpFile,
    pub game_over: XpFile,
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(SPLASH_SCREEN, "../resources/temple.xp");
        rltk::link_resource!(GAME_OVER, "../resources/coffin2.xp");
        rltk::link_resource!(WFC_DEMO_IMAGE1, "../resources/wfc-demo1.xp");

        RexAssets {
            main_menu: XpFile::from_resource("../resources/temple.xp").unwrap(),
            game_over: XpFile::from_resource("../resources/coffin2.xp").unwrap(),
        }
    }
}
