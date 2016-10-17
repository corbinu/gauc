extern crate clap;
extern crate iron;
extern crate router;

mod handler;

//use iron::prelude::*;
//use iron::status;
//use router::Router;
//
//use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc;
//
//use self::handler::bucket::doc;
use super::client::Client;
//use super::couchbase::types::response;

// Bucket REST Interface
//
// GET  /bucket/<BUCKET_NAME>/doc - list
// POST /bucket/<BUCKET_NAME>/flush - flush
//
// DELETE  /bucket/<BUCKET_NAME>/doc/<ID>            - delete *
// GET     /bucket/<BUCKET_NAME>/doc/<ID>            - get *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>            - upsert *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/add        - add *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/append     - append *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/prepend    - prepend *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/replace    - replace *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/set        - set *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/upsert     - upsert (explitcit) *

pub fn start_web(args: &clap::ArgMatches, _client: &Client) {
    let port: u16 = args.value_of("rest-port").unwrap().to_string().parse::<u16>().unwrap();
    println!("Starting REST Interface on port {}.", port);

//    let mut router = Router::new();
//
//    type CouchbaseResult = Result<&'static response::Store, (Option<&'static response::Store>, &'static str)>;
//
//    let get_handler = move |req: &mut Request| -> IronResult<Response> {
//        let (tx, rx): (Sender<&CouchbaseResult>, Receiver<&CouchbaseResult>) = mpsc::channel();
//        client.upsert("abc", "def", |result| {
//            // tx.send(result)
//        });
//        let my_result = rx.recv().unwrap();
//        Ok(Response::with((status::Ok, "ok\n")))
//    };
//
//    router.get("/bucket/:bucketid/doc/:docid", get_handler, "doc_get");
//    router.get("/bucket/:bucketid/doc/:docid", doc::get::get_handler, "doc_get");
//    router.delete("/bucket/:bucketid/doc/:docid", doc::delete::delete_handler, "doc_delete");
//    router.post("/bucket/:bucketid/doc/:docid", doc::upsert::upsert_handler, "doc_insert");
//    router.post("/bucket/:bucketid/doc/:docid/add", doc::add::add_handler, "doc_add");
//    router.post("/bucket/:bucketid/doc/:docid/append", doc::append::append_handler, "doc_append");
//    router.post("/bucket/:bucketid/doc/:docid/prepend", doc::prepend::prepend_handler, "doc_prepend");
//    router.post("/bucket/:bucketid/doc/:docid/replace", doc::replace::replace_handler, "doc_replace");
//    router.post("/bucket/:bucketid/doc/:docid/set", doc::set::set_handler, "doc_set");
//    router.post("/bucket/:bucketid/doc/:docid/upsert", doc::upsert::upsert_handler, "doc_upsert");
//
//    let address = format!("localhost:{}", port);
//    Iron::new(router).http(&address[..]).unwrap();
}
