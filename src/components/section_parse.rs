use leptos::ev::Event;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use web_sys::MouseEvent;
// use leptos log

use std::time::Duration;

use gloo_timers::future::TimeoutFuture;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use web_sys::{EventTarget, File, FileList};
// use web_sys::features::gen_File;

use reqwest::{Body, RequestBuilder};

use web_sys::HtmlElement;

use js_sys::Uint8Array;
use web_sys::Window;

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
        NameIDsDeserialized {
            ids: name_id.ids,
            names: name_id.names,
        }
    }

    pub fn add(&mut self, id: u32, name: String) {
        self.ids.push(id);
        self.names.push(name);
    }
}


#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct FileAndMetric {
    pub file: String,
    pub metric: String,
}

async fn load_metric_tags() -> NameIDsDeserialized {
    let res = reqwest::Client::new()
        .get("http://127.0.0.1:8080/api/get/metrics/all")
        .send()
        .await;

    let val = res.unwrap().json::<NameIDs>().await;

    let mut name_ids = NameIDsDeserialized::new(val.unwrap());
    name_ids.add(0, "Fresh Start".into());

    name_ids
}

async fn load_ids() -> NameIDsDeserialized {
    let res = reqwest::Client::new()
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
async fn load_data(target: ChooseDocument, tag_id: &ReadSignal<String>) -> u32 {
    match target {
        ChooseDocument::File(handle) => {

            let target = handle.expect("file list").get(0).expect("file");
            let text_file = wasm_bindgen_futures::JsFuture::from(target.text()).await;

            let arr = text_file.unwrap().as_string().unwrap();

            let file_and_metric = FileAndMetric{ file: arr, metric: tag_id.get()};

            let json_bytes = serde_json::to_vec(&file_and_metric).unwrap();

            let res = reqwest::Client::new()
                .post("http://127.0.0.1:8080/api/post/document")
                .body(json_bytes)
                .send()
                .await;

            log!("{:?}", res);

            let val = res.unwrap().json::<Doc>().await;

            val.unwrap().id
        }
        ChooseDocument::ID(handle) => {

            let file_and_metric = FileAndMetric{file: handle, metric: tag_id.get() };

            let json_bytes = serde_json::to_vec(&file_and_metric).unwrap();

            let res = reqwest::Client::new()
            .post("http://127.0.0.1:8080/api/post/document/add-metrics")
            .body(json_bytes)
            .send()
            .await;

            let val = res.unwrap().json::<Doc>().await;
            val.unwrap().id
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ChooseDocument {
    ID(String),
    File(Option<FileList>),
}

pub enum Steps {
    ChooseDocument(ChooseDocument),
    ChooseMetrics,
    Send(bool),
}

pub fn getChosen(val: String, chosen: String) -> String {
    if val == chosen {
        "chosen".into()
    } else {
        "".into()
    }
}

#[component]
pub fn Tag(cx: Scope, metric_tag: String, metric_tag_id: String, chosen: String) -> impl IntoView {
    let chosen = getChosen(metric_tag.clone(), chosen);

    view! {
      cx,
        <figure
        class=format!("tag-div {chosen}")
         id={metric_tag.clone()}>
        <h3
        id={metric_tag.clone()}>{metric_tag}</h3>
        </figure>
    }
}

#[component]
pub fn AvailableFile(
    cx: Scope,
    file_name: String,
    file_id: String,
    chosen: String,
) -> impl IntoView {
    let chosen = getChosen(file_id.clone(), chosen);

    view! {
      cx,
        <figure
         class=format!("file-div {chosen}")
         id={file_id.clone()}>
        <p id={file_id.clone()}>{file_id.clone()}</p>
        <h3  id={file_id}>{file_name}</h3>
        </figure>
    }
}

#[component]
pub fn Step(
    cx: Scope,
    step_number: i32,
    step_title: String,
    step_subtitle: String,
) -> impl IntoView {
    view! {
      cx,
        <div class="title-div">
        <h1>
        {
          step_number
        }
        </h1>
        <h2>{step_title}</h2>
        <p>{step_subtitle}</p>
        </div>
        // body
    }
}

#[component]
pub fn SectionParse(cx: Scope) -> impl IntoView {
    let all_file_ids = create_resource(cx, || (), |_| async move { load_ids().await });

    let all_metric_tags = create_resource(cx, || (), |_| async move { load_metric_tags().await });

    let inputStyles = "
  z-index: -1;
  position: absolute;
  display: none;
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

    /* END GET ID */

    
    /* USABILITY  */
    let (is_full_screen, set_is_full_screen) = create_signal(cx, true);
    
    let make_full_screen = move |_| {
        set_is_full_screen(!is_full_screen())
    };
    /* USABILITY  */
    /* STEPS */

    /* STEPS */

    /* Step 1 */

    // get all available document id's in resource
    let (is_id, set_is_id) = create_signal(cx, true);
    let (get_id, set_id) = create_signal(cx, String::from(""));

    // on step one, show available documents, or upload your own.
    let get_a_file = move |e: MouseEvent| {
        // dom handling to get file
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
        set_id(input.unwrap().id().into());
        set_is_id(true);
    };

    let input_element: NodeRef<Input> = create_node_ref(cx);
    // file list
    let (files, set_files) = create_signal(cx, None);
    // step 1 still
    let on_change = move |_: Event| {
        let value = input_element().expect("<input> to exist").files();
        set_files(value);
        set_is_id(false);
    };

     /* STEP 2. */
     let (tag_id, set_tag_id) = create_signal(cx, String::from("Fresh Start"));

     let get_a_tag = move |e: MouseEvent| {
         let target = e.target();
         let input = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
 
         set_tag_id(input.unwrap().id().into());
     };
 
     /* END OF STEP 2 */

     
    /* STEP 3. */
    // CHOSEN
    let (choose_document, set_choose_document) = create_signal(cx, ChooseDocument::ID("".into()));
    /* Step 3 */

    let (submitted, set_submitted) = create_signal(cx, false);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if is_id.get() {
            set_choose_document(ChooseDocument::ID(get_id.get()));
        } else {
            set_choose_document(ChooseDocument::File(files.get()));
        }

        set_files(None);
        set_id("".into());

        set_submitted(true);
    };

    // if we submit event... so this is step 3
    let file_post = 
    create_resource(cx, choose_document, move |file_list| async move {
        load_data(file_list, &tag_id).await
    });
    /* END Step 1 */

   
    /* SEND FILE */

    /* GET ID */

    /* SEND FILE */

    // implementation
    let id = move || {
        file_post
            .read(cx)
            .map(|value| format!("http://127.0.0.1:8080/api/get/document/${value}"))
            .unwrap_or_else(|| format!("http://127.0.0.1:8080/api/get/document/0"))
    };

    view! {
        cx,
        <section
        class="section-parse"
        id="demo"
        >
        <div
        class="container"
        >
        // STEP 1
        <div class="step-container">
          <Step step_number={1}
                step_title={"Choose Any Company".into()}
                step_subtitle="choose their 10k, 8k, 500k, 401k, or whatever you want".into()
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
                      chosen=get_id()
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
              class="file-upload-form"
              >
              <label
              class="file-label"
              for="upload">

              <img
                  src="../../public/assets/upload-simple.svg"
                  width="100%"
                  height="100%"
                  class="upload-svg"
                  />
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
              </form>
          </div>


        </div>
        // STEP 2
        <div class="step-container">
        <Step step_number={2}
              step_title={"Choose Your Metrics".into()}
              step_subtitle="Grade your document based on any set of metrics!".into()
        />
            <div class="all-metric-tags" on:click=get_a_tag>
              {
                move || match all_metric_tags.read(cx) {
                  Some(found_tags) => {
                    found_tags
                    .names
                    .into_iter()
                    .enumerate()
                    .map(|(i, tag)| view!{cx,
                        <Tag
                          metric_tag=format!("{}", tag.clone())
                          metric_tag_id=format!("{}",found_tags.ids[i].clone())
                          chosen=tag_id()
                          />

                        }).collect::<Vec<_>>().into_view(cx)

                  },
                  None => {
                    view!{cx, "Fetching"}.into_view(cx)
                  },
                }
              }
            </div>

      </div>

      <div class="step-container">
      <Step step_number={3}
            step_title={"Process Document".into()}
            step_subtitle="oogly boogly dont get too excitedly...".into()
      />
          // STEP 3

            <form
            on:submit=on_submit
            class="flex-center"
            class:make-full-screen=move||is_full_screen()
            >
            <button type="button"
            class="full-screen"
            class:remove=move || !submitted()
            on:click=make_full_screen>
            </button>
            <iframe src=id
            class="place-holder"
            class:show=move || get_id() != "" || files() == None
            style="height: 100vh; width: 100%; border: none;"
            >

            </iframe>

            <button class="cover"
            class:show=move || get_id() != "" || files() != None
            type="submit"
            >
              <p
              class="send-form"
              >
              "Let's Party"
              </p>
            </button>
            </form>
            </div>
      </div>
    </section>
        }
}
