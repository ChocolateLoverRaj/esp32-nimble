use std::sync::mpsc::channel;

use esp32_nimble::{enums::*, utilities::BleUuid, BLEDevice, BLEReturnCode, NimbleProperties};
use esp_idf_sys as _;

fn main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  let device = BLEDevice::take();
  device
    .security()
    .set_auth(AuthReq::all())
    .set_passkey(123456)
    .set_io_cap(SecurityIOCap::DisplayOnly);

  let server = device.get_server();

  let (advertise_tx, advertise_rx) = channel::<()>();

  println!("HI");
  server.on_connect(move |server, desc| {
    ::log::info!("Client connected: {:?}", desc);
    server
      .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
      .unwrap();

    if server.connected_count() < (esp_idf_sys::CONFIG_BT_NIMBLE_MAX_CONNECTIONS as _) {
      ::log::info!("Multi-connect support: start advertising");
      advertise_tx.send(()).unwrap();
    }
  });
  server.on_disconnect(|_desc, reason| {
    ::log::info!("Client disconnected ({:?})", BLEReturnCode(reason as _));
  });

  let service = server.create_service(BleUuid::Uuid16(0xABCD));

  let non_secure_characteristic = service
    .lock()
    .create_characteristic(BleUuid::Uuid16(0x1234), NimbleProperties::READ);
  non_secure_characteristic
    .lock()
    .set_value("non_secure_characteristic".as_bytes());

  let secure_characteristic = service.lock().create_characteristic(
    BleUuid::Uuid16(0x1235),
    NimbleProperties::READ | NimbleProperties::READ_ENC | NimbleProperties::READ_AUTHEN,
  );
  secure_characteristic
    .lock()
    .set_value("secure_characteristic".as_bytes());

  let ble_advertising = device.get_advertising();
  ble_advertising
    .name("ESP32-GATT-Server")
    .add_service_uuid(BleUuid::Uuid16(0xABCD))
    .start()
    .unwrap();

  ::log::info!("bonded_addresses: {:?}", device.bonded_addresses().unwrap());

  loop {
    advertise_rx.recv().unwrap();
    device.get_advertising().start().unwrap();
  }
}
