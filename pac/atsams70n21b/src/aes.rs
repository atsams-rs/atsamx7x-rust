#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub aes_cr: AES_CR,
    #[doc = "0x04 - Mode Register"]
    pub aes_mr: AES_MR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub aes_ier: AES_IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub aes_idr: AES_IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub aes_imr: AES_IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub aes_isr: AES_ISR,
    #[doc = "0x20 - Key Word Register"]
    pub aes_keywr: [AES_KEYWR; 8],
    #[doc = "0x40 - Input Data Register"]
    pub aes_idatar: [AES_IDATAR; 4],
    #[doc = "0x50 - Output Data Register"]
    pub aes_odatar: [AES_ODATAR; 4],
    #[doc = "0x60 - Initialization Vector Register"]
    pub aes_ivr: [AES_IVR; 4],
    #[doc = "0x70 - Additional Authenticated Data Length Register"]
    pub aes_aadlenr: AES_AADLENR,
    #[doc = "0x74 - Plaintext/Ciphertext Length Register"]
    pub aes_clenr: AES_CLENR,
    #[doc = "0x78 - GCM Intermediate Hash Word Register"]
    pub aes_ghashr: [AES_GHASHR; 4],
    #[doc = "0x88 - GCM Authentication Tag Word Register"]
    pub aes_tagr: [AES_TAGR; 4],
    #[doc = "0x98 - GCM Encryption Counter Value Register"]
    pub aes_ctrr: AES_CTRR,
    #[doc = "0x9c - GCM H Word Register"]
    pub aes_gcmhr: [AES_GCMHR; 4],
}
#[doc = "Control Register"]
pub struct AES_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod aes_cr;
#[doc = "Mode Register"]
pub struct AES_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod aes_mr;
#[doc = "Interrupt Enable Register"]
pub struct AES_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod aes_ier;
#[doc = "Interrupt Disable Register"]
pub struct AES_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod aes_idr;
#[doc = "Interrupt Mask Register"]
pub struct AES_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod aes_imr;
#[doc = "Interrupt Status Register"]
pub struct AES_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod aes_isr;
#[doc = "Key Word Register"]
pub struct AES_KEYWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Word Register"]
pub mod aes_keywr;
#[doc = "Input Data Register"]
pub struct AES_IDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Data Register"]
pub mod aes_idatar;
#[doc = "Output Data Register"]
pub struct AES_ODATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Data Register"]
pub mod aes_odatar;
#[doc = "Initialization Vector Register"]
pub struct AES_IVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initialization Vector Register"]
pub mod aes_ivr;
#[doc = "Additional Authenticated Data Length Register"]
pub struct AES_AADLENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional Authenticated Data Length Register"]
pub mod aes_aadlenr;
#[doc = "Plaintext/Ciphertext Length Register"]
pub struct AES_CLENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Plaintext/Ciphertext Length Register"]
pub mod aes_clenr;
#[doc = "GCM Intermediate Hash Word Register"]
pub struct AES_GHASHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCM Intermediate Hash Word Register"]
pub mod aes_ghashr;
#[doc = "GCM Authentication Tag Word Register"]
pub struct AES_TAGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCM Authentication Tag Word Register"]
pub mod aes_tagr;
#[doc = "GCM Encryption Counter Value Register"]
pub struct AES_CTRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCM Encryption Counter Value Register"]
pub mod aes_ctrr;
#[doc = "GCM H Word Register"]
pub struct AES_GCMHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCM H Word Register"]
pub mod aes_gcmhr;
