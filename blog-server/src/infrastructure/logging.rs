use tracing_subscriber::{fmt, EnvFilter};
pub fn init_tracing() {
    let filter = EnvFilter::from_default_env()
        .add_directive("actix_web=info".parse().unwrap())
        .add_directive("blog_server=debug".parse().unwrap());

    fmt().with_env_filter(filter).init();
}
