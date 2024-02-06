
pub use win32_rs::Window;

fn main() {
    println!("Hello, world!");

    // let display_handle =  egl_rs::get_display();
    // println!("display: {:#X?}", display_handle);
    let window = Window::new(800, 480, "OpenGL ES 2.0");
    
    let _context = egl_rs::Context::create(window.get_handle() as _, 800, 480, true);
        
        
    
    window.show();
}
