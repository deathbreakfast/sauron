#[macro_use]
extern crate log;
use sauron_core::{
    diff,
    dom::DomUpdater,
    html::{attributes::*, events::*, *},
    mt_dom::patch::*,
    Node,
};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn nodes_with_event_should_not_recycle() {
    console_log::init_with_level(log::Level::Trace).ok();

    let old: Node<()> = div(
        vec![class("container")],
        vec![div(
            vec![class("child"), on_click(|_| log::trace!("I'm a div"))],
            vec![],
        )],
    );

    let new: Node<()> = div(
        vec![class("container")],
        vec![div(vec![class("child")], vec![])],
    );

    let diff = diff(&old, &new);
    log::info!("{:#?}", diff);
    assert_eq!(
        diff,
        vec![ReplaceNode::new(
            Some(&"div"),
            TreePath::new(vec![0, 0]),
            &div(vec![class("child")], vec![])
        )
        .into()]
    );
}

#[wasm_bindgen_test]
fn remove_event_from_replaced_node() {
    console_log::init_with_level(log::Level::Trace).ok();

    let old: Node<()> = div(vec![on_click(|_| trace!("I'm a div"))], vec![]);

    let new: Node<()> = p(vec![], vec![]);

    let body = sauron_core::body();
    let simple_program = simple_program();
    let diff = sauron_core::diff(&old, &new);
    log::info!("{:#?}", diff);
    assert_eq!(
        diff,
        vec![ReplaceNode::new(
            Some(&"div"),
            TreePath::new(vec![0]),
            &p(vec![], vec![])
        )
        .into()],
    );
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    assert_eq!(
        dom_updater.active_closure_len(),
        1,
        "There should be 1 event attached to the DomUpdater"
    );
    dom_updater.update_dom(&simple_program, new);

    assert_eq!(
        dom_updater.active_closure_len(),
        0,
        "There should only be 0 left after replacing it with a different tag"
    );
}
