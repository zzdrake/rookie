use futures::executor::block_on;
use paho_mqtt as mqtt;
use std::{env, process};

fn main() {
    env_logger::init();

    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    println!("Connecting to the client: {}", host);

    // 创建客户端
    let cli = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

    // async 返回一个 Result 类型，block_on 会等待其完成，如果出错则匹配 Err
    if let Err(err) = block_on(async {
        // 连接默认选项等待连接完成或者失败
        // 默认是 MQTT v3.x 连接
        cli.connect(None).await?;

        // 发布消息
        println!("Publishing a message on the topic 'test'");
        let msg = mqtt::Message::new("test", "Hello Rust MQTTT world", mqtt::QOS_1);
        cli.publish(msg).await?;

        // 从 broker 断开
        println!("Disconnecting");
        cli.disconnect(None).await?;

        // let result: Result<(), mqtt::error> = Ok::<(), mqtt::error>(())
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
}
