#[macro_export]
macro_rules! cron_tasks {
    ($ctx:expr, $($t:path),* $(,)?) => {
        $({
            const TASK_NAME: &str = const_str::convert_ascii_case!(upper_camel, stringify!($t));
            let now = std::time::Instant::now();
            info!("Starting Task {TASK_NAME}");
            match ($t)($ctx).await {
                Ok(v) => info!("Completed Task {TASK_NAME} in {:?} with result {v:?}", now.elapsed()),
                Err(e) => error!("Error occurred in {TASK_NAME}: {e:?}"),
            }
        })*
    };
}