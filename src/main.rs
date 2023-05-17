#[macro_use] extern crate rocket;
use oracle::{Connection, Result};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;


#[derive(Serialize, Deserialize)]
struct Product {
    ProductID: Option<String>,
    ProductName: Option<String>,
    Quantity: Option<String>,
    Price: Option<String>,
    Brand: Option<String>,
    BranchID: Option<String>,
    SupplierID: Option<String>
}
/*
let productSQL = format!("SELECT * FROM INVENTORY WHERE PRODUCTID = '{}'", productID);
*/


#[derive(serde::Deserialize)]
struct ParamsProducts {
    pID: Option<String>,
    pBrand: Option<String>,
    pName: Option<String>,
}



#[post("/products", data = "<params>")]
async fn post(params: Json<ParamsProducts>) -> Option<Json<Vec<Product>>> {
   
    let s = getProduct(params).unwrap();
    if s.is_empty() {
        None
    } else {
        Some(Json(s.into_iter().map(|i| Product {
            ProductID: i.ProductID.clone(),
            ProductName: i.ProductName.clone(),
            Quantity: i.Quantity.clone(),
            Price: i.Price.clone(),
            Brand: i.Brand.clone(),
            BranchID: i.BranchID.clone(),
            SupplierID: i.SupplierID.clone(),
    }).collect()))
}
}
/*
#[post("/new", data = "<params>")]
async fn postEmp(params: Json<Params>) -> Boolean {
   
    let s = newEmp(params).unwrap();
    if s {
        true
    } else {
        false
    }
}
*/

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![post])
}

fn getProduct(params: Json<ParamsProducts>) -> Result<Vec<Product>> {
    let username = "odbc_jhc";
    let password = "odbc_jhc";
    let database = "//10.0.0.21:1521/a12";

    let mut mypID = "%";
    let mut mypBrand = "%";
    let mut mypName = "%";
    if let Some(pID) = &params.pID {
        mypID = pID;
    }

    if let Some(pBrand) = &params.pBrand {
        mypBrand = pBrand;
    }

    if let Some(pName) = &params.pName {
        mypName = pName;
    }

    
    let sql = format!("SELECT * FROM INVENTORY WHERE PRODUCTID LIKE '{}' AND BRAND LIKE '{}' AND PRODUCTNAME LIKE '{}'", mypID, mypBrand, mypName);
    
    let conn = Connection::connect(username, password, database)?;
    let mut stmt = conn.statement(&sql.to_string()).build()?;
    let rows = stmt.query(&[])?;

    let mut products : Vec<Product> = vec![];
    
    for row_result in rows {
        // print column values

        let row = row_result?;

        let ProductID : Option<String> = row.get("PRODUCTID")?;
        let ProductName : Option<String> = row.get("PRODUCTNAME")?;
        let Quantity : Option<String> = row.get("QUANTITY")?;
        let Price : Option<String> = row.get("PRICE")?;
        let Brand : Option<String> = row.get("BRAND")?;
        let BranchID : Option<String> = row.get("BRANCHID")?;
        let SupplierID : Option<String> = row.get("SUPPLIERID")?;

        

        let prod = Product {
            ProductID : ProductID,
            ProductName : ProductName,
            Quantity : Quantity,
            Price : Price,
            Brand : Brand,
            BranchID : BranchID,
            SupplierID : SupplierID,            
        };
        products.push(prod);
    }

    Ok(products)
}

