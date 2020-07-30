use super::uikit::*;

static mut WINDOW: *mut Object = std::ptr::null_mut();

extern "C" fn scene_did_become_active(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    println!("ACTIVE");
}

extern "C" fn did_finish_launching(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    println!("FINISHED LAUNCHING");
}

extern "C" fn scene_will_connect_to_session(
    _this: &Object,
    _sel: Sel,
    scene: *mut Object,
    session: *mut Object,
    options: *mut Object,
) {
    println!("Connect to session");
    unsafe {
        let main_screen: *mut Object = msg_send![class!(UIScreen), mainScreen];
        let main_screen_bounds: CGRect = msg_send![main_screen, bounds];
        let ui_window: *mut Object = msg_send![class!(UIWindow), alloc];
        let ui_window: *mut Object = msg_send![ui_window, initWithFrame: main_screen_bounds];
        let () = msg_send![ui_window, makeKeyAndVisible];
        WINDOW = ui_window;
    }
}

extern "C" fn scene_did_disconnect(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    println!("Scene did disconnect");
}

extern "C" fn scene_will_resign_active(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    println!("Scene will resign active");
}

extern "C" fn scene_will_enter_foreground(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    println!("Scene will enter foreground");
}

extern "C" fn scene_will_enter_background(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    println!("Scene will enter background");
}

extern "C" fn window(_this: &Object, _sel: Sel) -> *mut Object {
    println!("Get window");
    //  unimplemented!();
    unsafe { WINDOW }
    //std::ptr::null_mut()
}

extern "C" fn set_window(_this: &Object, _sel: Sel, value: *mut Object) {
    println!("Set window");
}

extern "C" fn configuration_for_scene(
    _this: &Object,
    _sel: Sel,
    application: *mut Object,
    session: *mut Object,
    options: *mut Object,
) -> *mut Object {
    println!("Configuration for scene");
    unimplemented!()
}

extern "C" fn touches_began_with_event(
    _this: &Object,
    _sel: Sel,
    touches: *mut Object,
    event: *mut Object,
) {
    println!("TOUCH");
}
pub fn add_application_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
        let ui_window_scene_delegate_protocol = Protocol::get("UIWindowSceneDelegate").unwrap();
        decl.add_protocol(&ui_window_scene_delegate_protocol);

        decl.add_method(
            sel!(touchesBegan:withEvent:),
            touches_began_with_event as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
        );

        decl.add_method(
            sel!(application:configurationForConnectingSceneSession:options:),
            configuration_for_scene
                as extern "C" fn(
                    &Object,
                    Sel,
                    *mut Object,
                    *mut Object,
                    *mut Object,
                ) -> *mut Object,
        );
        decl.add_method(
            sel!(sceneDidBecomeActive:),
            scene_did_become_active as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(scene:willConnectToSession:options:),
            scene_will_connect_to_session
                as extern "C" fn(&Object, Sel, *mut Object, *mut Object, *mut Object),
        );
        decl.add_method(
            sel!(sceneDidDisconnect:),
            scene_did_disconnect as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(sceneWillResignActive:),
            scene_will_resign_active as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(sceneWillEnterForeground:),
            scene_will_enter_foreground as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(sceneDidEnterBackground:),
            scene_will_enter_background as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(window),
            window as extern "C" fn(&Object, Sel) -> *mut Object,
        );
        decl.add_method(
            sel!(setWindow:),
            set_window as extern "C" fn(&Object, Sel, *mut Object),
        );
    }
}
