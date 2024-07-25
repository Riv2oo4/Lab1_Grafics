mod framebuffer;
mod line;
mod polygon;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);


    let poly1 = vec![
        (165, 380), 
        (185, 360), 
        (180, 330), 
        (207, 345), 
        (233, 330),
        (230, 360), 
        (250, 380), 
        (220, 385), 
        (205, 410), 
        (193, 383)
    ];
    
    framebuffer.set_current_color(0x00FFFF);
    framebuffer.fill_polygon(&poly1);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly1);

    let poly2 = vec![
        (321, 335),
        (288, 286),
        (339, 251),
        (374, 302)

    ];
    framebuffer.set_current_color(0xFF0000);
    framebuffer.fill_polygon(&poly2);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly2);

    let poly3=vec![
        (377, 249),
        (411, 197),
        (436, 249)

    ];
    framebuffer.set_current_color(0x0000FF);
    framebuffer.fill_polygon(&poly3);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly3);



    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
} 