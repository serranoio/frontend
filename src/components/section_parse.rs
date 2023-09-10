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
// leptos::log!("where do I run?");

// what we need to do next is to create an interval that ends after 100 seconds.
// create the timer by creating a for loop and and incrementing counter + sleep(1000)
// the position of the div is based on the percentage of completion
// 

#[component]
pub fn SectionParse(cx: Scope) -> impl IntoView {


  let randomWords = vec!["Tokenizing...", "Parsing...", "Distributing...", "React is tech debt", "Creating...", "Hydrating...", "Finalizing...", "Done!"];

  let inputStyles = "
  z-index: -1;
  position: absolute;
  ";

  let submitStyles="
   
  ";

  let fileLabel="
    font-size: 2.4rem;
    color: var(--primary-1);
    background-color: rgba(0,0,0,.25);
    padding: 2.4rem 1.2rem;
    cursor: pointer;
  ";

  let readied = "
    font-size: 2.4rem;
    background-color: rgba(0,0,0, .25);
    padding: 2.4rem;
    color: var(--primary-1);
  ";

  let title = "
  font-size: 8rem;
  transform: translateY(-30rem);
  color: var(--secondary-1);
  ";

  let (processing_status, set_processing_status) = create_signal(cx, "");
  let (file_added, set_file_added) = create_signal(cx, false);
  let (file_send, send_file) = create_signal(cx, false);

  let input_element: NodeRef<Input> = create_node_ref(cx);
  


  let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();
    send_file(true);
  };

  let on_change = move |_: Event | {
    set_file_added(true);
  };

  let parseStatusStyles = "
  position: absolute;
  bottom: 0%;
  left: 50%;
  color: var(--primary-1);
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
    <section style="
    background: linear-gradient(to top ,var(--black), var(--secondary-3));
    // background: var(--black);
    "
    id="demo"
    >
    <div style="
    max-width: 120rem;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    margin: 0 auto;
    ">
    <form on:submit=on_submit style="display: flex; align-items: center; flex-direction: column;">
    <h1 style={title}>"And the Fiesta Begins."</h1>
    <div style="position: relative; display: flex; align-items: center;">
    <div>
    <label style={fileLabel} for="upload">"Upload file here"</label>
    <input type="file"
    on:change=on_change
    style={inputStyles}
    id="upload"
                  accept=".pdf" 
                  class="file-submit"
                  node_ref=input_element
                  />
                  </div>

                  <div class="file-dispatcher">
                    <div class="file"
                    style={"
                    position: absolute;
                    background-color: white;
                    width: 20%;
                    height: 100%;
                    transition: all .2s;
                    
                    "}
                    style:left=move|| format!("{}%", count())
                    
                    >

                    </div>

                    <button type="submit"
                    style={submitStyles}
                    class:show=move || file_added() && !file_send()
                    class="send-form"
                    >
                    ">>"
                    </button>
                    <p style={parseStatusStyles}>{display}</p>
                  </div>

                  <p style={readied}>"end"</p>
            </div>
          </form>
        </div>
        </section>
    }
}
    