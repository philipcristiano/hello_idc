use tracing::Level;

pub fn logging(level: Level, is_json: bool) {
    let subscriber = tracing_subscriber::fmt().with_max_level(level);
    if is_json {
        subscriber.json().init()
    } else {
        subscriber.init()
    };
}
