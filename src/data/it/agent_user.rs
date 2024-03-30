use std::collections::HashMap;
use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn user_agent(agent: Option<String>) -> String {
  let mut vendor: String = "".to_owned();
  if let Some(arg) = agent {
    vendor.replace_range(.., &arg);
  } else {
    vendor.replace_range(.., &random_vendor())
  }

  let agents = AGENT_HASHMAP.get(vendor.as_str());
  if let Some(arg) = agents {
    return arg.to_string();
  }

  "".to_owned()
}

fn random_vendor() -> String {
  VENDOR_LIST[seeder::gen_range(0..VENDOR_LIST_LEN)].to_owned()
}

static VENDOR_AOL: &str = "aol";
static AGENT_AOL: &str = "Mozilla/5.0 (compatible; MSIE 9.0; AOL 9.7; AOLBuild 4343.19; Windows NT 6.1; WOW64; Trident/5.0; FunWebProducts)";

static VENDOR_CHROME: &str = "chrome";
static AGENT_CHROME: &str = "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36";

static VENDOR_FIREFOX: &str = "firefox";
static AGENT_FIREFOX: &str = "Mozilla/5.0 (Windows NT x.y; Win64; x64; rv:10.0) Gecko/20100101 Firefox/10.0";

static VENDOR_IE: &str = "internet_explorer";
static AGENT_IE: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; AS; rv:11.0) like Gecko";

static VENDOR_NETSCAPE: &str = "netscape";
static AGENT_NETSCAPE: &str = "Mozilla/5.0 (Windows; U; Win 9x 4.90; SG; rv:1.9.2.4) Gecko/20101104 Netscape/9.1.0285";

static VENDOR_OPERA: &str = "opera";
static AGENT_OPERA: &str = "Opera/9.80 (X11; Linux i686; Ubuntu/14.10) Presto/2.12.388 Version/12.16";

static VENDOR_SAFARI: &str = "safari";
static AGENT_SAFARI: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.75.14 (KHTML, like Gecko) Version/7.0.3 Safari/7046A194A";

static VENDOR_LIST: [&'static str; 7] = [
  VENDOR_AOL,
  VENDOR_CHROME,
  VENDOR_FIREFOX,
  VENDOR_IE,
  VENDOR_NETSCAPE,
  VENDOR_OPERA,
  VENDOR_SAFARI
];
static VENDOR_LIST_LEN: usize = VENDOR_LIST.len();

lazy_static! {
  static ref AGENT_HASHMAP: HashMap<&'static str, &'static str> = {
    let mut m = HashMap::new();
    m.insert(
      VENDOR_AOL,
      AGENT_AOL
    );

    m.insert(
      VENDOR_CHROME,
      AGENT_CHROME
    );

    m.insert(
      VENDOR_FIREFOX,
      AGENT_FIREFOX
    );

    m.insert(
      VENDOR_IE,
      AGENT_IE
    );

    m.insert(
      VENDOR_NETSCAPE,
      AGENT_NETSCAPE
    );

    m.insert(
      VENDOR_OPERA,
      AGENT_OPERA
    );

    m.insert(
      VENDOR_SAFARI,
      AGENT_SAFARI
    );
    m
  };
}
