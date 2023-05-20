#[macro_use] extern crate rocket;
use oracle::{Connection, Result};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use rocket::http::Status;


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

#[derive(serde::Deserialize)]
struct Insparams {
    pSSN: Option<String>,
    pBDate: Option<String>,
    pFname: Option<String>,
    pLname: Option<String>,
    pType: Option<String>,
    pCountry: Option<String>,
    pCity: Option<String>,
    pStreet: Option<String>,
    pSalary: Option<String>,
    pSex: Option<String>,
    pBranchID: Option<String>,
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![post,ins])
}

#[post("/new", data = "<params>")]
fn ins(params: Json<Insparams>) -> Status {
    if postNew(params).unwrap() {
        Status::Ok
    } else {
        Status::BadRequest
    }
}



fn postNew(params: Json<Insparams>) -> Result<bool> {
    let username = "system";
    let password = "system";
    let database = "//localhost:1521/ORCL";

    let mut mypSSN = "";
    let mut mypBDate = "";
    let mut mypFname = "";
    let mut mypLname = "";
    let mut mypType = "";
    let mut mypCountry = "";
    let mut mypCity = "";
    let mut mypStreet = "";
    let mut mypSalary = "";
    let mut mypSex = "";
    let mut mypBranchID = "";
    if let Some(pSSN) = &params.pSSN {
        mypSSN = pSSN;
    }
    
    if let Some(pBDate) = &params.pBDate {
        mypBDate = pBDate;
    }

    if let Some(pFname) = &params.pFname {
        mypFname = pFname;
    }

    if let Some(pLname) = &params.pLname {
        mypLname = pLname;
    }

    if let Some(pType) = &params.pType {
        mypType = pType;
    }

    if let Some(pCountry) = &params.pCountry {
        mypCountry = pCountry;
    }

    if let Some(pCity) = &params.pCity {
        mypCity = pCity;
    }

    if let Some(pStreet) = &params.pStreet {
        mypStreet = pStreet;
    }

    if let Some(pSalary) = &params.pSalary {
        mypSalary = pSalary;
    }

    if let Some(pSex) = &params.pSex {
        mypSex = pSex;
    }

    if let Some(pBranchID) = &params.pBranchID {
        mypBranchID = pBranchID;
    }



    let sql = format!("INSERT INTO EMPLOYEES VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')", mypSSN, mypBDate, mypFname, mypLname, mypType, mypCountry, mypCity, mypStreet, mypSalary, mypSex, mypBranchID);


    let conn = Connection::connect(username, password, database)?;
    let mut stmt = conn.statement(&sql.to_string()).build()?;
    let rows = stmt.execute(&[])?;

    //print statement results
    conn.commit()?;

    Ok(true)
}

fn getProduct(params: Json<ParamsProducts>) -> Result<Vec<Product>> {
    let username = "user";
    let password = "user";
    let database = "//localhost:1521/ORCL";

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
