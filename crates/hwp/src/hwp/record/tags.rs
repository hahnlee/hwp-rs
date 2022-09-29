const HWPTAG_BEGIN: u32 = 0x10;

#[repr(u32)]
#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DocInfoRecord {
    HWPTAG_DOCUMENT_PROPERTIES = HWPTAG_BEGIN,
    HWPTAG_ID_MAPPINGS,
    HWPTAG_BIN_DATA,
}
