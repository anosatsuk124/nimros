pub const FONT_SIZE: usize = 16;

pub static HANKAKU_FONT: Font = Font::new(include_bytes!("../../hankaku.bin"));

#[derive(Educe)]
#[educe(Deref)]
pub struct Font(&'static [u8; FONT_SIZE * 256]);

impl Font {
    pub const fn new(data: &'static [u8; FONT_SIZE * 256]) -> Self {
        Self(data)
    }

    pub fn get_font(&self, char: u8) -> Result<&'static [u8], ()> {
        let index = char as usize * FONT_SIZE;

        if index > self.len() {
            return Err(());
        }

        unsafe {
            let ptr = (*self).as_ptr().add(index);
            let slice = core::slice::from_raw_parts(ptr, FONT_SIZE);

            Ok(slice)
        }
    }
}
