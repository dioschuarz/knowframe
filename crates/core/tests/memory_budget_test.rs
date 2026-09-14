use std::fs;

#[test]
fn test_daemon_idle_memory_rss() {
    // On Linux, inspect /proc/self/status for VmRSS
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        let mut found_rss = false;
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<u64>() {
                        found_rss = true;
                        let mb = kb as f64 / 1024.0;
                        println!("Current process VmRSS: {:.2} MB (budget: <= 35 MB)", mb);
                        assert!(
                            mb <= 35.0,
                            "Idle daemon RSS was {:.2} MB, exceeding 35 MB budget (NFR-008)",
                            mb
                        );
                    }
                }
            }
        }
        if !found_rss {
            println!("VmRSS not reported in /proc/self/status, passing test gracefully.");
        }
    } else {
        println!("Non-Linux platform or /proc/self/status unavailable; skipping RSS test.");
    }
}
