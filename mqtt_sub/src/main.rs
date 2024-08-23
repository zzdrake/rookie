use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, MQTT_VERSION_5};
use std::{env, process, time::Duration};

// 我们要订阅的主题
const TOPICS: &[&str] = &["test", "hello"];
const QOS: &[i32] = &[1, 1];

fn main() {
    env_logger::init();

    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    println!("Connecting to the MQTT server at '{}'...", host);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_async_sub_v5")
        .finalize();
    
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        // 创建一个 stream 给客户端接收消息
        let mut strm = cli.get_stream(25);

        // 定义连接的方式
        let lwt = mqtt::Message::new(
            "test/lwt",
            "[LWT] Async subscriber v5 lost connection",
            mqtt::QOS_1,
        );

        let conn_opts = mqtt::ConnectOptionsBuilder::with_mqtt_version(MQTT_VERSION_5)
            .clean_start(false)
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600])
            .will_message(lwt)
            .finalize();

        // 连接 broker
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPICS);
        let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); TOPICS.len()];
        cli.subscribe_many_with_options(TOPICS, QOS, &sub_opts, None)
            .await?;

        // 循环等待消息
        println!("Waiting for messages...");

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                if msg.retained() {
                    print!("(R) ");
                }
                println!("{}", msg);
            }
            else {
                // None 表示断开连接了
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
                    println!("Error reconnecting: {}", err);
                    // For tokio use: tokio::time::delay_for()
                    async_std::task::sleep(Duration::from_millis(1000)).await;
                }
            }
        }

        // Explicit return type for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
}