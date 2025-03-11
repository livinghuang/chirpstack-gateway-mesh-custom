#[macro_use]
extern crate anyhow;

use chirpstack_api::gw;
use chirpstack_api::prost::Message;
use zeromq::{SocketRecv, SocketSend};

use chirpstack_gateway_mesh::aes128::Aes128Key;
use chirpstack_gateway_mesh::packets;
mod common;

/*
    This tests the scenario when the Border Gateway receives a downlink which must
    be mesh encapsulated and forwarded to the Relay Gateway.
*/
#[tokio::test]
async fn test_border_gateway_downlink_mesh() {
    common::setup(true).await;

    // ✅ 1. 等待 Mesh Network 回傳 Heartbeat
    let mut down: gw::DownlinkFrame = {
        let mut cmd_sock = common::MESH_BACKEND_COMMAND_SOCK.get().unwrap().lock().await;
        let msg = cmd_sock.recv().await.expect("❌ Failed to receive message from MESH_BACKEND_COMMAND_SOCK");

        let cmd = String::from_utf8(msg.get(0).map(|v| v.to_vec()).unwrap()).unwrap();
        assert_eq!("down", cmd);

        gw::DownlinkFrame::decode(msg.get(1).cloned().unwrap()).expect("❌ Failed to decode DownlinkFrame")
    };

    // ✅ 2. 解析 Heartbeat 並確認收到
    let heartbeat_down_item = down.items.first().expect("❌ No items in received DownlinkFrame!");
    let heartbeat_mesh_packet = packets::MeshPacket::from_slice(&heartbeat_down_item.phy_payload)
        .expect("❌ Failed to parse MeshPacket!");

    println!("📥 Received down_item: {:?}", heartbeat_down_item);
    println!("📦 Parsed mesh_packet: {:?}", heartbeat_mesh_packet);

    // ✅ 3. 確認收到 Heartbeat 才繼續
    match heartbeat_mesh_packet.payload {
        packets::Payload::Heartbeat(_) => {
            println!("✅ We successfully got Heartbeat! Now starting the downlink test...");
        },
        _ => {
            panic!("❌ Expected a Heartbeat packet but received something else!");
        }
    }

    // ✅ 4. 發送 Downlink 測試封包
    let mut down = gw::DownlinkFrame {
        downlink_id: 1,
        gateway_id: "0101010101010101".into(),
        items: vec![gw::DownlinkFrameItem {
            phy_payload: vec![9, 8, 7, 6],
            tx_info: Some(gw::DownlinkTxInfo {
                frequency: 868500000,
                power: 16,
                modulation: Some(gw::Modulation {
                    parameters: Some(gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                        bandwidth: 125000,
                        spreading_factor: 12,
                        code_rate: gw::CodeRate::Cr45.into(),
                        polarization_inversion: true,
                        ..Default::default()
                    })),
                }),
                timing: Some(gw::Timing {
                    parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                        delay: Some(prost_types::Duration {
                            seconds: 3,
                            ..Default::default()
                        }),
                    })),
                }),
                context: vec![1, 2, 3, 1, 2, 3, 4, 0, 123],
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    };

    // ✅ 5. 發送 Downlink 測試命令
    {
        let mut cmd_sock = common::FORWARDER_COMMAND_SOCK.get().unwrap().lock().await;
        cmd_sock
            .send(
                vec![
                    bytes::Bytes::from("down"),
                    bytes::Bytes::from(down.encode_to_vec()),
                ]
                .try_into()
                .unwrap(),
            )
            .await
            .expect("❌ Failed to send downlink test command");
    }
    println!("✅ Downlink test started!");

    // ✅ 6. 接收 Downlink 回應
    let down: gw::DownlinkFrame = {
        let mut cmd_sock = common::MESH_BACKEND_COMMAND_SOCK.get().unwrap().lock().await;
        let msg = cmd_sock.recv().await.expect("❌ Failed to receive downlink response");

        let cmd = String::from_utf8(msg.get(0).map(|v| v.to_vec()).unwrap()).unwrap();
        assert_eq!("down", cmd);

        gw::DownlinkFrame::decode(msg.get(1).cloned().unwrap()).expect("❌ Failed to decode DownlinkFrame")
    };

    // ✅ 7. 解析並打印 Downlink 回應
    let down_item = down.items.first().expect("❌ Downlink response is empty!");
    let mesh_packet = packets::MeshPacket::from_slice(&down_item.phy_payload)
        .expect("❌ Failed to parse MeshPacket!");
    println!("📥 Received downlink response: {:?}", down_item);
    println!("📦 Parsed downlink mesh_packet: {:?}", mesh_packet);
    std::process::exit(0);
assert_eq!(
        {
            let mut packet = packets::MeshPacket {
                mhdr: packets::MHDR {
                    payload_type: packets::PayloadType::Downlink,
                    hop_count: 1,
                },
                payload: packets::Payload::Downlink(packets::DownlinkPayload {
                    metadata: packets::DownlinkMetadata {
                        uplink_id: 123,
                        dr: 0,
                        frequency: 868500000,
                        tx_power: 1,
                        delay: 3,
                    },
                    relay_id: [1, 2, 3, 4],
                    phy_payload: vec![9, 8, 7, 6],
                }),
                mic: None,
            };
            packet.set_mic(Aes128Key::null()).unwrap();
            packet
        },
        mesh_packet
    );

    assert_eq!(
        &gw::DownlinkTxInfo {
            frequency: 868100000,
            power: 16,
            modulation: Some(gw::Modulation {
                parameters: Some(gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                    bandwidth: 125000,
                    spreading_factor: 7,
                    code_rate: gw::CodeRate::Cr45.into(),
                    ..Default::default()
                }))
            }),
            timing: Some(gw::Timing {
                parameters: Some(gw::timing::Parameters::Immediately(
                    gw::ImmediatelyTimingInfo {}
                )),
            }),
            ..Default::default()
        },
        down_item.tx_info.as_ref().unwrap()
    );
}
