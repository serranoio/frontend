use leptos::*;

// leptos::log!("where do I run?");

// what we need to do next is to create an interval that ends after 100 seconds.
// create the timer by creating a for loop and and incrementing counter + sleep(1000)
// the position of the div is based on the percentage of completion
// 

#[component]
pub fn Card(cx: Scope, img: String, title: String, subtitle: String, body: String) -> impl IntoView {


    view! {
        cx, 


        <div class="card">

            <div class="image-div">
            <img
            src={img}
            blur=true
            width="50%"
            height="100%"
            quality=85
            class="img"
            />
            </div>

            <div class="text-div">
                <h3 class="title">{title}</h3>
                <p class="subtitle">{subtitle}</p>
                <p class="body">
                    {body}
                </p>
            </div>
                
        
        </div>
    }
}


#[component]
pub fn SectionAbout(cx: Scope) -> impl IntoView {

    let sectionStyles = "
    ";

    let container = "
        max-width: 120rem;
        margin: 0 auto;
        padding: 9.8rem 0;
        height: 100%;
    ";

    let title = "
        font-size: 6.4rem;
        color: var(--secondary-1);
        text-align: center;
        margin-bottom: 6.4rem;
    ";


    let davidBody = String::from("I write poetry.
    I interweave logic and abstraction within every line of code that I write, hoping to form poems
    that can impact millions of people. This project came about 
    because I believe that human potential is absolutely limitless. If I were to dream of something
    realistic, then my potential would be bounded by reality. That is why I must shoot for the stars
    within everything that I do.
    ");


    let racjBody = String::from("
    I'm Patrick, an aspiring accounting student at UIC with a curious mind.
     I have a deep passion for exploring intriguing subjects and uncovering
      new knowledge. Whether it's numbers or fascinating facts, I'm always
       eager to dive in and learn. I look forward to applying my analytical
        skills and dedication to my studies while continuing to explore
         the world of accounting.
    ");

  view! {
    cx,
    
    <section
    style={sectionStyles}
    id="about"
    >
        <div style={container}>
            <h2 style={title}>"About Us"</h2>

            <div class="grid">
                <Card
                    img="../../public/images/david.webp".to_string()
                    title="David Serrano".to_string()
                    subtitle="Chief Engineer / CEO".to_string()
                    body=davidBody
                />

                <Card
                    img="../../public/images/tech-stack.png".to_string()
                    title="Patrick Racjel".to_string()
                    subtitle="CFO / Consultant".to_string()
                    body=racjBody
                />
            </div>
        </div>

    </section>
    }
}
    