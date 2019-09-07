//! Switching keybind modes: TGTracker (Based on MilkyTracker) and FastTracker2
//! Differences documented in specs
use keybind::{Keybind, Keycode};
/// tgtracker section switching global keys
pub fn tgtracker_section_switching() {
    let mut adv_edit = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::A]);
    adv_edit.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+A");
    });
    adv_edit.wait();
    let mut config = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::C]);
    config.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+C");
    });
    config.wait();
    let mut disk_op = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::D]);
    disk_op.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+D");
    });
    disk_op.wait();
    let mut inst_edit = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::I]);
    inst_edit.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+I");
    });
    inst_edit.wait();
    let mut disk_rec = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::R]);
    disk_rec.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+R");
    });
    disk_rec.wait();
    let mut phrase_edit = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::P]);
    phrase_edit.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+P");
    });
    phrase_edit.wait();
    let mut transpose = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::T]);
    transpose.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+T");
    });
    transpose.wait();
    let mut main_screen = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::X]);
    main_screen.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+X");
    });
    main_screen.wait();
    let mut tgl_scopes = Keybind::new(&[Keycode::LControl, Keycode::LAlt, Keycode::Z]);
    tgl_scopes.on_trigger(|| {
        println!("This will be printed when you press CTRL+ALT+Z");
    });
    tgl_scopes.wait();
}
