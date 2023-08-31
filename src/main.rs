use leptos::*;
use leptos_start::components::section_parse::SectionParse;
use leptos_start::components::head_nav::HeadNav;
use leptos_start::components::section_hero::SectionHero;


#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <main>
        <HeadNav/>
        <SectionHero/>
        <SectionParse/>
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


