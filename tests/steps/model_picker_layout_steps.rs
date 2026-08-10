use cucumber::{then, when};

use crate::WatnWorld;

use super::{finish_pty_session, pty_snapshot, pty_write, start_pty_session};

#[when("I start `watn models` in a terminal")]
fn start_models_in_terminal(_world: &mut WatnWorld) {
    unimplemented!()
}

#[then(regex = r#"^the model picker should show a bordered \"([^\"]+)\" panel$"#)]
fn model_picker_bordered_panel(_world: &mut WatnWorld, _title: String) {
    unimplemented!()
}

#[then("the model picker should show tabs for the three model tiers")]
fn model_picker_tier_tabs(_world: &mut WatnWorld) {
    unimplemented!()
}

#[then("the model picker should show models in aligned columns")]
fn model_picker_table_columns(_world: &mut WatnWorld) {
    unimplemented!()
}

#[then("the model picker should show a scrollbar for the model list")]
fn model_picker_scrollbar(_world: &mut WatnWorld) {
    unimplemented!()
}

#[when("I move to the next model and advance to the normal tier")]
fn move_to_next_model_and_advance(_world: &mut WatnWorld) {
    unimplemented!()
}

#[then(regex = r#"^the model picker should show the active tier \"([^\"]+)\"$"#)]
fn model_picker_active_tier(_world: &mut WatnWorld, _tier: String) {
    unimplemented!()
}

#[then("the model picker should keep the selected row visible")]
fn model_picker_selected_row(_world: &mut WatnWorld) {
    unimplemented!()
}
