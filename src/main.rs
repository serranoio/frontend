use leptos::*;
use leptos_start::components::section_parse::SectionParse;
use leptos_start::components::head_nav::HeadNav;
use leptos_start::components::section_hero::SectionHero;
use leptos_start::components::section_do::SectionDo;
use leptos_start::components::section_tech::SectionTech;
use leptos_start::components::section_about::SectionAbout;
#[component]
fn App(cx: Scope) -> impl IntoView {


    let colorStyles = "
    
    background-color: var(--black);
    // background: radial-gradient(ellipse 300% 300% at 50% 50%,rgba(206, 53, 0, .9), rgba(0,0,0, 1));
    // -webkit-mask-image: radial-gradient(circle at 50% 50%, rgba(0,0,0,.5), black 10%);
    -webkit-mask-image: radial-gradient(ellipse 200% 200% at 50% 50%,rgba(205, 53, 0, 0.7), rgba(0,0,0,1) 33.5%)
    ";

    view! { cx,
        <main>
        <HeadNav/>
        <SectionHero/>
        <SectionParse/>
        <div style={colorStyles}>
        <SectionDo/>
        <SectionTech/>
        <SectionAbout/>
        </div>
        </main>
    }
}



#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
    mount_to_body(|cx| view! { cx,  <App/>  });

}


