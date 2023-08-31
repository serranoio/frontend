use leptos::*;
use leptos::html::Input;
use leptos::ev::SubmitEvent;
use leptos::ev::Event;

use std::{thread, time};

// what we need to do next is to create an interval that ends after 100 seconds.
// create the timer by creating a for loop and and incrementing counter + sleep(1000)
// the position of the div is based on the percentage of completion
// 

#[component]
pub fn SectionParse(cx: Scope) -> impl IntoView {



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



  let (count, increment_count) = create_signal(cx, 0);


  let (processing_status, set_processing_status) = create_signal(cx, "First");
  let (file_added, set_file_added) = create_signal(cx, false);
  let (file_send, send_file) = create_signal(cx, false);




  let input_element: NodeRef<Input> = create_node_ref(cx);

  let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();
    send_file(true);
};

let on_change = move |ev: Event | {
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

create_effect(cx, move |_| {

  if file_send() {
    set_processing_status("Uploading");

    
    if count() < 100 {
      thread::sleep(time::Duration::from_millis(1000));
      increment_count(count() + 1); 
    }
  }

});


    view! {
        cx,
        <section style="
        background: var(--primary-4);
        background: linear-gradient(to top ,var(--primary-4), var(--primary-4));
        ">
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
                    style=format!("
                    position: absolute;
                    left: {};
                    background-color: white;
                    width: 20%;
                    height: 100%;
                    
                    ", count())>

                    </div>

                    <button type="submit"
                    style={submitStyles}
                    class:show=move || file_added() && !file_send()
                    class="send-form"
                    >
                    ">>"
                    </button>
                    <p style={parseStatusStyles}>{processing_status}{count()}</p>
                  </div>

                  <p style={readied}>{"end"}</p>
            </div>
          </form>
        </div>
        </section>
    }
}
    