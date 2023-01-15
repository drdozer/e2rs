//! Work with image representations of boards.
//! 
use image::Rgba;
use image::imageops::{rotate90, rotate180, rotate270};
use lazy_static::lazy_static;
use image::{self, load_from_memory, DynamicImage, GenericImageView, ImageBuffer, imageops::overlay};

use crate::{e2::E2_EDGE_COUNT, board::{Tile, Side::*}};

use super::E2Edge;

lazy_static! {
    /// Edge images.
    pub static ref IMAGES: [DynamicImage; E2_EDGE_COUNT] = [
        load_from_memory(
            include_bytes!("../../data/edge_images/0.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/1.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/2.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/3.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/4.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/5.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/6.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/7.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/8.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/9.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/10.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/11.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/12.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/13.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/14.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/15.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/16.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/17.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/18.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/19.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/20.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/21.png"))
            .expect("Failed to load image resource"),
        load_from_memory(
            include_bytes!("../../data/edge_images/22.png"))
            .expect("Failed to load image resource"),
            ];
}

/// Render a tile as an image.
pub fn edge_image(tile: Tile<E2Edge>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(IMAGES[0].dimensions().0, IMAGES[0].dimensions().1);
    
    overlay(&mut img, &IMAGES[tile[North] as usize], 0, 0);
    overlay(&mut img, &rotate90 (&IMAGES[tile[East]  as usize]), 0, 0);
    overlay(&mut img, &rotate180(&IMAGES[tile[South] as usize]), 0, 0);
    overlay(&mut img, &rotate270(&IMAGES[tile[West]  as usize]), 0, 0);

    img
}

