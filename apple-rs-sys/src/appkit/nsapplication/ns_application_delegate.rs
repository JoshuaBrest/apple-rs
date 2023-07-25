use crate::{class_name, utils::*};

trait NSApplicationDelegate {
    /* Launching Applications */

    /// Called when the application initialization is *almost* complete.
    fn applicationWillFinishLaunching(&self, notification: UnsafeId);
    /// Called when the application initialization is complete.
    fn applicationDidFinishLaunching(&self, notification: UnsafeId);

    /* Managing Active Status */
}

#[inline]
pub fn init_application_delegate() {
    get_class(
        "NSObject",
        &class_name!("NSApplicationDelegate"),
        |decl| unsafe {
            // decl.add_method(
            //     sel!(applicationDidFinishLaunching:),
            //     application_did_finish_launching as extern "C" fn(&mut Object, Sel, Id),
            // );

            // decl.add_method(
            //     sel!(applicationWillTerminate:),
            //     application_will_terminate as extern "C" fn(&mut Object, Sel, Id),
            // );
        },
    );
}
