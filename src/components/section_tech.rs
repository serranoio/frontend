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
        out on top to be the gold standard of websites. Rust is able to target WASM with extremely small binaries,
        allowing for fast performance. Unfortunately, the ecosystem is young. If we were to take this public,
        we would have to switch over to TypeScript.
        Good thing I am a solo developer ;). (I'm keeping it in Rust + Leptos). It is a lot harder to create a progressive
         web app using Rust, but that's why for one of my solo projects, I wish to create my own frontend Rust framework.
         https://github.com/serranoio/pinnacle.

         The frontend is completely static - so I'm hosting it on Netlify.
       ".to_string());

       let backend = TechStack::Backend(
        "There was no way that I could make my backend in Rust haha. 
        Also, RabbitMQ does not provide official SDK's in Rust.
        The language of choice is Golang, which is currently dominating the microservice industry
        as well as the cloud. All of the appropriate tooling is built in Golang, so I might as well make this
        in Golang. We use RabbitMQ to distribute the task of parsing, tokenizing, converting, and building into
        individual microservices. The benefit of RabbitMQ is that I am going to create pipelined concurrency. Instead
        of having the entire document parsed, then tokenized, etc., as soon as parts of the document is parsed, it will be
        sent to the tokenizer. As soon as some parts are tokenized, it will be sent to the converter, etc. This will create
        a blazingly fast architecture. I will be hosting the backend on fly.io in a Docker container.

        We will use a small sqlite3 file for the database.
        We will use unidoc for PDF parsing (github.com/unidoc/unipdf).
        ".to_string()
       );

       let document = TechStack::Document(
        "In the final step, we will generate an html-report from the gathered data. To do so,
         we will use Lit Element for reactivity,
        (a lightweight framework using web components)
        and then hydrate an HTML document with TypeScript. We will use Bun to do everything ;)."
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
            <div class="white-golang"></div>
            <div class="white-rust"></div>
                <img
                src="../../public/images/tech-stack.png"
                blur=true

                width=1100
                height=700
                quality=85
                class="img"
                />
                <img
                src="../../public/images/bun.png"
                width=100
                height=100
                quality=85
                class="bun"
                />

            </div>
        </div>
    </section>
    }
}
    