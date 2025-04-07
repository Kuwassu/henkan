use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

// キーコード定数
const VK_CONVERT: u16 = 0x1C;      // 変換キー
const VK_NONCONVERT: u16 = 0x1D;   // 無変換キー
const VK_I: u16 = 0x49;            // Iキー
const VK_J: u16 = 0x4A;            // Jキー
const VK_K: u16 = 0x4B;            // Kキー
const VK_L: u16 = 0x4C;            // Lキー
const VK_UP: u16 = 0x26;           // ↑キー
const VK_LEFT: u16 = 0x25;         // ←キー
const VK_DOWN: u16 = 0x28;         // ↓キー
const VK_RIGHT: u16 = 0x27;        // →キー

struct KeyState {
    convert_pressed: bool,
    nonconvert_pressed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("キー変換アプリケーションを起動しています...");

    // トレイアイコン設定（実際のアプリではこちらを実装）
    // setup_tray_icon()?;

    // グローバルな終了フラグ
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Ctrl+Cで終了できるようにする
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("終了しています...");
    })?;

    // キー状態を管理する
    let key_state = Arc::new(Mutex::new(KeyState {
        convert_pressed: false,
        nonconvert_pressed: false,
    }));

    // キーボードフックを設定
    let hook_key_state = key_state.clone();
    let hook_running = running.clone();

    // 安全なキーボードフックのセットアップ
    unsafe {
        let hook_id = SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            HINSTANCE(0),
            0,
        )?;

        // ユーザーデータをスレッドローカルストレージに保存
        // 実際のアプリでは、より安全な方法でフックプロシージャにデータを渡す仕組みが必要
        thread_local_storage_set_key_state(hook_key_state);
        thread_local_storage_set_running(hook_running);

        // メッセージループ
        let mut msg = MSG::default();
        while running.load(Ordering::SeqCst) {
            if PeekMessageA(&mut msg, HWND(0), 0, 0, PM_REMOVE).into() {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            thread::sleep(Duration::from_millis(10));
        }

        // フックを解除
        UnhookWindowsHookEx(hook_id)?;
    }

    println!("アプリケーションを終了しました。");
    Ok(())
}

// スレッドローカルストレージ用のヘルパー関数（実際のコードではもっと堅牢に実装する必要があります）
fn thread_local_storage_set_key_state(key_state: Arc<Mutex<KeyState>>) {
    // 実装省略 - 実際のコードではスレッドローカルストレージを適切に実装
}

fn thread_local_storage_set_running(running: Arc<AtomicBool>) {
    // 実装省略 - 実際のコードではスレッドローカルストレージを適切に実装
}

fn thread_local_storage_get_key_state() -> Option<Arc<Mutex<KeyState>>> {
    // 実装省略 - 実際のコードではスレッドローカルストレージを適切に実装
    None // プレースホルダー
}

fn thread_local_storage_get_running() -> Option<Arc<AtomicBool>> {
    // 実装省略 - 実際のコードではスレッドローカルストレージを適切に実装
    None // プレースホルダー
}

// キーボードフックプロシージャ
extern "system" fn keyboard_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if code < 0 {
            return CallNextHookEx(HHOOK(0), code, wparam, lparam);
        }

        // オプション処理を改善
        let key_state_opt = thread_local_storage_get_key_state();
        let running_opt = thread_local_storage_get_running();

        if key_state_opt.is_none() || running_opt.is_none() {
            return CallNextHookEx(HHOOK(0), code, wparam, lparam);
        }

        let key_state = key_state_opt.unwrap();
        let running = running_opt.unwrap();

        if !running.load(Ordering::SeqCst) {
            return CallNextHookEx(HHOOK(0), code, wparam, lparam);
        }

        // キーイベント情報を取得
        let kb_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
        let virtual_key = kb_struct.vkCode as u16;
        let is_key_down = wparam.0 == WM_KEYDOWN as usize || wparam.0 == WM_SYSKEYDOWN as usize;
        let is_key_up = wparam.0 == WM_KEYUP as usize || wparam.0 == WM_SYSKEYUP as usize;

        // キー状態を更新
        let mut should_block_key = false;

        if let Ok(mut state) = key_state.lock() {
            // 変換キーの状態を更新
            if virtual_key == VK_CONVERT {
                if is_key_down {
                    state.convert_pressed = true;
                } else if is_key_up {
                    state.convert_pressed = false;
                }
            }

            // 無変換キーの状態を更新
            if virtual_key == VK_NONCONVERT {
                if is_key_down {
                    state.nonconvert_pressed = true;
                } else if is_key_up {
                    state.nonconvert_pressed = false;
                }
            }

            // 変換キーが押されているときの追加キー処理
            if state.convert_pressed && is_key_down {
                match virtual_key {
                    VK_I => {
                        // 変換+I → 上矢印
                        send_key(VK_UP);
                        should_block_key = true;
                    },
                    VK_K => {
                        // 変換+K → 下矢印
                        send_key(VK_DOWN);
                        should_block_key = true;
                    },
                    VK_J => {
                        // 変換+J → 左矢印
                        send_key(VK_LEFT);
                        should_block_key = true;
                    },
                    VK_L => {
                        // 変換+L → 右矢印
                        send_key(VK_RIGHT);
                        should_block_key = true;
                    },
                    _ => {}
                }
            }

            // 無変換キーが押されているときの処理も同様に実装可能
            // 例: if state.nonconvert_pressed && is_key_down { ... }
        }

        // キーを処理した場合は元のイベントをブロック
        if should_block_key {
            return LRESULT(1);
        }

        CallNextHookEx(HHOOK(0), code, wparam, lparam)
    }
}

// キー送信関数
fn send_key(key_code: u16) {
    unsafe {
        let mut inputs: [INPUT; 2] = [INPUT::default(), INPUT::default()];

        // キーを押す
        inputs[0].r#type = INPUT_KEYBOARD;
        inputs[0].Anonymous.ki.wVk = key_code;

        // キーを離す
        inputs[1].r#type = INPUT_KEYBOARD;
        inputs[1].Anonymous.ki.wVk = key_code;
        inputs[1].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

// トレイアイコン設定（実装省略）
fn setup_tray_icon() -> Result<(), Box<dyn std::error::Error>> {
    // トレイアイコンの実装
    // 実際のアプリでは、tray-iconなどのクレートを使用することを推奨
    Ok(())
}