/*
fn newEmp(params: Json<Params>) -> Boolean {
    let username = "odbc_jhc";
    let password = "odbc_jhc";
    let database = "//10.0.0.21:1521/a12";
    let mut mypRef = "%";
    let mut mypBarcode = "%";
    let mut mypId = "%";
    if let Some(pRef) = &params.pRef {
        mypRef = pRef;
    }

    if let Some(pBarcode) = &params.pBarcode {
        mypBarcode = pBarcode;
    }

    if let Some(pId) = &params.pId {
        mypId = pId;
    }

    let sql = format!("SELECT ITEM_ID, IS_ACTIVE, CAN_BE_SOLD, ITEM_DESC, ITEM_DESC_S, FOREIGN_ITEM_CODE, ITEM_CAT, ITEM_SUB_CAT, SALE_UNIT, UNIT_DESC, PACKING, CARD_OPEN_DATE, HS_CODE, COUNTRY, COUNTRY_DESC, SUPPLIER_ID, SUPPLIER_DESC, ITEM_MAIN_BARCODE, NATURE_ID, NATURE_DESC, TRADE_ID, TRADE_DESC, QTY_STORE_01, QTY_STORE_02, QTY_STORE_05, QTY_STORE_06, QTY_STORE_07, QTY_STORE_08, QTY_STORE_10, QTY_STORE_11, QTY_STORE_12, QTY_STORE_19, QTY_STORE_21, QTY_STORE_23, QTY_STORE_31, QTY_STORE_32, QTY_STORE_33, QTY_STORE_34, QTY_STORE_35, SALE_PRICE_NOTAX_STORE_01, SALE_PRICE_NOTAX_STORE_02, SALE_PRICE_NOTAX_STORE_05, SALE_PRICE_NOTAX_STORE_06, SALE_PRICE_NOTAX_STORE_08, SALE_PRICE_NOTAX_STORE_07, SALE_PRICE_NOTAX_STORE_31, SALE_PRICE_NOTAX_STORE_32, SALE_PRICE_NOTAX_STORE_33, SALE_PRICE_NOTAX_STORE_34, SALE_PRICE_NOTAX_STORE_35, FIRST_DISC_PER_STORE_01, FIRST_DISC_PER_STORE_02, FIRST_DISC_PER_STORE_05, FIRST_DISC_PER_STORE_06, FIRST_DISC_PER_STORE_07, FIRST_DISC_PER_STORE_08, FIRST_DISC_PER_STORE_31, FIRST_DISC_PER_STORE_32, FIRST_DISC_PER_STORE_33, FIRST_DISC_PER_STORE_34, FIRST_DISC_PER_STORE_35, SECOND_DISC_PER_STORE_01, SECOND_DISC_PER_STORE_02, SECOND_DISC_PER_STORE_05, SECOND_DISC_PER_STORE_06, SECOND_DISC_PER_STORE_07, SECOND_DISC_PER_STORE_08, SECOND_DISC_PER_STORE_31, SECOND_DISC_PER_STORE_32, SECOND_DISC_PER_STORE_33, SECOND_DISC_PER_STORE_34, SECOND_DISC_PER_STORE_35 FROM ODBC_JHC.JHC_INVDATA WHERE FOREIGN_ITEM_CODE LIKE '{}' AND ITEM_MAIN_BARCODE LIKE '{}' AND ITEM_ID LIKE '{}'", mypRef, mypBarcode, mypId);
    let conn = Connection::connect(username, password, database)?;
    let mut stmt = conn.statement(&sql.to_string()).build()?;
    let rows = stmt.query(&[])?;

    let mut products : Vec<PRODUCT> = vec![];
    
    for row_result in rows {
        // print column values

        let row = row_result?;
        let ITEM_ID : Option<String> = row.get("ITEM_ID")?;
        let IS_ACTIVE : Option<String> = row.get("IS_ACTIVE")?;
        

        let prod = PRODUCT {
            ITEM_ID : ITEM_ID,
            IS_ACTIVE : IS_ACTIVE,
            CAN_BE_SOLD : CAN_BE_SOLD,
            ITEM_DESC : ITEM_DESC,
            ITEM_DESC_S : ITEM_DESC_S,
            FOREIGN_ITEM_CODE : FOREIGN_ITEM_CODE,
            ITEM_CAT : ITEM_CAT,
            
        };
        products.push(prod);
    }

    Ok(products)
}
*/