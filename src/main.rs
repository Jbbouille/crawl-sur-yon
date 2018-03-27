extern crate bincode;
extern crate kuchiki;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use bincode::{deserialize_from, serialize};
use kuchiki::iter::Siblings;
use kuchiki::NodeRef;
use kuchiki::traits::*;
use reqwest::Client;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Good {
    reference: Option<String>,
    description: Option<String>,
    price: Option<String>,
    link: Option<String>,
}

fn main() {
    let path = "goods-datafile.yon";
    let mut previous_goods = read_file(path);
    let mut new_good_counter = 0;

    let url = "http://www.bailly-immo.com/catalog/advanced_search_result.php?action=update_search&search_id=&C_28_search=EGAL&C_28_type=UNIQUE&C_28=Vente&C_27_search=EGAL&C_27_type=UNIQUE&C_27=23&C_65_search=CONTIENT&C_65_type=TEXT&C_65=85000+LA+ROCHE+SUR+YON&C_65_temp=85000+LA+ROCHE+SUR+YON&C_30=9&C_30_search=COMPRIS&C_30_type=NUMBER&C_30_MIN=&C_30_MAX=&C_30_loc=9&C_33_search=COMPRIS&C_33_type=NUMBER&C_33_MAX=&C_33_MIN=0&C_34=0&C_34_search=COMPRIS&C_34_type=NUMBER&C_34_MIN=&C_34_MAX=";
    let client = Client::new();
    let result = client.get(url)
        .send()
        .map(|mut response| response.text())
        .and_then(|n| n)
        .map(|response_request| kuchiki::parse_html().one(response_request))
        .map_err(stringify_reqwest_error)
        .map(|document| document.select("div.bien div.col-md-12").map_err(|()| String::from("Error when selecting 'div.bien div.col-md-12'")))
        .and_then(|n| n)
        .map(|n| {
            for css_match in n {
                let as_node: &NodeRef = css_match.as_node();
                let children: Siblings = as_node.children();

                let reference = get_text_of_node(children.clone(), "span.listing_ref").map(|r| r.replacen("Rèf : ", "", 2));
                let description = get_text_of_node(children.clone(), "p");
                let price = get_text_of_node(children.clone(), "span.listing_price").map(|p| p.replacen("\u{a0}", "", 2));
                let link = get_src_value_of_node(children.clone(), "a");

                let good = Good { reference, description, price, link };
                if !previous_goods.contains(&good) {
                    previous_goods.push(good);
                    new_good_counter += 1
                }
            }
            previous_goods
        });

    if new_good_counter == 0 {
        println!("There are {} new goods!", new_good_counter);
        return;
    }

    if result.is_err() {
        panic!("{}, while crawling the web site {}.", result.unwrap_err(), url);
    };

    let goods = result.unwrap();
    if goods.is_empty() {
        panic!("There are no result !");
    };

    let encoded = serialize(&goods);
    if encoded.is_err() {
        panic!("{}, while serializing data to bincode format file.", encoded.unwrap_err());
    };

    let created = File::create("goods-datafile.yon");
    if created.is_err() {
        panic!("{}, while creating file.", created.unwrap_err());
    };

    let writing = created.unwrap().write_all(&encoded.unwrap());
    if writing.is_err() {
        panic!("{}, while writing in file.", writing.unwrap_err());
    }
}

fn read_file(path: &str) -> Vec<Good> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    let result = File::open(path);
    if result.is_err() {
        panic!("{}, while opening file {}.", result.unwrap_err(), path);
    }
    let mut br = BufReader::new(result.unwrap());
    let deserialize: Result<Vec<Good>, std::boxed::Box<bincode::ErrorKind>> = deserialize_from(&mut br);

    if deserialize.is_err() {
        panic!("{}, while deserialize file {}.", deserialize.unwrap_err(), path);
    }
    deserialize.unwrap()
}

fn get_text_of_node(children: Siblings, selector: &str) -> Option<String> {
    let selection = children.select(selector);
    if selection.is_err() {
        return None;
    }
    let first_selection = selection.unwrap().next();
    if first_selection.is_none() {
        return None;
    }
    Some(first_selection.unwrap().text_contents())
}

fn get_src_value_of_node(children: Siblings, selector: &str) -> Option<String> {
    let selection = children.select(selector);
    if selection.is_err() {
        return None;
    }
    let first_selection = selection.unwrap().next();
    if first_selection.is_none() {
        return None;
    }
    let data_ref = first_selection.unwrap();
    let x = data_ref.as_node().as_element().unwrap();
    let attributes = x.clone().attributes.into_inner();
    let src = attributes.get("href");
    if src.is_none() {
        return None;
    }
    Some(format!("http://www.bailly-immo.com/{}", src.unwrap()))
}

fn stringify_reqwest_error(x: reqwest::Error) -> String { format!("{:?}", x) }
