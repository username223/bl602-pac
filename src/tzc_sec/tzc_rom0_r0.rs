#[doc = "Register `tzc_rom0_r0` reader"]
pub struct R(crate::R<TZC_ROM0_R0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ROM0_R0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TZC_ROM0_R0_SPEC>> for R {
    fn from(reader: crate::R<TZC_ROM0_R0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_rom0_r0` writer"]
pub struct W(crate::W<TZC_ROM0_R0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_ROM0_R0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<TZC_ROM0_R0_SPEC>> for W {
    fn from(writer: crate::W<TZC_ROM0_R0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_rom0_r0_start` reader - "]
pub struct TZC_ROM0_R0_START_R(crate::FieldReader<u16, u16>);
impl TZC_ROM0_R0_START_R {
    pub(crate) fn new(bits: u16) -> Self {
        TZC_ROM0_R0_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R0_START_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r0_start` writer - "]
pub struct TZC_ROM0_R0_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `tzc_rom0_r0_end` reader - "]
pub struct TZC_ROM0_R0_END_R(crate::FieldReader<u16, u16>);
impl TZC_ROM0_R0_END_R {
    pub(crate) fn new(bits: u16) -> Self {
        TZC_ROM0_R0_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R0_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r0_end` writer - "]
pub struct TZC_ROM0_R0_END_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r0_start(&self) -> TZC_ROM0_R0_START_R {
        TZC_ROM0_R0_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r0_end(&self) -> TZC_ROM0_R0_END_R {
        TZC_ROM0_R0_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r0_start(&mut self) -> TZC_ROM0_R0_START_W {
        TZC_ROM0_R0_START_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r0_end(&mut self) -> TZC_ROM0_R0_END_W {
        TZC_ROM0_R0_END_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_rom0_r0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom0_r0](index.html) module"]
pub struct TZC_ROM0_R0_SPEC;
impl crate::RegisterSpec for TZC_ROM0_R0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_rom0_r0::R](R) reader structure"]
impl crate::Readable for TZC_ROM0_R0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_rom0_r0::W](W) writer structure"]
impl crate::Writable for TZC_ROM0_R0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tzc_rom0_r0 to value 0"]
impl crate::Resettable for TZC_ROM0_R0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
