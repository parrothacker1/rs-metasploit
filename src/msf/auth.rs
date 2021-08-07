#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use error::MsfError;
use crate::client;
use structs::{request as req,response as res};
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::Error as derror};

pub fn logout(clientdata:client::Client,out_tok:String) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::logout("auth.logout".to_string(),clientdata.token.as_ref().unwrap().to_string(),out_tok);
    byte.serialize(&mut serializer);
    let con=connect::connect(clientdata.url,body,&mut buf);
    let mut new_buf=buf.clone();
    match con {
		Ok(_) => {
			let mut de=Deserializer::new(new_buf.as_slice());
			let de_ret:Result<res::auth::logout,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				if val.result=="success".to_string() {
					test=Ok(true);
				} else {
					test=Ok(false);
				}
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			}
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
	test
}
pub fn token_add(clientdata:client::Client,new_tok:String) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokenadd("auth.token_add".to_string(),clientdata.token.as_ref().unwrap().to_string(),new_tok);
    byte.serialize(&mut serializer);
    let conn=connect::connect(clientdata.url,body,&mut buf);
    let mut new_buf=buf.clone();
    match conn {
		Ok(_) => {
			let mut de=Deserializer::new(new_buf.as_slice());
			let de_ret:Result<res::auth::tokenadd,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				if val.result=="success".to_string() {
					test=Ok(true);
				} else {
					test=Ok(false);
				}
			}
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			}
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn token_gen(clientdata:client::Client) -> Result<String,MsfError> {
    let mut test:Result<String,MsfError>=Ok(String::new());
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokengen("auth.token_generate".to_string(),clientdata.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer);
    let mut buf=vec![];
    let conn=connect::connect(clientdata.url,body,&mut buf);
    let new_buf=buf.clone();
    match conn {
		Ok(_) => {
			let mut de=Deserializer::new(new_buf.as_slice());
			let de_ret:Result<res::auth::tokengen,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				if val.result=="success".to_string() {
					test=Ok(val.token.clone());
				} else {
					panic!("Failed to retrive token.");
				}
			}
			if let Err(e) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			}
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn token_list(clientdata:client::Client) -> Result<Vec<String>,MsfError> {
    let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokenlist("auth.token_list".to_string(),clientdata.token.unwrap());
    byte.serialize(&mut serializer);
    let con=connect::connect(clientdata.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::auth::tokenlist,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				test=Ok(val.tokens.clone());
			}
			if let Err(e) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			}
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn token_remove(clientdata:client::Client,token_rem:String) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokenrem("auth.token_remove".to_string(),clientdata.token.unwrap(),token_rem);
    byte.serialize(&mut serializer);
    let con=connect::connect(clientdata.url,body,&mut buf);
    let mut new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::auth::tokenrem,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				if val.result=="success".to_string() {
					test=Ok(true);
				} else {
					test=Ok(false);
				}
			}
			if let Err(e) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			}
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
