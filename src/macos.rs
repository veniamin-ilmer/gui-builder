use cocoa::base::{ id, nil, NO, YES };
use cocoa::foundation::{ NSString, NSRect, NSSize, NSPoint };
use cocoa::appkit::{ NSApp, NSApplication, NSWindow, NSView, NSBackingStoreBuffered, NSRunningApplication, NSWindowStyleMask,
NSApplicationActivateIgnoringOtherApps, NSApplicationActivationPolicyRegular, NSFilenamesPboardType };

pub fn create_window(title: &str, width: f64, height: f64) {

        let window = unsafe { NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(width, height)),
                                                          NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO) };

        let view = unsafe { NSWindow::contentView(window) };

        unsafe {

            window.setAcceptsMouseMovedEvents_(YES);
            window.makeKeyAndOrderFront_(nil);
            window.makeFirstResponder_(view);

            let nstitle = NSString::alloc(nil).init_str(title);
            window.setTitle_(nstitle);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            app.activateIgnoringOtherApps_(YES);

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

            window.setOpaque_(YES);
            window.center();

        }

}




pub fn main() {
  create_window("test", 240_f64, 200_f64);
	unsafe {
            let app = NSApp();
            app.run();
	}
}
