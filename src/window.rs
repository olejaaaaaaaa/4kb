
use windows_sys::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::{Gdi::*, OpenGL::*},
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};

use core::{mem, ptr};

use crate::gl;

pub fn handle_message(_window: HWND) -> bool {
    let mut msg: mem::MaybeUninit<MSG> = mem::MaybeUninit::uninit();
    loop {
        unsafe {
            if PeekMessageA(msg.as_mut_ptr(), 0 as HWND, 0, 0, PM_REMOVE) == 0 {
                return true;
            }
            let msg = msg.assume_init();
            if msg.message == WM_QUIT {
                return false;
            }

            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

#[must_use]
pub fn create() -> (HWND, HDC) {
    unsafe {
        let instance = GetModuleHandleA(ptr::null());

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(0, IDC_ARROW),
            hInstance: instance,
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: 0,
            hbrBackground: 0,
            lpszMenuName: ptr::null(),
        };

        let _atom = RegisterClassA(&wc);

        let title = c"Pixel";

        let h_wnd = CreateWindowExA(
            0,
            window_class,
            title.as_ptr() as *const _,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            720,
            640,
            0 as HWND,
            0 as HMENU,
            instance,
            ptr::null(),
        );

        let h_dc: HDC = GetDC(h_wnd);

        let mut pfd: PIXELFORMATDESCRIPTOR = mem::zeroed();
        pfd.nSize = mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
        pfd.nVersion = 1;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.cColorBits = 32;
        pfd.cAlphaBits = 8;
        pfd.cDepthBits = 24;

        let pfd_id = ChoosePixelFormat(h_dc, &pfd);
        SetPixelFormat(h_dc, pfd_id, &pfd);

        let gl_context: HGLRC = wglCreateContext(h_dc);

        wglMakeCurrent(h_dc, gl_context);

        gl::init();

        (h_wnd, h_dc)
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                ValidateRect(window, ptr::null());
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}