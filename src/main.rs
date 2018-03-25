extern crate kuchiki;
extern crate reqwest;

use kuchiki::iter::Siblings;
use kuchiki::NodeRef;
use kuchiki::traits::*;
use reqwest::Client;

struct Good {
    reference: Option<String>,
    description: Option<String>,
    price: Option<String>,
    link: Option<String>
}

fn main() {
    let url = "http://www.bailly-immo.com/catalog/advanced_search_result.php?action=update_search&search_id=&C_28_search=EGAL&C_28_type=UNIQUE&C_28=Vente&C_27_search=EGAL&C_27_type=UNIQUE&C_27=23&C_65_search=CONTIENT&C_65_type=TEXT&C_65=85000+LA+ROCHE+SUR+YON&C_65_temp=85000+LA+ROCHE+SUR+YON&C_30=9&C_30_search=COMPRIS&C_30_type=NUMBER&C_30_MIN=&C_30_MAX=&C_30_loc=9&C_33_search=COMPRIS&C_33_type=NUMBER&C_33_MAX=&C_33_MIN=0&C_34=0&C_34_search=COMPRIS&C_34_type=NUMBER&C_34_MIN=&C_34_MAX=";

    let client = Client::new();
    client.get(url)
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

                let refe = get_text_of_node(children.clone(), "span.listing_ref");
                let desc = get_text_of_node(children.clone(), "p");
                let price = get_text_of_node(children.clone(), "span.listing_price");
                let link = get_src_value_of_node(children.clone(), "a");

                println!("{:?},{:?},{:?},{:?}", refe, desc, price, link);
            }
        });
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
    Some(format!("http://www.bailly-immo.com/{}",src.unwrap()))
}

fn stringify_reqwest_error(x: reqwest::Error) -> String { format!("{:?}", x) }
