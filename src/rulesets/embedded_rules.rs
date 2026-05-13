// Generated from base/config/{main,nobase,universal}.ini active ruleset URLs.
// Do not edit manually unless you know exactly what you are changing.

pub fn is_embedded_ruleset_url(url: &str) -> bool {
    url.starts_with("embedded://")
}

pub fn get_embedded_ruleset(url: &str) -> Option<&'static str> {
    match url {
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/ai_platform.list" => Some(include_str!("../../base/rules/clash/ai_platform.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/direct.list" => Some(include_str!("../../base/rules/clash/direct.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/download.list" => Some(include_str!("../../base/rules/clash/download.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/ip_location.list" => Some(include_str!("../../base/rules/clash/ip_location.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/LanZouYun.list" => Some(include_str!("../../base/rules/clash/LanZouYun.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/larksuite.list" => Some(include_str!("../../base/rules/clash/larksuite.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/maoxiong_emby.list" => Some(include_str!("../../base/rules/clash/maoxiong_emby.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/one_click_login.list" => Some(include_str!("../../base/rules/clash/one_click_login.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/payoneer.list" => Some(include_str!("../../base/rules/clash/payoneer.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/proxy_provider_block.list" => Some(include_str!("../../base/rules/clash/proxy_provider_block.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/reject.list" => Some(include_str!("../../base/rules/clash/reject.list")),
        "embedded://raw.githubusercontent.com/Monopink/my-sub/main/base/rules/clash/tiktok_shop.list" => Some(include_str!("../../base/rules/clash/tiktok_shop.list")),
        _ => None,
    }
}
