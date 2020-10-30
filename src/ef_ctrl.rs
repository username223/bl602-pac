#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - ef_if_ctrl_0."]
    pub ef_if_ctrl_0: crate::Reg<ef_if_ctrl_0::EF_IF_CTRL_0_SPEC>,
    #[doc = "0x804 - ef_if_cyc_0."]
    pub ef_if_cyc_0: crate::Reg<ef_if_cyc_0::EF_IF_CYC_0_SPEC>,
    #[doc = "0x808 - ef_if_cyc_1."]
    pub ef_if_cyc_1: crate::Reg<ef_if_cyc_1::EF_IF_CYC_1_SPEC>,
    #[doc = "0x80c - ef_if_0_manual."]
    pub ef_if_0_manual: crate::Reg<ef_if_0_manual::EF_IF_0_MANUAL_SPEC>,
    #[doc = "0x810 - ef_if_0_status."]
    pub ef_if_0_status: crate::Reg<ef_if_0_status::EF_IF_0_STATUS_SPEC>,
    #[doc = "0x814 - ef_if_cfg_0."]
    pub ef_if_cfg_0: crate::Reg<ef_if_cfg_0::EF_IF_CFG_0_SPEC>,
    #[doc = "0x818 - ef_sw_cfg_0."]
    pub ef_sw_cfg_0: crate::Reg<ef_sw_cfg_0::EF_SW_CFG_0_SPEC>,
    #[doc = "0x81c - ef_reserved."]
    pub ef_reserved: crate::Reg<ef_reserved::EF_RESERVED_SPEC>,
    #[doc = "0x820 - ef_if_ana_trim_0."]
    pub ef_if_ana_trim_0: crate::Reg<ef_if_ana_trim_0::EF_IF_ANA_TRIM_0_SPEC>,
    #[doc = "0x824 - ef_if_sw_usage_0."]
    pub ef_if_sw_usage_0: crate::Reg<ef_if_sw_usage_0::EF_IF_SW_USAGE_0_SPEC>,
    _reserved10: [u8; 472usize],
    #[doc = "0xa00 - ef_crc_ctrl_0."]
    pub ef_crc_ctrl_0: crate::Reg<ef_crc_ctrl_0::EF_CRC_CTRL_0_SPEC>,
    #[doc = "0xa04 - ef_crc_ctrl_1."]
    pub ef_crc_ctrl_1: crate::Reg<ef_crc_ctrl_1::EF_CRC_CTRL_1_SPEC>,
    #[doc = "0xa08 - ef_crc_ctrl_2."]
    pub ef_crc_ctrl_2: crate::Reg<ef_crc_ctrl_2::EF_CRC_CTRL_2_SPEC>,
    #[doc = "0xa0c - ef_crc_ctrl_3."]
    pub ef_crc_ctrl_3: crate::Reg<ef_crc_ctrl_3::EF_CRC_CTRL_3_SPEC>,
    #[doc = "0xa10 - ef_crc_ctrl_4."]
    pub ef_crc_ctrl_4: crate::Reg<ef_crc_ctrl_4::EF_CRC_CTRL_4_SPEC>,
    #[doc = "0xa14 - ef_crc_ctrl_5."]
    pub ef_crc_ctrl_5: crate::Reg<ef_crc_ctrl_5::EF_CRC_CTRL_5_SPEC>,
}
#[doc = "ef_if_ctrl_0 register accessor: an alias for `Reg<EF_IF_CTRL_0_SPEC>`"]
pub type EF_IF_CTRL_0 = crate::Reg<ef_if_ctrl_0::EF_IF_CTRL_0_SPEC>;
#[doc = "ef_if_ctrl_0."]
pub mod ef_if_ctrl_0;
#[doc = "ef_if_cyc_0 register accessor: an alias for `Reg<EF_IF_CYC_0_SPEC>`"]
pub type EF_IF_CYC_0 = crate::Reg<ef_if_cyc_0::EF_IF_CYC_0_SPEC>;
#[doc = "ef_if_cyc_0."]
pub mod ef_if_cyc_0;
#[doc = "ef_if_cyc_1 register accessor: an alias for `Reg<EF_IF_CYC_1_SPEC>`"]
pub type EF_IF_CYC_1 = crate::Reg<ef_if_cyc_1::EF_IF_CYC_1_SPEC>;
#[doc = "ef_if_cyc_1."]
pub mod ef_if_cyc_1;
#[doc = "ef_if_0_manual register accessor: an alias for `Reg<EF_IF_0_MANUAL_SPEC>`"]
pub type EF_IF_0_MANUAL = crate::Reg<ef_if_0_manual::EF_IF_0_MANUAL_SPEC>;
#[doc = "ef_if_0_manual."]
pub mod ef_if_0_manual;
#[doc = "ef_if_0_status register accessor: an alias for `Reg<EF_IF_0_STATUS_SPEC>`"]
pub type EF_IF_0_STATUS = crate::Reg<ef_if_0_status::EF_IF_0_STATUS_SPEC>;
#[doc = "ef_if_0_status."]
pub mod ef_if_0_status;
#[doc = "ef_if_cfg_0 register accessor: an alias for `Reg<EF_IF_CFG_0_SPEC>`"]
pub type EF_IF_CFG_0 = crate::Reg<ef_if_cfg_0::EF_IF_CFG_0_SPEC>;
#[doc = "ef_if_cfg_0."]
pub mod ef_if_cfg_0;
#[doc = "ef_sw_cfg_0 register accessor: an alias for `Reg<EF_SW_CFG_0_SPEC>`"]
pub type EF_SW_CFG_0 = crate::Reg<ef_sw_cfg_0::EF_SW_CFG_0_SPEC>;
#[doc = "ef_sw_cfg_0."]
pub mod ef_sw_cfg_0;
#[doc = "ef_reserved register accessor: an alias for `Reg<EF_RESERVED_SPEC>`"]
pub type EF_RESERVED = crate::Reg<ef_reserved::EF_RESERVED_SPEC>;
#[doc = "ef_reserved."]
pub mod ef_reserved;
#[doc = "ef_if_ana_trim_0 register accessor: an alias for `Reg<EF_IF_ANA_TRIM_0_SPEC>`"]
pub type EF_IF_ANA_TRIM_0 = crate::Reg<ef_if_ana_trim_0::EF_IF_ANA_TRIM_0_SPEC>;
#[doc = "ef_if_ana_trim_0."]
pub mod ef_if_ana_trim_0;
#[doc = "ef_if_sw_usage_0 register accessor: an alias for `Reg<EF_IF_SW_USAGE_0_SPEC>`"]
pub type EF_IF_SW_USAGE_0 = crate::Reg<ef_if_sw_usage_0::EF_IF_SW_USAGE_0_SPEC>;
#[doc = "ef_if_sw_usage_0."]
pub mod ef_if_sw_usage_0;
#[doc = "ef_crc_ctrl_0 register accessor: an alias for `Reg<EF_CRC_CTRL_0_SPEC>`"]
pub type EF_CRC_CTRL_0 = crate::Reg<ef_crc_ctrl_0::EF_CRC_CTRL_0_SPEC>;
#[doc = "ef_crc_ctrl_0."]
pub mod ef_crc_ctrl_0;
#[doc = "ef_crc_ctrl_1 register accessor: an alias for `Reg<EF_CRC_CTRL_1_SPEC>`"]
pub type EF_CRC_CTRL_1 = crate::Reg<ef_crc_ctrl_1::EF_CRC_CTRL_1_SPEC>;
#[doc = "ef_crc_ctrl_1."]
pub mod ef_crc_ctrl_1;
#[doc = "ef_crc_ctrl_2 register accessor: an alias for `Reg<EF_CRC_CTRL_2_SPEC>`"]
pub type EF_CRC_CTRL_2 = crate::Reg<ef_crc_ctrl_2::EF_CRC_CTRL_2_SPEC>;
#[doc = "ef_crc_ctrl_2."]
pub mod ef_crc_ctrl_2;
#[doc = "ef_crc_ctrl_3 register accessor: an alias for `Reg<EF_CRC_CTRL_3_SPEC>`"]
pub type EF_CRC_CTRL_3 = crate::Reg<ef_crc_ctrl_3::EF_CRC_CTRL_3_SPEC>;
#[doc = "ef_crc_ctrl_3."]
pub mod ef_crc_ctrl_3;
#[doc = "ef_crc_ctrl_4 register accessor: an alias for `Reg<EF_CRC_CTRL_4_SPEC>`"]
pub type EF_CRC_CTRL_4 = crate::Reg<ef_crc_ctrl_4::EF_CRC_CTRL_4_SPEC>;
#[doc = "ef_crc_ctrl_4."]
pub mod ef_crc_ctrl_4;
#[doc = "ef_crc_ctrl_5 register accessor: an alias for `Reg<EF_CRC_CTRL_5_SPEC>`"]
pub type EF_CRC_CTRL_5 = crate::Reg<ef_crc_ctrl_5::EF_CRC_CTRL_5_SPEC>;
#[doc = "ef_crc_ctrl_5."]
pub mod ef_crc_ctrl_5;
