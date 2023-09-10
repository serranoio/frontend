use leptos::{*, html::Nav, ev::Event};



pub struct NavSection {
    pub name: String,
    pub section: String,
    pub class: String,
    pub style: String,

}

impl NavSection {

    pub fn new(name: &str, section: &str, class: &str, style: String) -> NavSection {
        NavSection {
            name: name.to_string(),
            section: section.to_string(),
            class: class.to_string(),
            style
        }
    } 
}

#[component]
pub fn HeadNav(cx: Scope) -> impl IntoView {
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
    text-decoration: none;
    ";
    
    
    let dominantButtonStyles = "
    border-radius: 10px;
    padding: 1.2rem 4.4rem;
    ";
    
    let navElements = vec![
        NavSection::new("Online Demo", "demo", "hero", buttonStyles.to_string() + dominantButtonStyles),
        NavSection::new("What We Do", "do", "nav", buttonStyles.to_string()),
        NavSection::new("Tech Stack", "tech", "nav", buttonStyles.to_string()),
        NavSection::new("About Us", "about", "nav", buttonStyles.to_string())
        
        
         ];
        
    
    let navigate = move |_| {
        
        
        log!("hello");
    };
    
    view! { cx,
        <nav
        style={navStyles}
        >
        <h3>"Accential"</h3>
        <ul style={ulStyles}>
        {
            {navElements.into_iter()
                .map(|tab| view! { cx,
                    <li>
                    <a
                      href="#".to_owned() + &tab.section
                      class=tab.class
                      on:click=navigate
                      style={tab.style}>
                      {tab.name}
                     </a>
                     </li>
            })
                .collect::<Vec<_>>()}
            }
        </ul>
        </nav>
    }
}
