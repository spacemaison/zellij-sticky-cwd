use std::path::PathBuf;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    cwd: Option<PathBuf>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[ EventType::Action, EventType::PaneUpdate ]);
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::Action(Action::NewPane(direction)) => {
                open_terminal(RunCommandAction {
                    current_dir: self.cwd.clone(),
                    direction,
                    ..Default::default()
                });
            }
            Event::PaneUpdate(panes) => {
                self.cwd = panes
                    .into_iter()
                    .find(|pane| pane.active)
                    .and_then(|pane| match pane.kind {
                        PaneKind::Terminal { cwd } => Some(cwd),
                        _ => None,
                    });
            }
            _ => {}
        }
    }
}
