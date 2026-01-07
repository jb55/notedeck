use crate::ui::{edge_ui, node_ui};
use egui::{Pos2, Rect};
use jsoncanvas::JsonCanvas;
use notedeck::{AppContext, AppResponse};

mod ui;

pub struct Notebook {
    canvas: JsonCanvas,
    scene_rect: Rect,
    loaded: bool,
}

impl Notebook {
    pub fn new() -> Self {
        Notebook::default()
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Notebook {
            canvas: demo_canvas(),
            scene_rect: Rect::from_min_max(Pos2::ZERO, Pos2::ZERO),
            loaded: false,
        }
    }
}

impl notedeck::App for Notebook {
    fn update(&mut self, ctx: &mut AppContext<'_>, ui: &mut egui::Ui) -> AppResponse {
        use rand::SeedableRng;
        //let app_action: Option<AppAction> = None;

        if !self.loaded {
            self.scene_rect = ui.available_rect_before_wrap();
            self.loaded = true;
        }

        // TODO(jb55): make this less horrible
        let mut note_context = notedeck::NoteContext {
            ndb: ctx.ndb,
            accounts: ctx.accounts,
            img_cache: ctx.img_cache,
            note_cache: ctx.note_cache,
            zaps: ctx.zaps,
            pool: ctx.pool,
            jobs: ctx.media_jobs.sender(),
            unknown_ids: ctx.unknown_ids,
            clipboard: ctx.clipboard,
            i18n: ctx.i18n,
            global_wallet: ctx.global_wallet,
        };

        let mut rng = rand::rngs::SmallRng::seed_from_u64(4);

        egui::Scene::new().show(ui, &mut self.scene_rect, |ui| {
            // render nodes
            for (_node_id, node) in self.canvas.get_nodes().iter() {
                let _resp = node_ui(&mut rng, &mut note_context, ui, node);
            }

            // render edges
            for (_edge_id, edge) in self.canvas.get_edges().iter() {
                let _resp = edge_ui(ui, self.canvas.get_nodes(), edge);
            }
        });

        AppResponse::none()
    }
}

fn demo_canvas() -> JsonCanvas {
    let demo_json: String = include_str!("../demo.canvas").to_string();

    let canvas: JsonCanvas = demo_json.parse().unwrap_or_else(|_| JsonCanvas::default());
    canvas
}
