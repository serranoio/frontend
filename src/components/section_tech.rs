use leptos::{*, ev::{Event, MouseEvent}};


#[component]
pub fn SectionTech(cx: Scope) -> impl IntoView {
  
    

    

    let sectionStyles = "
     padding-top: 9.8rem;
    ";

    let container = "
        max-width: 130rem;
        margin: 0 auto;
        padding: 9.8rem 0;
        height: 100%;
    ";

    let title = "
        font-size: 6.2rem;
        color: var(--primary-1);
        text-align: center;
        margin-top: 9rem;
    ";

    let center = "
    display: flex;
    justify-content: center;

    ";

    let tabP = "
        font-size: 2.4rem;
        color: var(--primary-1);
        text-align: center;
        line-height: 1.5;
        height: 10rem;
        z-index: 90;
    ";

    let tabSection = "
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 7.2rem;
    ";

  
    // let on_tab_click = move |event: MouseEvent| {


    //    log!("{}", event_target(leptos::ev::Event::from(&event)));
      
    // };

  view! {
    cx,
    <section
    style={sectionStyles}
    id="tech"
    >
    <h2 style={title}>"Introducing..."</h2>
        <div style={container.to_string() + "display: flex; justify-content: center;"}>    

           <a 
           href="https://www.youtube.com/watch?v=aE7Saw6qgWA"
           style="font-size: 2.4rem;"
           >"Link to video!"</a>
        </div>
    </section>
    }
}
    