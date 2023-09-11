use leptos::*;
use crate::components::logo::Logo;

#[component]
pub fn SectionHero(cx: Scope) -> impl IntoView {
    
    let baseSectionStyles = "
    max-width: 130rem;
    margin: 0 auto;
    ";
    let sectionStyles = "
        display: grid;
        grid-template-columns: 4fr 4fr;
        height: calc(100vh - 80px);
        // width: 100vw;
        position: relative;
    ";
    
    let heroTitleStyles = "
        color: var(--primary-1);
        font-size: 9.2rem;
        z-index: 10;
    ";

let flex = "
    padding: 9.8rem 0 0 0;
    z-index: 1;
";


let strong = "
    font-weight: 600;
    letter-spacing: 2px;
    text-transform: uppercase;
";

let demo = "
    position: absolute;
    left: 50%;

    font-size: 4.6rem;
    transform: translateX(-50%);
    bottom: 10%;
    color: var(--primary-1);
    font-weight: 400;
    cursor: pointer;
    text-decoration: none;
    z-index: 999;
";

    view! { cx,
        <section style={sectionStyles.to_string() + baseSectionStyles}>
            <a href="#demo" style={demo} class="arrow-down">
                <p>"Try Online Demo!"</p>
            </a>
        <div style={flex}>
        <h1 style={heroTitleStyles} class="hero-title">
        "Convert your Financials to an Accounting Statement in"
         <strong
        style={strong}
        class="seconds"
         >" Seconds."</strong>
        </h1>
        </div>
        
        <Logo/>
            </section>
    }
}

