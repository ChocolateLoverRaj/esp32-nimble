use bitflags::bitflags;
use esp_idf_sys::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SecurityIOCap {
  /// DisplayOnly IO capability
  DisplayOnly = BLE_HS_IO_DISPLAY_ONLY as _,
  /// DisplayYesNo IO capability
  DisplayYesNo = BLE_HS_IO_DISPLAY_YESNO as _,
  /// KeyboardOnly IO capability
  KeyboardOnly = BLE_HS_IO_KEYBOARD_ONLY as _,
  /// NoInputNoOutput IO capability
  NoInputNoOutput = BLE_HS_IO_NO_INPUT_OUTPUT as _,
  /// KeyboardDisplay Only IO capability
  KeyboardDisplay = BLE_HS_IO_KEYBOARD_DISPLAY as _,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PowerLevel {
  /// Corresponding to -12dbm
  N12 = esp_power_level_t_ESP_PWR_LVL_N12 as _,
  /// Corresponding to  -9dbm
  N9 = esp_power_level_t_ESP_PWR_LVL_N9 as _,
  /// Corresponding to  -6dbm
  N6 = esp_power_level_t_ESP_PWR_LVL_N6 as _,
  /// Corresponding to  -3dbm
  N3 = esp_power_level_t_ESP_PWR_LVL_N3 as _,
  /// Corresponding to   0dbm
  N0 = esp_power_level_t_ESP_PWR_LVL_N0 as _,
  /// Corresponding to  +3dbm
  P3 = esp_power_level_t_ESP_PWR_LVL_P3 as _,
  /// Corresponding to  +6dbm
  P6 = esp_power_level_t_ESP_PWR_LVL_P6 as _,
  /// Corresponding to  +9dbm
  P9 = esp_power_level_t_ESP_PWR_LVL_P9 as _,
}

impl PowerLevel {
  pub fn to_dbm(&self) -> i8 {
    match self {
      PowerLevel::N12 => -12,
      PowerLevel::N9 => -9,
      PowerLevel::N6 => -6,
      PowerLevel::N3 => -3,
      PowerLevel::N0 => 0,
      PowerLevel::P3 => 3,
      PowerLevel::P6 => 6,
      PowerLevel::P9 => 9,
    }
  }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PowerType {
  /// For connection handle 0
  ConnHdl0 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL0 as _,
  /// For connection handle 1
  ConnHdl1 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL1 as _,
  /// For connection handle 2
  ConnHdl2 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL2 as _,
  /// For connection handle 3
  ConnHdl3 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL3 as _,
  /// For connection handle 4
  ConnHdl4 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL4 as _,
  /// For connection handle 5
  ConnHdl5 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL5 as _,
  /// For connection handle 6
  ConnHdl6 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL6 as _,
  /// For connection handle 7
  ConnHdl7 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL7 as _,
  /// For connection handle 8
  ConnHdl8 = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL8 as _,
  /// For advertising
  Advertising = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV as _,
  /// For scan
  Scan = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN as _,
  /// For default, if not set other, it will use default value
  Default = esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT as _,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OwnAddrType {
  Public = esp_idf_sys::BLE_OWN_ADDR_PUBLIC as _,
  Random = esp_idf_sys::BLE_OWN_ADDR_RANDOM as _,
  RpaPublicDefault = esp_idf_sys::BLE_OWN_ADDR_RPA_PUBLIC_DEFAULT as _,
  RpaRandomDefault = esp_idf_sys::BLE_OWN_ADDR_RPA_RANDOM_DEFAULT as _,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ConnMode {
  /// non-connectable (3.C.9.3.2)
  Non = esp_idf_sys::BLE_GAP_CONN_MODE_NON as _,
  /// directed-connectable (3.C.9.3.3)
  Dir = esp_idf_sys::BLE_GAP_CONN_MODE_DIR as _,
  /// undirected-connectable (3.C.9.3.4)
  Und = esp_idf_sys::BLE_GAP_CONN_MODE_UND as _,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DiscMode {
  /// non-discoverable; 3.C.9.2.2
  Non = BLE_GAP_DISC_MODE_NON as _,
  /// limited-discoverable; 3.C.9.2.3
  Ltd = BLE_GAP_DISC_MODE_LTD as _,
  /// general-discoverable; 3.C.9.2.4
  Gen = BLE_GAP_DISC_MODE_GEN as _,
}

bitflags! {
  #[repr(transparent)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct PairKeyDist: u8 {
    /// Accept/Distribute the encryption key.
    const ENC = BLE_SM_PAIR_KEY_DIST_ENC as _;
    /// Accept/Distribute the ID key (IRK).
    const ID = BLE_SM_PAIR_KEY_DIST_ID as _;
    const SIGN = BLE_SM_PAIR_KEY_DIST_SIGN as _;
    const LINK = BLE_SM_PAIR_KEY_DIST_LINK as _;
  }
}

bitflags! {
  #[repr(transparent)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct AuthReq: u8 {
    /// allow bounding
    const Bond = 0b001;
    /// man in the middle protection
    const Mitm = 0b010;
    /// secure connection pairing
    const Sc = 0b100;
  }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive)]
pub enum AdvType {
  /// indirect advertising
  Ind = esp_idf_sys::BLE_HCI_ADV_TYPE_ADV_IND as _,
  /// direct advertising
  DirectInd = esp_idf_sys::BLE_HCI_ADV_TYPE_ADV_DIRECT_IND_HD as _,
  /// indirect scan response
  ScanInd = esp_idf_sys::BLE_HCI_ADV_TYPE_ADV_SCAN_IND as _,
  /// indirect advertising - not connectable
  NonconnInd = esp_idf_sys::BLE_HCI_ADV_TYPE_ADV_NONCONN_IND as _,
  // DirectIndLd = esp_idf_sys::BLE_HCI_ADV_TYPE_ADV_DIRECT_IND_LD as _,
}

bitflags! {
  #[repr(transparent)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct AdvFlag: u8 {
    /// LE Limited Discoverable Mode
    const DiscLimited = esp_idf_sys::BLE_HS_ADV_F_DISC_LTD as _;
    /// LE General Discoverable Mode
    const DiscGeneral = esp_idf_sys::BLE_HS_ADV_F_DISC_GEN as _;
    /// BR/EDR Not Supported
    const BrEdrUnsupported = esp_idf_sys::BLE_HS_ADV_F_BREDR_UNSUP as _;
    /// Simultaneous LE and BR/EDR to Same Device Capable (Controller)
    const SimultaneousController = 0b01000;
    /// Simultaneous LE and BR/EDR to Same Device Capable (Host)
    const SimultaneousHost       = 0b10000;
  }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, IntoPrimitive)]
