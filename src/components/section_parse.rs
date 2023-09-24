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
// leptos::log!("where do I run?");

// what we need to do next is to create an interval that ends after 100 seconds.
// create the timer by creating a for loop and and incrementing counter + sleep(1000)
// the position of the div is based on the percentage of completion
// 
async fn load_data(target: Option<FileList>) {

  let target = target.expect("file list").get(0).expect("file");

  // log!("{:?}", target);

  let text_file = wasm_bindgen_futures::JsFuture::from(target.text()).await;

  let arr = text_file.unwrap().as_string().unwrap();

        
        let res =  reqwest::Client::new()
        .post("http://127.0.0.1:8080/api/document")
        .body(
          arr
        )
        .send()
        .await;
      
      
      
      
      
    }
    
    
    #[component]
    pub fn SectionParse(cx: Scope) -> impl IntoView {

  let randomWords = vec!["Tokenizing...", "Parsing...", "Distributing...", "React is tech debt", "Creating...", "Hydrating...", "Finalizing...", "Done!"];

  let inputStyles = "
  z-index: -1;
  position: absolute;
  ";

  let submitStyles="
   
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

  let (processing_status, set_processing_status) = create_signal(cx, "");
  let (file_added, set_file_added) = create_signal(cx, false);
  let (file_send, send_file) = create_signal(cx, false);

  let input_element: NodeRef<Input> = create_node_ref(cx);
  
  let (files, set_files) = create_signal(cx, None);
  


  let once = 
  create_resource(cx,
   files,
   |file_list| 
  async move { 
    load_data(file_list).await });
  
  
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

  let parseStatusStyles = "
  position: absolute;
  bottom: 0%;
  left: 50%;
  color: var(--secondary-1);
  transform: translate(-50%, 105%);
  font-size: 2.4rem;
  ";

  let (count, increment_count) = create_signal(cx, 0);

  let (turn_off, set_turn_off) = create_signal(cx, true);

  let display = move || randomWords[count.get() / 14];


  // let mut timerHandle = RefCell::new(Err(JsValue::null()));
  let mut timerHandle = leptos::set_interval_with_handle(move || {
     increment_count(count.get()+1);
   }, Duration::from_millis(100));

  create_effect(cx, move |_| {
    if file_send() && turn_off() {
      set_processing_status("Uploading");

      set_turn_off(false);
    }
    
    // log!("{:?}", timerHandle);
    if count() == 100 {    
      match &timerHandle {
        Ok(handle) => handle.clear(),
        Err(val) => log!("wtf {val:?}"),
      }
    }

  });

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

    <div
    class="place-holder"
    class:show=move || file_added() && file_send()
    >
    <p class="text">
    "iFrame goes here on send"
    </p>
    </div>
                <button class="cover"
                class:show=move || file_added() && !file_send()
                type="submit"
                >
                  <p 
                  style={submitStyles}
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
    </div>
        </section>
    }
}
    