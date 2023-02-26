use mongodb:: {
    bson::doc,
    sync::Client, options,
};
use serde:: {Deserialize , Serialize};

#[derive(Debug , Serialize , Deserialize)]
struct User {
  name: String,
  password: String,

}

fn add_user(client: mongodb::sync::Client){
    let db = client.database("firstDatabase");
    let coll = db.collection("users");
    coll.insert_one(doc!{"name": "hardik" , "password" : "12345"}, None);
    coll.insert_one(doc!{"name" : "kartik" , "password": "3333"} ,  None);


} 

fn find_user(coll: mongodb::sync::Collection<User>){
    let find = coll.find(doc! {"name": "kartik"} , None).unwrap();
    for res in find {
        println!("{:?}" , res);

    }
}

 fn delete_user(coll: mongodb::sync::Collection<User>){
    let del = coll.delete_one(doc!{"name" : "kartik"}, None).unwrap();
    
        println!("{:?}", del);

    }



fn main() {
    println!("Hello, world!");
    let client = Client::with_uri_str("mongodb+srv://hardikjain:2130@cluster0.nfcjepp.mongodb.net/test?w=majority").unwrap();
    //   add_user(client);
      let db = client.database("firstDatabase");
      let coll = db.collection::<User>("users");
    //   find_user(coll);

      delete_user(coll);
     
}