pub enum ScanFilterPolicy {
  /// Scanner processes all advertising packets (white list not used)
  /// except directed, connectable advertising packets not sent to the scanner.
  NoWl = BLE_HCI_SCAN_FILT_NO_WL as _,
  /// Scanner processes advertisements from white list only.
  /// A connectable, directed advertisement is ignored unless it contains scanners address.
  UseWl = BLE_HCI_SCAN_FILT_USE_WL as _,
  /// Scanner process all advertising packets (white list not used).
  /// A connectable, directed advertisement shall not be ignored if the InitA is a resolvable private address.
  NoWlInitA = BLE_HCI_SCAN_FILT_NO_WL_INITA as _,
  /// Scanner process advertisements from white list only.
  /// A connectable, directed advertisement shall not be ignored if the InitA is a resolvable private address.
  UseWlInitA = BLE_HCI_SCAN_FILT_USE_WL_INITA as _,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, IntoPrimitive)]
pub enum AdvFilterPolicy {
  /// No filtering
  None = BLE_HCI_ADV_FILT_NONE as _,
  /// only allow scan requests from those on the white list.
  Scan = BLE_HCI_ADV_FILT_SCAN as _,
  /// only allow connections from those on the white list.
  Connect = BLE_HCI_ADV_FILT_CONN as _,
  /// only allow scan/connections from those on the white list.
  Both = BLE_HCI_ADV_FILT_BOTH as _,
}
