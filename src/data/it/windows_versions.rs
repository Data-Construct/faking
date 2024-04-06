use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn windows_version() -> String {
	WINDOWS_VERSIONS[seeder::gen_range(0..WINDOWS_VERSIONS_LEN)].to_string()
}

#[wasm_bindgen(js_name = windows_client_version)]
pub fn client_version() -> String {
	CLIENT_VERSIONS[seeder::gen_range(0..CLIENT_VERSIONS_LEN)].to_string()
}

#[wasm_bindgen(js_name = windows_server_version)]
pub fn server_version() -> String {
	SERVER_VERSIONS[seeder::gen_range(0..SERVER_VERSIONS_LEN)].to_string()
}

#[wasm_bindgen(js_name = windows_specialized_version)]
pub fn specialized_version() -> String {
	SPECIALIZED_VERSIONS[seeder::gen_range(0..SPECIALIZED_VERSIONS_LEN)].to_string()
}

#[wasm_bindgen(js_name = windows_device_version)]
pub fn device_version() -> String {
	DEVICE_VERSIONS[seeder::gen_range(0..DEVICE_VERSIONS_LEN)].to_string()
}
static WINDOWS_VERSIONS: [&'static str; 88] = [
	"Windows 1.0",
	"Windows 2.x",
	"Windows 3.0",
	"Windows 3.1x",
	"Windows NT 3.1",
	"Windows NT 3.5x",
	"Windows NT 4.0",
	"Windows NT 4.0 Terminal Server Edition",
	"Windows 95",
	"Windows 98",
	"Windows CE 1.0",
	"Windows CE 2.0",
	"Windows CE 3.0",
	"Windows CE 4.0",
	"Windows CE 4.1",
	"Windows CE 5.0",
	"Windows CE 6.0",
	"Windows Mobile 6.0",
	"Windows Mobile 6.1",
	"Windows Mobile 6.5",
	"Windows Mobile 2003",
	"Windows XP",
	"Windows XP 64-Bit Edition",
	"Windows XP Embedded",
	"Windows XP Media Center Edition",
	"Windows XP Professional x64 Edition",
	"Windows XP Starter Edition",
	"Windows XP Tablet PC Edition",
	"Windows Vista",
	"Windows 7",
	"Windows 8",
	"Windows 8.1",
	"Windows 10",
	"Windows 10 Anniversary Update",
	"Windows 10 April 2018 Update",
	"Windows 10 Creators Update",
	"Windows 10 Fall Creators Update",
	"Windows 10 IoT Core",
	"Windows 10 May 2019 Update",
	"Windows 10 May 2020 Update",
	"Windows 10 May 2021 Update",
	"Windows 10 Mobile",
	"Windows 10 November 2019 Update",
	"Windows 10 November 2021 Update",
	"Windows 10 October 2018 Update",
	"Windows 10 October 2020 Update",
	"Windows 10 original release",
	"Windows 10X",
	"Windows 11",
	"Windows Server 2003",
	"Windows Server 2008",
	"Windows Server 2008 R2",
	"Windows Server 2012",
	"Windows Server 2012 R2",
	"Windows Server 2016",
	"Windows Server 2019",
	"Windows Server 2022",
	"Windows Server version 1709",
	"Windows Server version 1803",
	"Windows Server version 1903",
	"Windows Server version 1909",
	"Windows Server version 2004",
	"Windows Server version 20H2",
	"Windows Small Business Server 2000",
	"Windows Small Business Server 2003",
	"Windows Small Business Server 2008",
	"Windows Small Business Server 2011",
	"Windows Embedded 7",
	"Windows Embedded 8.1",
	"Windows Embedded 8",
	"Windows Embedded Compact 2013",
	"Windows Embedded Compact 7",
	"Windows Embedded for Point of Service",
	"Windows Embedded POSReady 2009",
	"Windows Embedded Standard 2009",
	"Windows Essential Business Server 2008",
	"Windows Fundamentals for Legacy PCs",
	"Windows Home Server",
	"Windows Home Server 2011",
	"Windows MultiPoint Server 2010",
	"Windows MultiPoint Server 2011",
	"Windows MultiPoint Server 2012",
	"Windows Thin PC",
	"Windows Polaris",
	"Windows Nashville",
	"Windows Neptune",
	"Windows Me",
	"Windows 2000",
];
static WINDOWS_VERSIONS_LEN: usize = WINDOWS_VERSIONS.len();

static CLIENT_VERSIONS: [&'static str; 22] = [
	"Windows 1.0",
	"Windows 2.0",
	"Windows 2.1",
	"Windows 3.0",
	"Windows 3.1",
	"Windows 3.2",
	"Windows NT 3.1",
	"Windows NT 3.5",
	"Windows NT 3.51",
	"Windows 95",
	"Windows NT 4.0",
	"Windows 98",
	"Windows Me",
	"Windows 2000",
	"Windows XP",
	"Windows XP Professional x64 Edition",
	"Windows Vista",
	"Windows 7",
	"Windows 8",
	"Windows 8.1",
	"Windows 10",
	"Windows 11",
];
static CLIENT_VERSIONS_LEN: usize = CLIENT_VERSIONS.len();

static SERVER_VERSIONS: [&'static str; 14] = [
	"Windows NT 3.1",
	"Windows NT 3.5",
	"Windows NT 3.51",
	"Windows 2000 Server",
	"Windows NT 4.0",
	"Windows Server 2003",
	"Windows Server 2003 R2",
	"Windows Server 2008",
	"Windows Server 2008 R2",
	"Windows Server 2012",
	"Windows Server 2012 R2",
	"Windows Server 2016",
	"Windows Server 2019",
	"Windows Server 2022",
];
static SERVER_VERSIONS_LEN: usize = SERVER_VERSIONS.len();

static SPECIALIZED_VERSIONS: [&'static str; 3] = [
	"Windows Embedded",
	"Windows PE",
	"Windows Fundamentals for Legacy PCs",
];
static SPECIALIZED_VERSIONS_LEN: usize = SPECIALIZED_VERSIONS.len();

static DEVICE_VERSIONS: [&'static str; 16] = [
	"Windows RT 8.1",
	"Windows RT 8",
	"Windows 7 Starter",
	"Windows Vista Starter",
	"Windows XP Media Center Edition",
	"Windows Embedded Industry",
	"Windows NT 4.0 Embedded",
	"Windows XP Embedded",
	"Windows Embedded for Point of Service",
	"Windows Embedded Standard 2009",
	"Windows Embedded POSReady 2009",
	"Windows Embedded Standard 7",
	"Windows Embedded POSReady 7",
	"Windows Embedded 8 Industry",
	"Windows Embedded 8.1 Industry",
	"Windows 10 IoT",
];
static DEVICE_VERSIONS_LEN: usize = DEVICE_VERSIONS.len();
