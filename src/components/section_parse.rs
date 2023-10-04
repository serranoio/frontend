use leptos::*;
use leptos::html::Input;
use leptos::ev::SubmitEvent;
use leptos::ev::Event;
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

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Doc {
    pub id: u32,
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
    
    
    #[component]
    pub fn SectionParse(cx: Scope) -> impl IntoView {

  let randomWords = vec!["Tokenizing...", "Parsing...", "Distributing...", "React is tech debt", "Creating...", "Hydrating...", "Finalizing...", "Done!"];

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


    // let documentID = create_resource(
    //   cx, 
    //   once.read(cx)
    //   , async move {

    //     once.read(cx)
    //   });
  
  let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();
    send_file(true);

    // log!("{:?}",ev)

    let value = input_element()
    // event handlers can only fire after the view
    // is mounted to the DOM, so the `NodeRef` will be `Some`
    .expect("<input> to exist").files();

    set_files(value);
  };

  let on_change = move |_: Event | {
    set_file_added(true);
  };


  let id = move || 
    file_post
    .read(cx)
    .map(|value| format!("http://127.0.0.1:8080/api/get/document/${value}"))
    .unwrap_or_else(|| format!("http://127.0.0.1:8080/api/get/document/0"));


  let make_full_screen = move |_| {

    log!("hello");
  };
  // let id_value = move || id();

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
    //               </div>

    //               <div class="file-dispatcher">
    //                 <div class="file"
    //                 style={"
    //                 position: absolute;
    //                 background-color: white;
    //                 width: 20%;
    //                 height: 100%;
    //                 transition: all .2s;
                    
    //                 "}
    //                 style:left=move|| format!("{}%", count())
                    
    //                 >

    //                 </div>
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
    //                 <p style={parseStatusStyles}>{display}</p>
    //               </div>

    //               <p style={readied}>"end"</p>
    //         </div>
    </form>
    // <p>hey</p>


    </div>
        </section>
    }
}
    