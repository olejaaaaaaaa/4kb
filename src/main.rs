#![no_main]
#![no_std]
#![windows_subsystem = "windows"]

use music::play;
use windows_sys::Win32::{Graphics::OpenGL::SwapBuffers, System::{Memory::{GetProcessHeap, HeapAlloc}, Threading::ExitProcess}};

mod gl;
mod window;
mod music;
mod random;


struct CustomAllocator;

unsafe impl GlobalAlloc for CustomAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        unsafe { HeapAlloc(GetProcessHeap(), 0, layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: alloc::Layout) {}
}

#[global_allocator]
static ALLOC: CustomAllocator = CustomAllocator;


#[unsafe(no_mangle)]
fn main() -> ! {

    play();

    let (HWND, HDC) = window::create();

    unsafe {

    let vtx_shader_src: &'static str = concat!(include_str!("../shaders/vs.glsl"), "\0");
    let frag_shader_src: &'static str = concat!(include_str!("../shaders/fs_sky.glsl"), "\0\0");

    let vtx_coords: [[gl::GlFloat; 3]; 4] =
        [[-1., -1., 0.], [1., -1., 0.], [-1., 1., 0.], [1., 1., 0.]];

    let vtx_shader = gl::shader_from_source(vtx_shader_src, gl::VERTEX_SHADER);
    let frag_shader = gl::shader_from_source(frag_shader_src, gl::FRAGMENT_SHADER);
    let shader_prog = gl::program_from_shaders(vtx_shader, frag_shader);

    // OpenGL setup
    let mut vertex_buffer_id: gl::GlUint = 0;
    let mut vertex_array_id: gl::GlUint = 0;

    unsafe {
        gl::GenBuffers(1, &mut vertex_buffer_id);
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);

        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of::<gl::GlFloat>() as isize * 12,
            vtx_coords.as_ptr() as *const gl::CVoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<gl::GlFloat>() as gl::GlInt,
            0 as *const gl::CVoid,
        );
    }

    let mut tick = 0.0;

    loop {

        if !window::handle_message(HWND) {
            break;
        }

        let rgba = &[0., 0., 0., 0.];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, rgba.as_ptr());
            gl::UseProgram(shader_prog);

            let tick_loc = gl::GetUniformLocation(shader_prog, "iTime\0".as_ptr());
            gl::Uniform1f(tick_loc, tick);

            gl::BindVertexArray(vertex_array_id);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            SwapBuffers(HDC);
        }

        tick += 0.1;

    }

        ExitProcess(0);
    }
}

use core::{alloc::{self, GlobalAlloc}, mem, panic::PanicInfo};
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}