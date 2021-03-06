use eth_watcher::config;
use eth_watcher::eth_client::EthereumGateway;
use eth_watcher::eth_watch::{EthHttpClient, EthWatch, EthWatchRequest};
use fluidex_common::non_blocking_tracing;
use futures::{channel::mpsc, SinkExt};
use tokio::{runtime::Runtime, time};

fn main() {
    let mut main_runtime = Runtime::new().expect("main runtime start");

    dotenv::dotenv().ok();
    let _guard = non_blocking_tracing::setup();
    log::info!("ETH watcher started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("CONFIG_FILE").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: config::Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    let client = EthereumGateway::from_config(&settings);

    let (eth_req_sender, eth_req_receiver) = mpsc::channel(256);

    let eth_client = EthHttpClient::new(client, settings.contracts.contract_addr);
    let watcher = EthWatch::new(eth_client, settings.eth_watch.confirmations_for_eth_event);

    main_runtime.spawn(watcher.run(eth_req_receiver));
    let poll_interval = settings.eth_watch.poll_interval();
    main_runtime.block_on(async move {
        let mut timer = time::interval(poll_interval);

        loop {
            timer.tick().await;
            eth_req_sender
                .clone()
                .send(EthWatchRequest::PollETHNode)
                .await
                .expect("ETH watch receiver dropped");
        }
    });
}
