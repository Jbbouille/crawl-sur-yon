extern crate reqwest;
extern crate kuchiki;

use kuchiki::traits::*;

use reqwest::Client;

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

    let css_selector = "div.bien div.col-md-12";
    let document = kuchiki::parse_html().one(result.unwrap());

    for css_match in document.select(css_selector).unwrap() {
        let as_node = css_match.as_node();

        let x = as_node.select("p").unwrap().next().unwrap();
        let text_description = x.as_node().as_text().unwrap().borrow();

        let y = as_node.select("span.listing_ref").unwrap().next().unwrap();
        let text__product_reference = x.as_node().as_text().unwrap().borrow();

        let z = as_node.select("span.listing_price").unwrap().next().unwrap();
        let text_product_price = z.as_node().as_text().unwrap().borrow();
        println!("{:?},{:?},{:?}", text__product_reference, text_description, text_product_price);
    }
}
