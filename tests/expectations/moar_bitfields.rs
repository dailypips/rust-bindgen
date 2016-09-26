/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(u32)]
pub enum WhenToScroll {
    SCROLL_ALWAYS = 0,
    SCROLL_IF_NOT_VISIBLE = 1,
    SCROLL_IF_NOT_FULLY_VISIBLE = 2,
}
#[repr(C)]
pub struct ScrollAxis {
    pub mWhereToScroll: ::std::os::raw::c_short,
    pub _bitfield_1: u16,
}
#[test]
fn bindgen_test_layout_ScrollAxis() {
    assert_eq!(::std::mem::size_of::<ScrollAxis>() , 4usize);
    assert_eq!(::std::mem::align_of::<ScrollAxis>() , 4usize);
}
impl ScrollAxis {
    #[inline]
    pub fn mWhenToScroll(&self) -> WhenToScroll {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 & (255usize as u16)) >>
                                       0u32) as u32)
        }
    }
    #[inline]
    pub fn set_mWhenToScroll(&mut self, val: WhenToScroll) {
        self._bitfield_1 &= !(255usize as u16);
        self._bitfield_1 |= ((val as u32 as u16) << 0u32) & (255usize as u16);
    }
    #[inline]
    pub fn mOnlyIfPerceivedScrollableDirection(&self) -> bool {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 & (256usize as u16)) >>
                                       8u32) as u8)
        }
    }
    #[inline]
    pub fn set_mOnlyIfPerceivedScrollableDirection(&mut self, val: bool) {
        self._bitfield_1 &= !(256usize as u16);
        self._bitfield_1 |= ((val as u8 as u16) << 8u32) & (256usize as u16);
    }
}