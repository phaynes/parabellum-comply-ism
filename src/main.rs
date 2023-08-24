
use std::fs::File;
use std::io::Read;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// From https://github.com/RazrFalcon/roxmltree
use roxmltree::*;
use serde::{Serialize, Deserialize};



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

    pub fn set_sort_id(&mut self, id: &str) {
        let mut work_str = id.replace("catalog[1].", "");
        work_str = work_str.replace("group[", "");
        work_str = work_str.replace(".", ",");
        work_str = work_str.replace("]","");
        self.sort_id = String::from(work_str);  
    }

    pub fn depth(&mut self) -> i32 {
      if self.sort_id.len() == 0 {
        return 0;
      }
      let mut depth: i32 = 1;
      for c in self.sort_id.chars() {
        if c == ',' {
            depth += 1;
        }
      }
      depth
    }
}

#[test]
fn test_ism_group() {
    let mut ism_group = IsmGroup::new();
    assert!(ism_group.depth() == 0);
    ism_group.set_sort_id("catalog[1].group[01]");
    assert!(ism_group.depth() == 1);

}


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
    pub fn set_sort_id(&mut self, id: &str) {
        let mut work_str = id.replace("catalog[1].", "");
        work_str = work_str.replace("group[", "");
        work_str = work_str.replace("control[", "");
        work_str = work_str.replace(".", ",");
        work_str = work_str.replace("]","");
        self.sort_id = String::from(work_str);  
    }
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
//
// Process an ISM control.
//
fn process_ism_control<'a, 'input>(ism_controls : &mut Vec::<IsmControl>, ism_group : &String, node : &Node<'a, 'input> ) -> serde_json::Result<String> {
    
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
    ism_controls.push(current_control);
    return Ok(result);
}

fn process_ism_group<'a, 'input>(ism_groups : &mut Vec::<IsmGroup>, ism_controls : &mut Vec::<IsmControl>, node : &Node<'a, 'input> ) ->  serde_json::Result<String>  {
    let mut current_group = IsmGroup::new();
    for n in node.children() {
        let tag = n.tag_name().name();
        match tag {
            "group" => {
                process_ism_group(ism_groups, ism_controls, &n)?;
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
                process_ism_control(ism_controls, &current_group.sort_id, &n)?;
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
    ism_groups.push(current_group);

    return Ok(result);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut ism_controls = Vec::<IsmControl>::new();
    let mut ism_groups = Vec::<IsmGroup>::new();

    let mut ism_file = File::open("./data/ISM_catalog.xml")?;

    // Read the file content into a string
    let mut ism_xml = String::new();
    ism_file.read_to_string(&mut ism_xml)?;
    let doc = roxmltree::Document::parse(&ism_xml).unwrap();
    for node in doc.root().descendants().filter(|n| n.is_element() && n.tag_name().name().eq("group")) {
        process_ism_group(&mut ism_groups, &mut ism_controls, &node)?;
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

