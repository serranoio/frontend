use leptos::*;

#[component]
pub fn Logo(cx: Scope) -> impl IntoView {
    
    let logoMain = "
        // background-color: var(--primary-4);
        background-image: linear-gradient(to right, var(--secondary-3), var(--primary-3));
        width: 450px;
        opacity: .75;
        height: 10px;
        transform: rotate(45deg) translateY(-50px);
        transform-origin: center;
        z-index: 0;
        ";
        
        let logoLine = "
        transform: rotate(45deg) translate(-35px, -25px);
        ";
        
        let upsideDown = "
        transform: rotate(315deg) translate(-155px, 205px);
        ";
    
        let upsideDownOver = "
        transform: rotate(315deg) translate(-180px, 170px);
        ";
    
    let logoBox = "
        display: flex;
        align-items: center;
        justify-content: center;
        transform: translate(100px, -175px);
        z-index: 0;
        position: relative;
    ";
    
    
    let logo2 = "
    position: absolute;
    z-index: 0;
    transform: translate(-650px, -100px) scale(1.5);
    ";
    
    let logo1 = "
     transform: translate(-450px, -50px);
    ";
    
    view! {
        cx, 
       <div style={logoBox}>
                <div style={logo1}>
                    <div style={logoMain}>
                    </div>
                    <div style={logoMain.to_string() + logoLine}>
                    </div>
    
                    <div style={logoMain.to_string() + upsideDown}>
                    </div>
    
                    <div style={logoMain.to_string() + upsideDownOver}>
                    </div>
                </div>
                <div style={logo2}>
                    <div style={logoMain}>
                    </div>
                    <div style={logoMain.to_string() + logoLine}>
                    </div>
    
                    <div style={logoMain.to_string() + upsideDown}>
                    </div>
    
                    <div style={logoMain.to_string() + upsideDownOver}>
                    </div>
                </div>
            </div>
    }
    }
    