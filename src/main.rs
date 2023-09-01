//
// Copyright 2023 Virtualnation Pty Ltd.
//
// Parabellum - Comply - ISM
// 
// Publishes the OSCAL definition of the ACSC ISM via a REST API
// to support direct security control traces into a system.
//
// https://www.cyber.gov.au/resources-business-and-government/essential-cyber-security/ism/oscal
// 


// std to read in the OSCAL ISM XML definition.
use std::fs::File;
use std::io::{Read, Write};
use std::env;

// For sorting collections.
use std::collections::BTreeMap;

// From https://github.com/RazrFalcon/roxmltree
// A read only XML parsing library. 
use roxmltree::*;

// Mechanism to publish Security controls and groups.
use serde::{Serialize, Deserialize};

// Web server library
use actix_web::{http::header::ContentType, get, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

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
    essential_eight_applicability: String,
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
            essential_eight_applicability: String::from("N/A"),
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
#[derive(Serialize, Deserialize, Debug)]
pub  struct IsmControlSummary {
    id: String,
    title: String, 
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
                    let mut value = attributes.next().unwrap().value().to_string();
                    if nm.ne("sort-id") {
                        value = attributes.next().unwrap().value().to_string();
                    }
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
                      "essential-eight-applicability" => {
                          current_control.essential_eight_applicability = value;
                      }
                      _ => {
                      }
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
fn produce_markdown() -> std::io::Result<()> {
    let mut dir = env::current_dir().unwrap();
    dir.push("static");
    dir.push("ism");
    dir.push("control");
    let _ = std::fs::create_dir_all(dir.clone());
    let mut pwd : String = String::new();
    pwd.push_str(&dir.to_str().unwrap());
    let mut control_index = BTreeMap::<String, String>::new();
    
    unsafe {
        for ism_control in ISM_CONTROLS.iter() {
            let ism_filename = format!("{}/{}.md", pwd, ism_control.id);
            let mut output = File::create(ism_filename)?;

            write!(output, "### {}; Revision: {}; Updated: {}; Applicability: {}; Essential Eight: {}\n{}", ism_control.title, ism_control.revision, ism_control.updated, ism_control.applicability, ism_control.essential_eight_applicability, ism_control.statement)?;
            
            let mut index_string = String::from("| [*](/ism/control/*) | ");
            index_string = index_string.replacen('*', ism_control.id.as_str(), 2);
            index_string.push_str(ism_control.statement.replace("\n", "").as_str());
            index_string.push_str(" | \n");

            control_index.insert(ism_control.id.clone(), index_string);
        }
    }

    let mut ism_values: Vec<_> = control_index.values().cloned().collect();
    ism_values.sort();
    let mut index_md = String::new();
    index_md.push_str("# ISM CONTROL INDEX\n|    ISM Control   | Statement |\n| :-------------: | ----------- |\n");
    for ism_value in ism_values.iter() {
        index_md.push_str(ism_value.as_str());
    }
    
    let ism_index_filename = format!("{}/index.md", pwd);
    let mut index_output = File::create(ism_index_filename)?;
    write!(index_output, "{}", index_md)?;
    Ok(())
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("ACSC ISM June 2023 Reference Library")
}

#[get("/group/")]
async fn groups() -> impl Responder {
    let mut groups_json : String = String::from("");
    unsafe {
        groups_json.push_str(&serde_json::to_string_pretty(&ISM_GROUPS).unwrap());
    }
    HttpResponse::Ok().
        content_type(ContentType::json()).
        body(groups_json)
}

#[get("/control/")]
async fn controls() -> impl Responder {
    let mut controls_json : String = String::from("");
    unsafe {
        controls_json.push_str(&serde_json::to_string_pretty(&ISM_CONTROLS).unwrap());
    }
    HttpResponse::Ok().
        content_type(ContentType::json()).
        body(controls_json)
}

#[get("/control/sum")]
async fn controls_sum() -> impl Responder {
    let mut control_json : String = String::from("[");
    unsafe {
        for ism_control in ISM_CONTROLS.iter() {
            let  sum : IsmControlSummary = IsmControlSummary {
                id : ism_control.id.to_string(),
                title: ism_control.title.to_string(),
            };
            control_json.push_str(",\n");
            control_json.push_str(&serde_json::to_string(&sum).unwrap());
        }
    }
    control_json.push_str("]");

    HttpResponse::Ok().
        content_type(ContentType::json()).
        body(control_json)
}

#[get("/control/{control_id}")]
async fn control(control_id: web::Path<String>) -> impl Responder {
    let mut control_json : String = String::new();
    let id = control_id.to_string();
    unsafe {
        for ism_control  in ISM_CONTROLS.iter() {
            if id.eq (&ism_control.id) {
                control_json.push_str(&serde_json::to_string_pretty(&ism_control).unwrap());
                break;
            }
        }
    }
    HttpResponse::Ok().
        content_type(ContentType::json()).
        body(control_json)
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
    
    produce_markdown()?;
    
    HttpServer::new(|| {
         App::new()
            .service(home)
            .service(controls_sum)
            .service(control)
            .service(controls)
            .service(groups)
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(
                fs::Files::new("/ism", "./static/ism")
                    .show_files_listing()
                    .use_last_modified(true),
            )

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

