//! Community and project links: the Discord button (app bar) and the URLs About and Home list.
//! Icons are Lucide's (ISC); Discord's own logo is a trademark, so the button uses a chat icon
//! on GitHub's colour with the word "Help".

use craft::behavior::IconId;
use craft::ui::buttons::Button;
use craft::ui::contract::{Props, Show};
use craft::ui::primitives::IconSpec;
use craft::ui::style::DataColor;
use egui::{Color32, Ui};

use crate::VectorzApp;

/// Discord's brand colour (a colour, not a logo).
const DISCORD: Color32 = Color32::from_rgb(0x58, 0x65, 0xF2);

/// The Discord button: the app's filled push button with no edge, as production's app bar drew it, with Discord's colour
/// on its icon alone. It opens the community link.
pub fn discord_button(app: &mut VectorzApp, ui: &mut Ui) {
    let icon = IconSpec::svg(IconId::new("message-circle")).data_color(DataColor::new(DISCORD));
    let shown = Button::new(tl!("Help")).icon(icon).tooltip(tl!("Join the community on GitHub")).show(ui);
    if shown.clicks > 0 {
        app.open_link("help.discord");
    }
}

/// The website, app page and GitHub links as (English label, URL), in that order: the one source
/// About and Home list them from.
pub fn link_urls(app: &mut VectorzApp) -> [(&'static str, String); 3] {
    let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
    let url = |k: &str| l[k].as_str().unwrap_or("").to_string();
    [
        (tl!("Upstream project"), url("website")),
        (tl!("VectorZ on GitHub"), url("appPage")),
        (tl!("Source code on GitHub"), url("github")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The URLs the frame after `app`'s last action asked to open.
    fn opened_in_frame(app: &mut VectorzApp, ctx: &egui::Context) -> Vec<String> {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.tick(ui.ctx()));
        out.textures_delta.clear();
        out.platform_output
            .commands
            .into_iter()
            .filter_map(|c| match c {
                egui::OutputCommand::OpenUrl(open) => Some(open.url),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn link_clicks_open_the_right_urls() {
        let mut app = VectorzApp::new(vectorz_engine::Session::new(), Default::default());
        let ctx = crate::testing::ui_context();
        let mut opened = vec![];
        for id in ["help.discord", "help.website", "help.appPage", "help.github"] {
            crate::menus::invoke(&mut app, id, serde_json::json!({}));
            opened.extend(opened_in_frame(&mut app, &ctx));
        }
        assert_eq!(
            opened,
            [
                "https://github.com/danish0440/vectorz/issues",
                "https://github.com/danish0440/vectorz",
                "https://github.com/danish0440/vectorz",
                "https://github.com/danish0440/vectorz"
            ]
        );
        // The control channel / MCP path returns the URL without opening a browser.
        let v = app.run("help.discord", serde_json::json!({})).unwrap();
        assert_eq!(v["url"], "https://github.com/danish0440/vectorz/issues");
        assert!(opened_in_frame(&mut app, &ctx).is_empty(), "nothing more is opened");
    }

    #[test]
    fn link_urls_reads_the_three_help_links() {
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let mut app = VectorzApp::new(vectorz_engine::Session::new(), Default::default());
        let links = link_urls(&mut app);
        assert_eq!(links.each_ref().map(|(label, _)| *label), ["Upstream project", "VectorZ on GitHub", "Source code on GitHub"]);
        for (label, url) in &links {
            assert!(url.starts_with("https://"), "{label}: {url}");
        }
        assert_eq!(links, link_urls(&mut app), "the order is stable");
    }
}
