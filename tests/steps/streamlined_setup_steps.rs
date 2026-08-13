use cucumber::then;

use crate::WatnWorld;

// RED proof for the active setup contract. Replace this with a real assertion
// before the first scenario is marked GREEN.
#[then("the setup coordinator should show the provider question first")]
fn setup_coordinator_provider_question(_world: &mut WatnWorld) {
    unimplemented!()
}
