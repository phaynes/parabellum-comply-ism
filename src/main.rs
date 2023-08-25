//
// Copyright 2023 Virtualnation Pty Ltd.
//
//
// Parabellum - Comply - ISM
// 
// An application to publish the OSCAL definition of the ACSC ISM via a REST API
// to support direct security control traces into a system.

// https://www.cyber.gov.au/resources-business-and-government/essential-cyber-security/ism/oscal
// 


// std to read in the OSCAL ISM XML definition.
use std::fs::File;
use std::io::Read;

// From https://github.com/RazrFalcon/roxmltree
// A read only XML parsing library. 
use roxmltree::*;

// Mechanism to publish Security controls and groups.
use serde::{Serialize, Deserialize};

// Web server library
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

//
// Models an ISM Group. An ISM group will typically contain subgroups or 
// security controls. 
// The sort_id field provides a specific mechanism for ordering the controls. 
// The implmentation removes surrounding words to simplify integration.
// 
#[derive(Serialize, Deserialize, Debug)]
pub struct IsmGroup {
    title: String, 
    overview: String,
    sort_id: String, 
}
impl IsmGroup {
    pub fn new() -> IsmGroup {
        IsmGroup {
            title: String::new(),
            overview: String::new(), 
            sort_id: String::new(),
        }
    }
    // sort_id is a field to support sorting and grouping of the different 
    // controls and groups. 
    // Withing the ism oscal definition it is of the form
    //   "catalog[1].group[03].group[1].group[03].control[2]"
    // 
    // While good for readabilty, it is less so for programatic use. 
    // This method strip out the text, so for the group definition about
    // the value 03,1,03,2 would be stored as the catalog definition is constant.
    //
    pub fn set_sort_id(&mut self, id: &str) {
        let mut work_str = id.replace("catalog[1].", "");
        work_str = work_str.replace("group[", "");
        work_str = work_str.replace(".", ",");
        work_str = work_str.replace("]","");
        self.sort_id = String::from(work_str);  
    }
}

//
// Models an ISM control based off the OSCAL definition.
// 
// The OSCAL document is built and parsed into this structure to support serialisation.
#[derive(Serialize, Deserialize, Debug)]
pub struct IsmControl {
    group: String,
    id: String,
    title: String, 
    revision: String, 
    updated: String,
    applicability: String,
    statement: String,
    class: String,
    sort_id: String,
}
impl IsmControl {
    pub fn new() -> IsmControl {
        IsmControl {
            group: String::new(),
            id: String::new(),
            title: String:: new(),
            revision: String::new(),
            updated: String::new(),
            applicability: String::new(),
            statement: String::new(),
            class: String::new(),
            sort_id: String::new(),
        }
    }
    // sort_id field supports parsing 
    pub fn set_sort_id(&mut self, id: &str) {
        let mut work_str = id.replace("catalog[1].", "");
        work_str = work_str.replace("group[", "");
        work_str = work_str.replace("control[", "");
        work_str = work_str.replace(".", ",");
        work_str = work_str.replace("]","");
        self.sort_id = String::from(work_str);  
    }
}

static mut ISM_CONTROLS : Vec::<IsmControl> = Vec::<IsmControl>::new();
static mut ISM_GROUPS : Vec::<IsmGroup>  =  Vec::<IsmGroup>::new();

// Parses an ISM control OSCAL XML fragment
fn process_ism_control<'a, 'input>(ism_group : &String, node : &Node<'a, 'input> ) -> serde_json::Result<String> {
    
    let mut current_control = IsmControl::new();

    let mut attributes = node.attributes();
    current_control.group = ism_group.to_string();
    current_control.id = attributes.next().unwrap().value().to_string();
    current_control.class = attributes.next().unwrap().value().to_string();
    
    for n in node.children() {
        let tag = n.tag_name().name();
        match tag {
            "title" => { 
                let title = n.text().unwrap().to_string();
                current_control.title = title;
            }
            "prop" => {
                let mut attributes = n.attributes();
                if attributes.len() > 0 {
                    let nm = attributes.next().unwrap().value();
                    let value = attributes.next().unwrap().value().to_string();
                    match nm {
                      "sort-id" => {
                          current_control.set_sort_id(&value.as_str());
                      }
                      "revision" => {
                          current_control.revision = value;
                      }
                      "updated" => {
                          current_control.updated = value;
                      }
                      "applicability" => {
                          current_control.applicability = value;
                      }
                      _ => {}
                  }
                }    
            }
            "part" => {
                let mut attributes = n.attributes();
                if attributes.len() > 0 {
                  let nm = attributes.next().unwrap().value();
                  let id =attributes.next().unwrap().value(); 
                  if nm.eq("statement") {
                    let xml_src =  n.document().input_text();
                    let start = n.range().start + "<part name=\"statement\" + id=>\r".len() + id.len() + 2;
                    let end = n.range().end - "</part>".len(); 
                    let xml_frag = &xml_src[start..end];
                    current_control.statement = String::from(xml_frag.trim());
                  }
                }

            }
            _ => {
            }
        }
    }

    let result = serde_json::to_string(&current_control)?;
    unsafe {
        ISM_CONTROLS.push(current_control);
    }
    return Ok(result);
}

// Parses and ISM Group oscal definition.
fn process_ism_group<'a, 'input>(node : &Node<'a, 'input> ) ->  serde_json::Result<String>  {
    let mut current_group = IsmGroup::new();
    for n in node.children() {
        let tag = n.tag_name().name();
        match tag {
            "group" => {
                process_ism_group(&n)?;
            }
            "prop" => {
                let mut attributes = n.attributes();
                if attributes.len() > 0 {
                  let nm = attributes.next().unwrap().value();
                  if nm.eq("sort-id") {
                      current_group.set_sort_id(attributes.next().unwrap().value());
                  }
                }    
            }
            "control" => {
                process_ism_control(&current_group.sort_id, &n)?;
            }
            "title" => { 
                let title = n.text().unwrap().to_string();
                current_group.title = title;
            }
            "" => {
            }
            "part" => {
                let mut attributes = n.attributes();
                if attributes.len() > 0 {
                  let nm = attributes.next().unwrap().value();
                  if nm.eq("overview") {
                    let xml_src =  n.document().input_text();
                    let start = n.range().start + "<part name=\"overview\">\r".len();
                    let end = n.range().end - "</part>".len(); 
                    let xml_frag = &xml_src[start..end];
                    current_group.overview = String::from(xml_frag);
                  }
                }

            }
            _ => {
            }
        }
    }
    let result = serde_json::to_string(&current_group)?;
    unsafe {
      ISM_GROUPS.push(current_group);
    }
    return Ok(result);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut ism_file = File::open("./data/ISM_catalog.xml")?;

    // Read the file content into a string
    let mut ism_xml = String::new();
    ism_file.read_to_string(&mut ism_xml)?;
    let doc = roxmltree::Document::parse(&ism_xml).unwrap();
    for node in doc.root().descendants().filter(|n| n.is_element() && n.tag_name().name().eq("group")) {
        process_ism_group(&node)?;
    }

    HttpServer::new(|| {
         App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

