use leptos::*;

#[component]
pub fn HeadNav(cx: Scope) -> impl IntoView {
    let navElements = vec![ "About Us", "What We Do", "Contact Us"];

    let flex = "
    display: flex;
    align-items: center;
    justify-content: space-between;
    ";

    let navStyles = "
    padding: 2.4rem 2.4rem;
    font-size: 2.4rem;
    z-index: 10;
    color: var(--primary-1);
    position: relative;
    ".to_string() + flex.clone();
    
    let ulStyles = "
    gap: 3.6rem;
    list-style: none;
    ".to_string() + flex.clone();
    
    
    let buttonStyles: &str = "
        color: var(--primary-1);
        border: none;
        background-color: transparent;
        font-size: 2.4rem;
        cursor: pointer;
        position: relative;
    ";

    
    let dominantButtonStyles = "
        border-radius: 10px;
        padding: 1.2rem 4.4rem;
    ";



    view! { cx,
        <nav
        style={navStyles}
        >
        <h3>"Accential"</h3>
        <ul style={ulStyles}>

        <li><button class="hero"
         style={buttonStyles.to_string() + dominantButtonStyles}>
        "Online Demo" <span>" >>"</span>
        </button></li>
        {
            {navElements.into_iter()
                .map(|n| view! { cx, <li><button class="nav" style={buttonStyles}>{n}</button></li>})
                .collect::<Vec<_>>()}
            }
        </ul>
        </nav>
    }
}
