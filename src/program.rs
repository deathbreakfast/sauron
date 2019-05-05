use crate::{Component,
            Dispatch,
            DomUpdater};
use std::{cell::RefCell,
          rc::Rc};
use wasm_bindgen::closure::Closure;
use web_sys::Node;

/// Holds the app and the dom updater
/// This is passed into the event listener and the dispatch program
/// will be called after the event is triggered.
pub struct Program<APP, MSG> {
    pub app: Rc<RefCell<APP>>,
    pub dom_updater: Rc<RefCell<DomUpdater<Self, MSG>>>,
}

impl<APP, MSG> Program<APP, MSG>
    where MSG: Clone + 'static,
          APP: Component<MSG> + 'static
{
    /// Create an Rc wrapped instance of program, initializing DomUpdater with the initial view
    /// and root node, but doesn't mount it yet.
    fn new(app: APP, root_node: &Node) -> Rc<Self> {
        let dom_updater: DomUpdater<Self, MSG> =
            DomUpdater::new(app.view(), root_node);
        let program = Program { app: Rc::new(RefCell::new(app)),
                                dom_updater:
                                    Rc::new(RefCell::new(dom_updater)) };
        Rc::new(program)
    }

    /// Creates an Rc wrapped instance of Program and mount the app view to the
    /// given root_node
    pub fn new_replace_mount(app: APP, root_node: &Node) -> Rc<Self> {
        let program = Self::new(app, root_node);
        program.start_replace_mount();
        program
    }

    pub fn new_append_to_mount(app: APP, root_node: &Node) -> Rc<Self> {
        let program = Self::new(app, root_node);
        program.start_append_to_mount();
        program
    }

    /// Instantiate the app and then append it to the document body
    pub fn mount_to_body(app: APP) -> Rc<Self> {
        Self::new_append_to_mount(app, &crate::body())
    }

    fn start_append_to_mount(self: &Rc<Self>) {
        self.dom_updater.borrow_mut().append_to_mount(self)
    }

    fn start_replace_mount(self: &Rc<Self>) {
        self.dom_updater.borrow_mut().replace_mount(self)
    }

    /// This is called when an event is triggered in the html DOM.
    fn dispatch_inner(self: &Rc<Self>, msg: MSG) {
        //let t1 = crate::now();
        self.app.borrow_mut().update(msg);
        //let t2 = crate::now();
        //crate::log!("app update took: {}ms", t2 - t1);
        let view = self.app.borrow().view();
        //let t3 = crate::now();
        //crate::log!("app view took: {}ms", t3 - t2);
        self.dom_updater.borrow_mut().update(self, view);
        //let t4 = crate::now();
        //crate::log!("dom update took: {}ms", t4 - t3);
    }
}

/// This will be called when the actual event is triggered.
/// Defined in the DomUpdater::create_closure_wrap function
impl<APP, MSG> Dispatch<MSG> for Program<APP, MSG>
    where MSG: Clone + 'static,
          APP: Component<MSG> + 'static
{
    fn dispatch(self: &Rc<Self>, msg: MSG) {
        let program_clone = Rc::clone(self);
        let closure_raf: Closure<FnMut() + 'static> =
            Closure::once(move || {
                program_clone.dispatch_inner(msg);
            });
        crate::request_animation_frame(&closure_raf);
        closure_raf.forget();
    }
}
