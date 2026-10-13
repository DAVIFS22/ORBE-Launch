//! The ORBE Apps: the twelve apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

use serde::{Deserialize, Serialize};

/// Which generation of the suite an app belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppGroup {
    /// The seven original apps, each with a page on github.com/SEU_USUARIO.
    Classic,
    /// The five apps added after them.
    New,
}

impl AppGroup {
    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::New => "New",
        }
    }
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// Display name (`PhotoCraft`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// The familiar app it reimplements, for orientation.
    pub like: &'static str,
    pub group: AppGroup,
    /// Brand colour (sRGB), for surfaces: github.com/SEU_USUARIO's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (PdfCraft shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
}

impl CraftApp {
    /// The product it's inspired by, with its registered-trademark sign (`Adobe Photoshop®`).
    pub fn inspired_by(&self) -> String {
        self.like.to_string()
    }

    /// `ai.storyteller.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("br.orbe.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page on github.com/SEU_USUARIO (classic apps only).
    pub fn website(&self) -> Option<String> {
        None
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    like: &'static str,
    group: AppGroup,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp { id, name, tagline, like, group, color, ink_light, ink_dark, repo, former_ids: &[] }
}

/// ORBE product catalog. Configure repositories before publishing releases.
pub const SUITE: [CraftApp; 2] = [
 app("orbe-studio", "ORBE Studio", "Edição e criação de imagens", "", AppGroup::Classic, [[0x2f,0x7b,0xf5],[0x1a,0x5b,0xc4],[0x7f,0xb0,0xff]], "SEU_USUARIO/ORBE-Studio"),
 app("orbe-motion", "ORBE Motion", "Motion graphics e efeitos visuais", "", AppGroup::New, [[0x38,0x83,0xf6],[0x1a,0x5b,0xc4],[0x8a,0xbd,0xff]], "SEU_USUARIO/ORBE-Motion"),
];
pub const TRADEMARK_NOTICE: &str = "ORBE Studio and ORBE Motion are product names of ORBE. All other trademarks belong to their respective owners.";
pub const WEBSITE_URL: &str = "https://github.com/SEU_USUARIO";
pub const GITHUB_URL: &str = "https://github.com/SEU_USUARIO";
pub const SOCIAL: [(&str, &str); 0] = [];
pub const DISCORD_URL: &str = "https://github.com/SEU_USUARIO/ORBE-Launcher/discussions";
pub const LAUNCHER: CraftApp = app("orbe-launcher", "ORBE Launcher", "Instala e atualiza os aplicativos ORBE", "", AppGroup::New, [[0x0b,0x19,0x33],[0x0b,0x19,0x33],[0x80,0xae,0xff]], "SEU_USUARIO/ORBE-Launcher");
pub const LAUNCHER_REPO: &str = "SEU_USUARIO/ORBE-Launcher";
pub const LAUNCHER_ID: &str = "orbe-launcher";
#[cfg(test)] mod tests { use super::*; #[test] fn suite_has_two_orbe_apps_with_unique_ids() { assert_eq!(SUITE.len(),2); let mut ids:Vec<_>=SUITE.iter().map(|a|a.id).collect(); ids.sort_unstable(); ids.dedup(); assert_eq!(ids.len(),SUITE.len()); assert_eq!(CraftApp::by_id("orbe-studio").map(|a|a.name),Some("ORBE Studio")); assert_eq!(CraftApp::by_id("orbe-motion").map(|a|a.name),Some("ORBE Motion")); } }
