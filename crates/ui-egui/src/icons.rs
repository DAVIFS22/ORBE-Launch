//! Bundled images: each Craft App's icon and the launcher's own.

use egui::ImageSource;

/// Icon for ORBE products.
pub fn app(id: &str) -> Option<ImageSource<'static>> { Some(match id {
 "orbe-studio" => egui::include_image!("../../../assets/app-icons/orbe-studio.png"),
 "orbe-motion" => egui::include_image!("../../../assets/app-icons/orbe-motion.png"),
 _ => return None,
}) }
pub fn artcraft_logo(dark_background: bool) -> ImageSource<'static> { if dark_background { egui::include_image!("../../../docs/brand/orbe-logo-white.svg") } else { egui::include_image!("../../../docs/brand/orbe-logo.svg") } }
pub const ARTCRAFT_LOGO_ASPECT: f32 = 4.8;

/// A social service's logo (Simple Icons, white, for tinting), by its name in `catalog::SOCIAL`.
pub fn social(service: &str) -> Option<ImageSource<'static>> {
    Some(match service {
        "X" => egui::include_image!("../../../assets/social/x.svg"),
        "YouTube" => egui::include_image!("../../../assets/social/youtube.svg"),
        "TikTok" => egui::include_image!("../../../assets/social/tiktok.svg"),
        "Instagram" => egui::include_image!("../../../assets/social/instagram.svg"),
        "LinkedIn" => egui::include_image!("../../../assets/social/linkedin.svg"),
        _ => return None,
    })
}

/// The ORBE "A" mark in blue, as supplied: the launcher's app icon and github.com/SEU_USUARIO's favicon.
pub fn artcraft_mark() -> ImageSource<'static> {
    egui::include_image!("../../../docs/brand/orbe-mark.svg")
}
