use leptos::*;

// leptos::log!("where do I run?");

// what we need to do next is to create an interval that ends after 100 seconds.
// create the timer by creating a for loop and and incrementing counter + sleep(1000)
// the position of the div is based on the percentage of completion
// 

#[component]
pub fn SectionDo(cx: Scope) -> impl IntoView {

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
        color: var(--secondary-1);
        text-align: center;
        margin-bottom: 6.4rem;
    ";

    let grid = "
        display: grid;
        grid-template-columns: .5fr 1fr;
        align-items: center;
        justify-content: center;
        align-content: center;
        justify-content: center;
        height: 100%;
        column-gap: 2.4rem;
        row-gap: 2.4rem;
        ";

    let textBox = "
        border-right: 2px solid var(--tertiary-4);
        font-size: 4.4rem;
        color: var(--secondary-1);

        width: 100%;
        height: 100%;
        padding: 6.4rem 0;
        ";

        let visualBox = "
        // background-color: ;
        box-shadow: 0 0 2px 3px rgba(206, 53, 0, 0.3);
        border-radius: 10px;
        width: 100%;
        height: 100%;
        padding: 0 0 0 2.4rem;
    ";

    let highlight = "
    background-image: linear-gradient(to right, #CE6100 0%, #CE3500 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    ";
  view! {
    cx,
    
    <section
    style={sectionStyles}
    id="do"
    >
    <div style={container}>
    <h2 style={title}>"What we do"</h2>

    <div style={grid}
    >

    <div style={textBox}>
        <p>"Distill your financial Statement ino the" <span style={highlight}>" highlights "</span>"for your stakeholders"</p>
    </div>

    <div style={visualBox}>
    <p></p>
    </div>
    
    <div style={textBox}>
    <p>"View the highlights in an" <span style={highlight}>" interactive "</span> "document!"</p>
    </div>
    
    <div style={visualBox}>
    <p>"i"</p>
    </div>
    </div>
    </div>
    </section>
    }
}
    