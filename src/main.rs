use jsonrpc_core::*;
use jsonrpc_http_server::*;
use sled::Db;
use serde_json::Map;

/// # author
/// db: author
///   + tree: _
///   + ...authors
///
/// # art
/// db: art
///   + tree: public_key
///     + ...arts
///
fn main() {
    let mut io = IoHandler::new();

    io.add_method("x", |p: Params| {
        // data
        let d: Map<String, Value> = p.parse().unwrap();

        // database
        let db_name = d.get("db").unwrap().as_str().unwrap();
        let mut path = dirs::home_dir().unwrap();
        path = path.join(".lark_in");
        path = path.join(db_name);

        // tree
        let db = Db::start_default(path).unwrap();
        let mut t: std::sync::Arc<sled::Tree>;
        let tree = d.get("tree");

        if tree.is_some() {
            t = db.open_tree(
                tree.unwrap().as_str().unwrap().as_bytes()
            ).unwrap();
        } else {
            t = db.open_tree(b"default").unwrap();
        }

        // contents
        let batch = d.get("batch");
        if batch.is_some() {
            match batch.unwrap().as_str().unwrap() {
                "true" => {
                    let mut values: Vec<Value> = vec![];
                    for i in t.iter() {
                        values.push(
                            Value::Array(vec![
                                Value::String(
                                    String::from_utf8(i.to_owned().unwrap().0.to_vec()).unwrap()
                                ),   
                                Value::String(
                                    String::from_utf8(i.unwrap().1.to_vec()).unwrap()
                                )
                            ])
                        );
                    }
                    return Ok(Value::Array(values))
                },
                _ => {
                    return Ok(Value::String("BATCH ERROR".to_string()))
                }
            }
        }

        // key && value
        let _key = d.get("key");
        let value =  d.get("value");
        
        let key: String;
        if _key.is_some() {
            key = _key.unwrap().as_str().unwrap().to_string();
        } else {
            key = String::from_utf8(
                (t.len() + 1).to_be_bytes().to_vec()
            ).unwrap();
        }

        // storage
        t.flush().unwrap();
        if value.is_some() {
            match t.set(
                key.as_bytes(),
                value.unwrap().as_str().unwrap().as_bytes()
            ).is_ok() {
                true => Ok(Value::String("OK".to_string())),
                false => Ok(Value::String("ERROR".to_string()))
            }
        } else {
            let data = t.get(key.as_bytes()).unwrap();

            match data.is_some() {
                true => {
                    Ok(Value::String(
                        String::from_utf8(data.unwrap().to_vec()
                        ).unwrap()))
                },
                false => {
                    Ok(Value::String("Err".to_string()))
                }
            }
        }
    });
    
    println!("server start at 3030...");
    let _server = ServerBuilder::new(io)
	.start_http(&"127.0.0.1:3030".parse().unwrap())
	.expect("Unable to start RPC server");

    _server.wait(); 
}
