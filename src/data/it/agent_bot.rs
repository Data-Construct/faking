use std::collections::HashMap;
use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bot_agent(agent: Option<String>) -> String {
  let mut vendor: String = "".to_owned();
  if let Some(arg) = agent {
    vendor.replace_range(.., &arg);
  } else {
    vendor.replace_range(.., &random_vendor())
  }

  let agents = AGENT_HASHMAP.get(vendor.as_str());
  if let Some(arg) = agents {
    let agent = arg[seeder::gen_range(0..arg.len())].to_owned();
    return agent;
  }

  "".to_owned()
}

fn random_vendor() -> String {
  VENDOR_LIST[seeder::gen_range(0..VENDOR_LIST_LEN)].to_owned()
}

static AGENT_GOOGLEBOT: &str = "googlebot";
static AGENTS_GOOGLEBOT: [&'static str; 5] = [
  "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/83.0.4103.122 Safari/537.36",
  "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/99.0.4844.84 Safari/537.36",
  "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
  "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/87.0.4280.90 Safari/537.36",
  "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Safari/537.36 Googlebot-Image/1.0"
];

static AGENT_BINGBOT: &str = "bingbot";
static AGENTS_BINGBOT: [&'static str; 5] = [
  "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm) Chrome/86.0.4240.68 Safari/537.36 Edg/86.0.622.31",
  "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534 +(KHTML, like Gecko) BingPreview/1.0b",
  "Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; rv:11.0; BingPreview/1.0b) like Gecko",
  "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm) Chrome/98.0.4758.102 Safari/537.36",
  "Mozilla/5.0 (iPhone; CPU iPhone OS 7_0 like Mac OS X) AppleWebKit/537.51.1 (KHTML, like Gecko) Version/7.0 Mobile/11A465 Safari/9537.53 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)"
];

static AGENT_DUCKDUCKBOT: &str = "duckduckbot";
static AGENTS_DUCKDUCKBOT: [&'static str; 5] = [
  "DuckDuckBot-Https/1.1; (+https://duckduckgo.com/duckduckbot)",
  "Mozilla/5.0 (compatible; DuckDuckBot-Https/1.1; https://duckduckgo.com/duckduckbot)",
  "DuckDuckBot/1.1; (+http://duckduckgo.com/duckduckbot.html)",
  "DuckDuckBot-Https/1.1; (+https://duckduckgo.com/duckduckbot)",
  "Mozilla/5.0 (compatible; DuckDuckBot-Https/1.1; https://duckduckgo.com/duckduckbot)"
];

static AGENT_BAIDUSPIDER: &str = "baiduspider";
static AGENTS_BAIDUSPIDER: [&'static str; 3] = [
  "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1 (compatible; Baiduspider-render/2.0 ; +http://www.baidu.com/search/spider.html)",
  "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1 (compatible; Baiduspider-render/2.0 ; Smartapp; +http://www.baidu.com/search/spider.html)",
  "Mozilla/5.0 (compatible; Baiduspider-render/2.0 ; +http://www.baidu.com/search/spider.html)"
];

static AGENT_YANDEXBOT: &str = "yandexbot";
static AGENTS_YANDEXBOT: [&'static str; 5] = [
  "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)",
  "Mozilla/5.0 (compatible; YandexDirect/3.0; +http://yandex.com/bots)",
  "Mozilla/5.0 (compatible; YandexMetrika/2.0; +http://yandex.com/bots yabs01)",
  "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.268",
  "Mozilla/5.0 (compatible; YandexImages/3.0; +http://yandex.com/bots)"
];

static VENDOR_LIST: [&'static str; 5] = [
  AGENT_GOOGLEBOT,
  AGENT_BINGBOT,
  AGENT_DUCKDUCKBOT,
  AGENT_BAIDUSPIDER,
  AGENT_YANDEXBOT
];
static VENDOR_LIST_LEN: usize = VENDOR_LIST.len();

lazy_static! {
  static ref AGENT_HASHMAP: HashMap<&'static str, Vec<&'static str>> = {
    let mut m = HashMap::new();
    m.insert(
      AGENT_GOOGLEBOT,
      AGENTS_GOOGLEBOT.to_vec()
    );

    m.insert(
      AGENT_BINGBOT,
      AGENTS_BINGBOT.to_vec()
    );

    m.insert(
      AGENT_DUCKDUCKBOT,
      AGENTS_DUCKDUCKBOT.to_vec()
    );

    m.insert(
      AGENT_BAIDUSPIDER,
      AGENTS_BAIDUSPIDER.to_vec()
    );

    m.insert(
      AGENT_YANDEXBOT,
      AGENTS_YANDEXBOT.to_vec()
    );
    m
  };
}