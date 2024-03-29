use crate::utils::seeder;
use wasm_bindgen::prelude::*;

//TODO kevinly77: Full product object?
#[wasm_bindgen]
pub fn cryptocurrency_name() -> String {
	let department_str = DEPARTMENTS[seeder::gen_range(0..DEPARTMENTS_LEN)].to_string();
	let department_split = department_str.split_once("::");
	department_split.unwrap().0.to_string()
}

#[wasm_bindgen]
pub fn cryptocurrency_symbol() -> String {
	let department_str = DEPARTMENTS[seeder::gen_range(0..DEPARTMENTS_LEN)].to_string();
	let department_split = department_str.split_once("::");
	department_split.unwrap().1.to_string()
}

static DEPARTMENTS: [&'static str; 200] = [
	"Bitcoin::BTC",
	"Ethereum::ETH",
	"Tether USDt::USDT",
	"BNB::BNB",
	"Solana::SOL",
	"XRP::XRP",
	"USDC::USDC",
	"Cardano::ADA",
	"Avalanche::AVAX",
	"Dogecoin::DOGE",
	"TRON::TRX",
	"Chainlink::LINK",
	"Polkadot::DOT",
	"Polygon::MATIC",
	"Toncoin::TON",
	"Uniswap::UNI",
	"Internet Computer::ICP",
	"Shiba Inu::SHIB",
	"Dai::DAI",
	"Bitcoin Cash::BCH",
	"Litecoin::LTC",
	"Immutable::IMX",
	"NEAR Protocol::NEAR",
	"Cosmos::ATOM",
	"Filecoin::FIL",
	"Stacks::STX",
	"Ethereum Classic::ETC",
	"UNUS SED LEO::LEO",
	"Bittensor::TAO",
	"Kaspa::KAS",
	"Hedera::HBAR",
	"Aptos::APT",
	"VeChain::VET",
	"Optimism::OP",
	"Injective::INJ",
	"Stellar::XLM",
	"First Digital USD::FDUSD",
	"Lido DAO::LDO",
	"OKB::OKB",
	"Mantle::MNT",
	"Celestia::TIA",
	"Render::RNDR",
	"The Graph::GRT",
	"Cronos::CRO",
	"Arbitrum::ARB",
	"Monero::XMR",
	"Sei::SEI",
	"Sui::SUI",
	"THORChain::RUNE",
	"Maker::MKR",
	"Beam::BEAM",
	"Flare::FLR",
	"Algorand::ALGO",
	"Theta Network::THETA",
	"MultiversX::EGLD",
	"Flow::FLOW",
	"Bitcoin SV::BSV",
	"Aave::AAVE",
	"Helium::HNT",
	"Bitget Token::BGB",
	"Starknet::STRK",
	"Mina::MINA",
	"ORDI::ORDI",
	"Quant::QNT",
	"TrueUSD::TUSD",
	"Synthetix::SNX",
	"Chiliz::CHZ",
	"Fantom::FTM",
	"The Sandbox::SAND",
	"ApeCoin::APE",
	"Axie Infinity::AXS",
	"Worldcoin::WLD",
	"Tezos::XTZ",
	"Blur::BLUR",
	"BitTorrent (New)::BTT",
	"KuCoin Token::KCS",
	"Arweave::AR",
	"dYdX (ethDYDX)::ETHDYDX",
	"SATS::1000SATS",
	"Decentraland::MANA",
	"Akash Network::AKT",
	"Fetch.ai::FET",
	"Conflux::CFX",
	"WOO::WOO",
	"SingularityNET::AGIX",
	"Dymension::DYM",
	"Astar::ASTR",
	"Neo::NEO",
	"Gnosis::GNO",
	"Ronin::RON",
	"Gala::GALA",
	"EOS::EOS",
	"Pyth Network::PYTH",
	"Kava::KAVA",
	"IOTA::IOTA",
	"Oasis Network::ROSE",
	"Axelar::AXL",
	"Klaytn::KLAY",
	"Osmosis::OSMO",
	"PancakeSwap::CAKE",
	"Bonk::BONK",
	"JasmyCoin::JASMY",
	"WEMIX::WEMIX",
	"Manta Network::MANTA",
	"USDD::USDD",
	"Terra Classic::LUNC",
	"Frax Share::FXS",
	"Ethereum Name Service::ENS",
	"Jupiter::JUP",
	"Curve DAO Token::CRV",
	"Pendle::PENDLE",
	"eCash::XEC",
	"Nexo::NEXO",
	"Nervos Network::CKB",
	"Pepe::PEPE",
	"Rocket Pool::RPL",
	"ZetaChain::ZETA",
	"Compound::COMP",
	"FTX Token::FTT",
	"Ondo::ONDO",
	"IoTeX::IOTX",
	"Siacoin::SC",
	"XDC Network::XDC",
	"1inch Network::1INCH",
	"Core::CORE",
	"Trust Wallet Token::TWT",
	"Metis::METIS",
	"Altlayer::ALT",
	"SuperVerse::SUPER",
	"Celo::CELO",
	"Enjin Coin::ENJ",
	"Tether Gold::XAUt",
	"GMX::GMX",
	"GMT::GMT",
	"Radix::XRD",
	"Neutron::NTRN",
	"GateToken::GT",
	"aelf::ELF",
	"Casper::CSPR",
	"Terra::LUNA",
	"Convex Finance::CVX",
	"Zcash::ZEC",
	"SKALE::SKL",
	"Pixels::PIXEL",
	"Zilliqa::ZIL",
	"Bitcoin Gold::BTG",
	"Livepeer::LPT",
	"dogwifhat::WIF",
	"Holo::HOT",
	"APENFT::NFT",
	"Gas::GAS",
	"Mask Network::MASK",
	"OriginTrail::TRAC",
	"Illuvium::ILV",
	"Ocean Protocol::OCEAN",
	"Golem::GLM",
	"Xai::XAI",
	"Kusama::KSM",
	"COTI::COTI",
	"PAX Gold::PAXG",
	"Basic Attention Token::BAT",
	"Chia::XCH",
	"Loopring::LRC",
	"VeThor Token::VTHO",
	"Moonbeam::GLMR",
	"SafePal::SFP",
	"Dash::DASH",
	"SushiSwap::SUSHI",
	"FLOKI::FLOKI",
	"Qtum::QTUM",
	"Theta Fuel::TFUEL",
	"Decentralized Social::DESO",
	"Galxe::GAL",
	"Ravencoin::RVN",
	"NEM::XEM",
	"API3::API3",
	"Audius::AUDIO",
	"Aragon::ANT",
	"ssv.network::SSV",
	"Chromia::CHR",
	"AIOZ Network::AIOZ",
	"EthereumPoW::ETHW",
	"Kadena::KDA",
	"Treasure::MAGIC",
	"Decred::DCR",
	"Ankr::ANKR",
	"Echelon Prime::PRIME",
	"TerraClassicUSD::USTC",
	"Flux::FLUX",
	"Centrifuge::CFG",
	"UMA::UMA",
	"Memecoin::MEME",
	"Harmony::ONE",
	"0x Protocol::ZRX",
	"PayPal USD::PYUSD",
	"Ribbon Finance::RBN",
	"Helium Mobile::MOBILE",
	"JUST::JST",
	"Tellor::TRB",
	"Storj::STORJ",
];

static DEPARTMENTS_LEN: usize = DEPARTMENTS.len();