use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    let level = |_| "acme_web=info,tower_http=info".into();
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(level);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
