use leptos::*;
use leptos::html::Input;
use leptos::ev::SubmitEvent;
use leptos::ev::Event;
use web_sys::MouseEvent;
// use leptos log

use std::time::Duration;

use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsValue;

use std::rc::Rc; 
use std::cell::RefCell;

use web_sys::{EventTarget, File, FileList};
// use web_sys::features::gen_File;

use reqwest::{RequestBuilder, Body};


use web_sys::Window;
use js_sys::Uint8Array;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Doc {
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct NameIDs {
  pub ids: Vec<u32>,
  pub names: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct NameIDsDeserialized {
  pub ids: Vec<u32>,
  pub names: Vec<String>,
}

impl NameIDsDeserialized {
    pub fn new(name_id: NameIDs) -> NameIDsDeserialized {
      NameIDsDeserialized { ids: name_id.ids, names: name_id.names }
    }
}



async fn load_ids() -> NameIDsDeserialized {

  let res =  reqwest::Client::new()
  .get("http://127.0.0.1:8080/api/get/document/all")
  .send()
  .await;

  let val = res.unwrap().json::<NameIDs>().await;



  NameIDsDeserialized::new(val.unwrap())
}
// leptos::log!("where do I run?");

// what we need to do next is to create an interval that ends after 100 seconds.
// create the timer by creating a for loop and and incrementing counter + sleep(1000)
// the position of the div is based on the percentage of completion
// 
async fn load_data(target: Option<FileList>) -> u32 {
  let target = target.expect("file list").get(0).expect("file");
  let text_file = wasm_bindgen_futures::JsFuture::from(target.text()).await;
  
  let arr = text_file.unwrap().as_string().unwrap();
    
        let res =  reqwest::Client::new()
        .post("http://127.0.0.1:8080/api/post/document")
        .body(
          arr
        )
        .send()
        .await;

      let val = res.unwrap().json::<Doc>().await;
      val.unwrap().id
}
    
pub enum ChooseDocument {
  ID(u32),
  File(ReadSignal<Option<FileList>>)
}

pub enum MetricOptions {
  Default,
  Evaluation,
}

pub enum Steps {
  ChooseDocument(ChooseDocument),
  ChooseMetrics(MetricOptions),
  Send(bool),
}







  
#[component]
pub fn AvailableFile(cx: Scope, file_name: String, file_id: String) -> impl IntoView {

  view!{
    cx,
      <figure class="file-div" data-id={file_id.clone()}>
      <p>{file_id}</p>
      <h3>{file_name}</h3>
      </figure>
  }
}

  #[component]
pub fn Step(cx: Scope, step_number: i32, step_title: String) -> impl IntoView {

    view!{
      cx,
        <div class="title-div">
        <h1>
        {
          step_number
        }
        </h1>
        <h2>{step_title}</h2>
        </div>
        // body
    }
}
    
#[component]
pub fn SectionParse(cx: Scope) -> impl IntoView {

  let all_file_ids = 
  create_resource(cx, || (), |_| async move { load_ids().await });



  let inputStyles = "
  z-index: -1;
  position: absolute;
  ";

  let readied = "
    font-size: 2.4rem;
    background-color: rgba(0,0,0, .25);
    padding: 2.4rem;
    color: var(--secondary-1);
  ";

  let title = "
  font-size: 8rem;
  // transform: translateY(-30rem);
  color: var(--secondary-1);
  text-align: center;
  margin-bottom: 9.2rem;
  ";

  /* SEND FILE */
  let (file_added, set_file_added) = create_signal(cx, false);
  let (file_send, send_file) = create_signal(cx, false);
  
  let input_element: NodeRef<Input> = create_node_ref(cx);
  
  let (files, set_files) = create_signal(cx, None);
  
  let file_post: Resource<Option<FileList>, u32> = 
  create_resource(cx,
    files,
    |file_list| 
    async move { 
      load_data(file_list).await });
      
      let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        send_file(true);
        let value = input_element()
        .expect("<input> to exist").files();
      
      set_files(value);
    };
    
    let on_change = move |_: Event | {
      set_file_added(true);
    };
    /* SEND FILE */

    /* GET ID */
    
    let id = move || 
    file_post
    .read(cx)
    .map(|value| format!("http://127.0.0.1:8080/api/get/document/${value}"))
    .unwrap_or_else(|| format!("http://127.0.0.1:8080/api/get/document/0"));
  
  /* END GET ID */

  /* USABILITY  */
  let make_full_screen = move |_| {
    
    
    log!("hello");
  };
  /* USABILITY  */
  /* STEPS */


  
  /* STEPS */

  /* Step 1 */

  // get all available document id's in resource


    

  // on step one, show available documents, or upload your own.
  let get_a_file = move |e: MouseEvent| {
    // dom handling to get file

    log!("get a file");
  };
  /* END Step 1 */

  view! {
    cx,
    <section 
    class="section-parse"
    id="demo"
    >
    <div
    class="container"
    >
    <div>
    <h1 style={title}>"And the Fiesta Begins."</h1>
    <h2 style="color: var(--primary-1); font-size: 6.4rem; text-align: center;">"DEMO SECTION IN WIP"</h2>
    </div>

    <div class="step-container">
      <Step step_number={1}
            step_title={"Choose Any Company".into()}
      />
      <div class="option-names">
        <div><h3>"Choose Saved"</h3></div>
        <div><h3>"Upload file"</h3></div>
      </div>
      <div class="split">
          <div class="all-files" on:click=get_a_file>
          // Select files from database
          // make resource request to get all file options
          // 
          {
            move || match all_file_ids.read(cx) {
              Some(name_ids) => {
                name_ids
                .ids
                .into_iter()
                .enumerate()
                .map(|(i, name_id)| view! { cx,
                  
                  <AvailableFile
                  file_name=String::from(name_ids.names[i].clone())
                  file_id=format!("{name_id}")
                  />
                  
                  
                }).collect::<Vec<_>>()
                .into_view(cx)
              },
              None => {
                view!{cx, <p>"Fetching"</p>}.into_view(cx)
              },
            }
          }
          
          </div>
          <form
          // on:submit=on_submit_file
          >
          
          </form>
      </div>
      <div>
      
      </div>
    </div>


      <form 
      on:submit=on_submit 
      class="flex-center"
      action="http://127.0.0.1:8080/api/document"
      method="POST"
      enctype="multipart/form-data"
      >
      
      
      <button type="button" class="full-screen" on:click=make_full_screen>"x"</button>
      // <div style="position: relative; display: flex; align-items: center;">
      // <div>
      <label
      class="file-label"
      for="upload">
      "Upload file here"
    </label>
      <input type="file"
      style={inputStyles}
      id="upload"
      accept=".htm,.html" 
      class="file-submit"
      value="Upload"
      on:change=on_change

      node_ref=input_element
      />
              <iframe src=id
              class="place-holder"
              class:show=move || file_added() && file_send()
              style="height: 100vh; width: 100%; border: none;"
              >

              </iframe>
                  <button class="cover"
                  class:show=move || file_added() && !file_send()
                  type="submit"
                  >
                    <p 
                    class="send-form"
                    >
                    "Process"
                    </p>
                  </button>
      </form>
    </div>
        </section>
    }
}
    