extern crate html5ever;
extern crate reqwest;
extern crate string_cache;

use std::default::Default;

use reqwest::Client;

use html5ever::parse_document;
use html5ever::rcdom::{NodeData, RcDom, Handle};
use html5ever::tendril::TendrilSink;

use string_cache::Atom;

fn main() {
    let url = "http://www.bailly-immo.com/catalog/advanced_search_result.php?action=update_search&search_id=&C_28_search=EGAL&C_28_type=UNIQUE&C_28=Vente&C_27_search=EGAL&C_27_type=UNIQUE&C_27=23&C_65_search=CONTIENT&C_65_type=TEXT&C_65=85000+LA+ROCHE+SUR+YON&C_65_temp=85000+LA+ROCHE+SUR+YON&C_30=9&C_30_search=COMPRIS&C_30_type=NUMBER&C_30_MIN=&C_30_MAX=&C_30_loc=9&C_33_search=COMPRIS&C_33_type=NUMBER&C_33_MAX=&C_33_MIN=0&C_34=0&C_34_search=COMPRIS&C_34_type=NUMBER&C_34_MIN=&C_34_MAX=";

    let client = Client::new();
    let result = client.get(url)
                    .send()
                    .map(|mut response| response.text())
                    .and_then(|n| n);

    if result.is_err() {
        println!("Error with the request");
        return;
    }

    let dom_result = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut result.unwrap().as_bytes());

    if dom_result.is_err() {
        println!("Error while parsing result of the request");
        return;
    }

    walk(dom_result.unwrap().document);
}


fn walk(handle: Handle) {
    let div = Atom::from("div");

    let node = handle;

    match node.data {
        NodeData::Element { ref name, .. } => {
            if name.local == div {
                println!("{}", name.local);
            }
        },
        _ => {},
    };

    for child in node.children.borrow().iter() {
        walk(child.clone());
    }
}
