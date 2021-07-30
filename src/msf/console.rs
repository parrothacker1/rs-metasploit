#![allow(non_camel_case_types)]
#[path="../connect.rs"] mod connect;
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
use common::{MsfError,create,read,Return_Type};
use error::conerr;
use serde_json::from_value;
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
pub fn create(client:Client) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.create".to_string()),connect::Parse_Type::String(client.token.unwrap())];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            let ret:create=from_value(val).unwrap();
            match ret {
                Ok => {
                    let retv:create=from_value(val).unwrap();
                    test=Return_Type::ConsoleCreate(retv);
                },
                Err => {
                    let retval:MsfError=from_value(val).unwrap();
                    test=Return_Type::MsfErr(retval);
                },
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn destroy(client:Client,consoleid:String) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.destroy".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(consoleid)];
    let conn=connect::connect(client.url,body);
    match conn {
        Ok(val) => {
            if val.get("result") != None && val.get("result").unwrap().as_str().unwrap()=="success" {
                test=Return_Type::Bool(true);
            } else {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn list(client:Client) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.list".to_string()),connect::Parse_Type::String(client.token.unwrap())];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {

        },
        Err(_e) => {
        test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn write(client:Client,consoleid:String,data:String) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.write".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(consoleid),connect::Parse_Type::String(data)];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("wrote") == None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                let wrote_string:String=val.get("wrote").unwrap().as_str().unwrap().to_string();
                let wrote:i32=wrote_string.parse().unwrap();
                test=Return_Type::Int(wrote);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn read(client:Client,consoleid:String) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.read".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(consoleid)];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("data")==None {
                let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			} else {
				let ret:read=from_value(val).unwrap();
                test=Return_Type::ConsoleRead(ret);
			};
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn session_detach(client:Client,consoleid:String) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.session_detach".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(consoleid)];
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				test=Return_Type::Bool(true);
			} else {
				let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
pub fn session_kill(client:Client,consoleid:String) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.session_kill".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(consoleid)];
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				test=Return_Type::Bool(true);
			} else {
				let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
pub fn tabs(client:Client,consoleid:String,inputline:String) -> Return_Type {
    let test;
    let body=vec![connect::Parse_Type::String("console.tabs".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(consoleid),connect::Parse_Type::String(inputline)];
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			if val.get("tabs")==None {
				let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			} else {
				let ret=val.get("tabs").unwrap().as_array().unwrap().to_vec();
				test=Return_Type::Array(ret);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
