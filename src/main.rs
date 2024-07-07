use inputbot::{KeybdKey::*, *};
use lazy_static::lazy_static;
use std::{sync::Mutex, thread::sleep, time::Duration};
const ZQSD: [KeybdKey; 4] = [ZKey, QKey, SKey, DKey];

lazy_static! {
    static ref last_pressed: Mutex<Option<KeybdKey>> = Mutex::new(None);
    static ref enabled: Mutex<bool> = Mutex::new(true);
}
fn main() {
    println!(" ____  ____  ____ ");
    println!("/ ___)(  __)(_  _)");
    println!("\\___ \\ ) _)   )(");
    println!("(____/(__)   (__) ");
    println!("\t> https://noais.fr");
    println!("\n");
    println!("Press [=] to enable/disable the script");
    println!("One tap [Z], [Q], [S] or [D] to move once ur ready to farm");
    println!("\n⋅.˳˳.⋅ॱ˙˙ॱ⋅.˳˳.⋅ॱ˙˙ॱᐧ.˳˳.⋅⋅.˳˳.⋅ॱ˙˙ॱ⋅.˳˳.⋅ॱ˙˙ॱᐧ.˳˳.⋅\n");
    KeybdKey::bind_all(|_| {
        return;
    });
    EqualKey.bind(|| {
        println!("script status is : {}", enabled.lock().unwrap());
        let current = *enabled.lock().unwrap();
        *enabled.lock().unwrap() = !current;
        println!("script status is now : {}", enabled.lock().unwrap());
        release_all();
    });
    for &key in ZQSD.iter() {
        key.bind(move || {
            if !*enabled.lock().unwrap() {
                return;
            }

            let lp = last_pressed.lock().unwrap().unwrap_or(FKey);
            if key == lp {
                return;
            } else {
                *last_pressed.lock().unwrap() = Some(key);
            }

            release_all();
            key.press();
            sleep(Duration::from_millis(get_u64_between_0_and_10()));
            MouseButton::LeftButton.press();
            println!("Pressed {:?}", key);
            println!("Pressed LeftButton");
            println!("----")
        });
    }

    // Call this to start listening for bound inputs.
    handle_input_events();
}

fn release_all() {
    println!("r-");
    for &key in ZQSD.iter() {
        key.release();
    }
    MouseButton::LeftButton.release();
}

fn get_u64_between_0_and_10() -> u64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(0..10)
}
