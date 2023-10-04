use leptos::{*, ev::{Event, MouseEvent}};

// leptos::log!("where do I run?");

// what we need to do next is to create an interval that ends after 100 seconds.
// create the timer by creating a for loop and and incrementing counter + sleep(1000)
// the position of the div is based on the percentage of completion
// 

#[derive(Clone)]
enum TechStack {
    Frontend(String),
    Backend(String),
    Document(String)
}

impl TechStack {
    fn read_content(&self) -> String {
        match self {
            Self::Frontend(string) => string.to_owned(),
            Self::Document(string) => string.to_owned(),
            Self::Backend(string) => string.to_owned(),
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Frontend(_) => "Frontend".to_string(),
            Self::Document(_) => "Document".to_string(),
            Self::Backend(_) => "Backend".to_string(),
        }
    }

    fn get_img(&self) -> String {
        match self {
            Self::Frontend(_) => "../../public/images/frontend.png".to_string(),
            Self::Document(_) => "../../public/images/document.png".to_string(),
            Self::Backend(_) => "../../public/images/backend.png".to_string(),
        }
    }


}

#[component]
pub fn SectionTech(cx: Scope) -> impl IntoView {
    let frontend = TechStack::Frontend("The
     frontend consists of Rust and Leptos.
      Leptos creates reactivity using signals
       in the same way that SolidJS does. 
       Leptos then compiles down my code into a wasm binary,
        which is then shipped to the browser. Although WASM is fast,
         it is still not as fast as SolidJS or vanillaJS. It's a baby.
        Why Rust? Because in the wake of the frontend framework war, I believe that WASM will rise 
        out on top to be the gold standard of websites (In like 5-10 years). Rust is able to target WASM with extremely small binaries,
        allowing for fast performance. Unfortunately, the ecosystem is young, so it is hard to create progressive web apps. So when I do
        get to adding indexxedDB and web workers (if I find a use case for web workers),
        I think I will have to add them in the JavaScript file that injects the WASM.
         The frontend is completely static - so I'm hosting it on Netlify.
       ".to_string());

       let backend = TechStack::Backend(
        "There was no way that we could make the backend in Rust haha. 
        The language of choice is Golang, which is currently dominating the backend industry
        as well as the cloud. All of the appropriate tooling is built in Golang, so we might as well make this
        in Golang.

        We will be creating a distributed monolith, where each part of the backend is heavily modularized.
        An HTML doc sent to the backend will go to the Tabulizer, where every table is parsed in the HTML document. Next,
        the tables will go to the 'financizer' (we made that word up), where we will create metrics from the tabular data.
        This financial data will then go to the html report module and be converted into a fully functional html report. 
        (More about it in the document section) This reactive html document is then sent back as the response on api/document.
  
        The backend api service will use Gin to create the api endpoints.

        We will use pocketbase, hosted on the backend. We will perhaps save metrics, or we will save the already parsed files.

        We will be hosting the backend on fly.io.
        ".to_string()
       );

       let document = TechStack::Document(
        "In the final step, we will generate an html-report from the gathered data. To do so,
        we will use Lit Element for reactivity with Typescript (a lightweight framework using web components).
        We will use Bun to do everything. This document allows you to create metrics manually, or from the 10-k file. You will
        be able to hover over it and select the metric directly from the document.
         This document is then sent to the 
        frontend and hosted in an iFrame."
        .to_string());

        let tabs = vec![frontend.clone(), backend.clone(), document.clone()];


    

    let sectionStyles = "
     
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
        margin-bottom: 18rem;
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
        margin-bottom: 8rem;
    ";

    let (selected_tab, set_tab) = create_signal(cx, frontend.clone());


    // let on_tab_click = move |event: MouseEvent| {


    //    log!("{}", event_target(leptos::ev::Event::from(&event)));
      
    // };

  view! {
    cx,
    <section
    style={sectionStyles}
    id="tech"
    >
        <div style={container}>
        <h3 style={title.to_string() + " font-size: 6.4rem;"}>"MVP"</h3>
        <p style={tabP}>"We aim to be able to create at least ONE metric from the financial statement that is evaluated and viewable from an interactive HTML document"</p>
            <h2 style={title}>"The Tech Stack"</h2>
            <div
            style={tabSection}
            >
            {tabs
            .into_iter()
            .map(|t| 
                {
                let closure = t.clone();
                let closure2 = t.clone();
                
                let backend = backend.clone();
                let frontend = frontend.clone();
                let document = document.clone();

                view !{ cx,
                        
                    <button
                    data-tab={t.name().to_string()}
                    class="tab"
                    class=("selected", move || closure2.name().to_string() == selected_tab.get().name())
                
                    on:click={move |_: MouseEvent| {

                        let name = closure.name();

                        if name == "Backend" {
                            set_tab(backend.clone())
                        } else if name == "Frontend" {
                            set_tab(frontend.clone())
                        } else if name == "Document" {
                            set_tab(document.clone())
                        } else {

                        }
                    }}>
                    {t.clone().name()}
                    </button>
                    }
            })
        .collect::<Vec<_>>()
        
        }
            </div>

            <p style={tabP}>{move || selected_tab.get().read_content()}</p>
     
            <div class="center" style={center}>
            <img
            src={move || selected_tab.get().get_img()}
            width=800
            height=500
            class="img move-down"
            />
            // <div class="white-golang"></div>
            // <div class="white-rust"></div>
            //     <img
            //     src="../../public/images/tech-stack.png"
            //     blur=true

            //     width=1100
            //     height=700
            //     quality=85
            //     class="img"
            //     />
            //     <img
            //     src="../../public/images/bun.png"
            //     width=100
            //     height=100
            //     quality=85
            //     class="bun"
            //     />

            </div>
        </div>
    </section>
    }
}
    