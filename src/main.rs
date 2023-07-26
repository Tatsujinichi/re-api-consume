use std::fmt::format;
use std::fs;
use serde::{Deserialize, Serialize};
use reqwest;
use reqwest::{Error, Client};
use std::collections::HashMap;
use futures::future;
use tokio;
use serde_json::json;

// https://www.land.mlit.go.jp/webland_english/note.html
// https://www.land.mlit.go.jp/webland/api.html

// https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest
#[tokio::main]
async fn main() -> Result<(), Error> {
    for n in 1..=47 {
        let padded = n.to_string();
        let padded = format!("{:0>2}", &padded);
        // let api_address = "https://www.land.mlit.go.jp/webland/api/CitySearch?area=";
        let api_address = "https://www.land.mlit.go.jp/webland_english/api/CitySearch?area=";
        let url = [api_address, &padded].join("");
        println!("{url}");
        let resp = get_request(&url).await?;
        println!("{:#?}", &resp);
        
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct CitySearchReponse {
    status: String,
    data: Vec<CityData>
}

#[derive(Deserialize, Debug)]
struct CityData {
    id: String,
    name: String,
}

async fn get_request(url: &str) -> Result<CitySearchReponse, Error> {
    let resp: CitySearchReponse = reqwest::get(url)
        .await?
        .json()
        .await?;
    // println!("Status: {}", response.status());

    // let body = response.text().await?;
    // println!("Body:\n{:#}", body);

    Ok(resp)
}

// [Japanese version] https://www.land.mlit.go.jp/webland/api/TradeListSearch?<Parameter>
// [English version] https://www.land.mlit.go.jp/webland_english/api/ TradeListSearch?<parameter>

// from: year / quarter
// YYYYN

// to: 

// area: Prefecture code NN

// city: NNNNN

// stations: NNNNN  0 prefix padded

// output
// Type	                transaction type	
// Region	            district	
// Municipality Code	City code	
// Prefecture	        Name of prefectures	
// Municipality	        City name	
// District Name	    District name	
// Trade Price	        Transaction price (gross price)	
// Price Per Unit	Unit price per tsubo	Not included in English version
// Floor Plan	Floor plan	
// Area	area (square meters)	
// Unit Price	Transaction price (per square meter)	
// Land Shape	land shape	
// Frontage	frontage	
// Total Floor Area	Total floor area (square meters)	
// Building Year	year of construction	
// Structure	building structure	
// Use	Usage	
// Purpose	Future purpose of use	
// Direction	Front road: direction	
// Classification	Front road: type	
// Breadth	Front road: Width (m)	
// City Planning	city ​​planning	
// Coverage ratio	Building coverage ratio (%)	
// Floor Area Ratio	Floor-area ratio(%)	
// period	time of transaction	
// renovation	renovation	
// Remarks	Circumstances of transactions, etc.



// List of municipalities in prefectures acquisition API
// By specifying the following parameters, you can obtain a list of municipality codes and municipality names within prefectures.
// [Japanese version] https://www.land.mlit.go.jp/webland/api/CitySearch?<Parameter>
// [English version] https://www.land.mlit.go.jp/webland_english/api/ CitySearch?<parameter>

// <<parameter> >
// parameter	content	remarks	Required
// area	Prefecture code	The format is NN (2 digits)
// NN … Prefecture code	○

// <<Output>>
// Output format: JSON format
// The tags are as shown in the table below.
// tag name	content
// id	City code
// name	City name

// <<Usage example>>
// https://www.land.mlit.go.jp/webland/api/CitySearch?area=13
// Get a list of municipalities in Tokyo.
// reference	List of prefecture codes
// The list of prefecture codes and prefecture names used on this website is as follows.
fn get_eng_jpn_prefectures() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("Hokkaido", "北海道"),
        ("Aomori", "青森県"),
        ("Iwate", "岩手県"),
        ("Miyagi", "宮城県"),
        ("Akita", "秋田県"),
        ("Yamagata", "山形県"),
        ("Fukushima", "福島県"),
        ("Ibaraki", "茨城県"),
        ("Tochigi", "栃木県"),
        ("Gunma", "群馬県"),
        ("Saitama", "埼玉県"),
        ("Chiba", "千葉県"),
        ("Tokyo", "東京都"),
        ("Kanagawa", "神奈川県"),
        ("Niigata", "新潟県"),
        ("Toyama", "富山県"),
        ("Ishikawa", "石川県"),
        ("Fukui", "福井県"),
        ("Yamanashi", "山梨県"),
        ("Nagano", "長野県"),
        ("Gifu", "岐阜県"),
        ("Shizuoka", "静岡県"),
        ("Aichi", "愛知県"),
        ("Mie", "三重県"),
        ("Shiga", "滋賀県"),
        ("Kyoto", "京都府"),
        ("Osaka", "大阪府"),
        ("Hyogo", "兵庫県"),
        ("Nara", "奈良県"),
        ("Wakayama", "和歌山県"),
        ("Tottori", "鳥取県"),
        ("Shimane", "島根県"),
        ("Okayama", "岡山県"),
        ("Hiroshima", "広島県"),
        ("Yamaguchi", "山口県"),
        ("Tokushima", "徳島県"),
        ("Kagawa", "香川県"),
        ("Ehime", "愛媛県"),
        ("Kochi", "高知県"),
        ("Fukuoka", "福岡県"),
        ("Saga", "佐賀県"),
        ("Nagasaki", "長崎県"),
        ("Kumamoto", "熊本県"),
        ("Oita", "大分県"),
        ("Miyazaki", "宮崎県"),
        ("Kagoshima", "鹿児島県"),
        ("Okinawa", "沖縄県"),
    ])
}


#[repr(u8)]
enum Prefecture {
    Hokkaido = 01,
    Aomori = 02,
    Iwate = 03,
    Miyagi = 04,
    Akita = 05,
    Yamagata = 06,
    Fukushima = 07,
    Ibaraki = 08,
    Tochigi = 09,
    Gunma = 10,
    Saitama = 11,
    Chiba = 12,
    Tokyo = 13,
    Kanagawa = 14,
    Niigata = 15,
    Toyama = 16,
    Ishikawa = 17,
    Fukui = 18,
    Yamanashi = 19,
    Nagano = 20,
    Gifu = 21,
    Shizuoka = 22,
    Aichi = 23,
    Mie = 24,
    Shiga = 25,
    Kyoto = 26,
    Osaka = 27,
    Hyogo = 28,
    Nara = 29,
    Wakayama = 30,
    Tottori = 31,
    Shimane = 32,
    Okayama = 33,
    Hiroshima = 34,
    Yamaguchi = 35,
    Tokushima = 36,
    Kagawa = 37,
    Ehime = 38,
    Kochi = 39,
    Fukuoka = 40,
    Saga = 41,
    Nagasaki = 42,
    Kumamoto = 43,
    Oita = 44,
    Miyazaki = 45,
    Kagoshima = 46,
    Okinawa = 47,
}