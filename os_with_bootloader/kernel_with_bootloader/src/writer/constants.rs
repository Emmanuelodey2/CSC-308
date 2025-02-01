use noto_sans_mono_bitmap::{ get_raster_width, FontWeight, RasterHeight };

pub mod font_constants {
    use super::*;
    
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
    pub const BACKUP_CHAR: char = '\t';
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
    pub const BACKSPACE: char = '\u{0008}';
}
