use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Title text="Leptos Static Website"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        
        <Router>
            <div class="container">
                <nav>
                    <A href="/" exact=true>"Home"</A>
                    <A href="/about">"About"</A>
                    <A href="/contact">"Contact"</A>
                </nav>
                
                <main>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/about" view=AboutPage/>
                        <Route path="/contact" view=ContactPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div>
            <h1>"Welcome to Our Leptos Website!"</h1>
            <p>"This is a simple static website built with Leptos and Rust."</p>
            
            <div class="counter">
                <h2>"Fantastic Interactive Counter"</h2>
                <div class="count">{count}</div>
                <div>
                    <button on:click=move |_| set_count.update(|n| *n -= 1)>
                        "Decrease"
                    </button>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>
                        "Increase"
                    </button>
                </div>
                <button on:click=move |_| set_count.set(0)>
                    "Reset"
                </button>
            </div>
            
            <p>"Click the buttons above to interact with the counter, or explore other pages using the navigation."</p>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <h1>"About This Website"</h1>
            <p>"This website demonstrates the power of Leptos - a modern web framework for Rust."</p>
            
            <h2>"Features:"</h2>
            <ul>
                <li>"🚀 Fast compilation to WebAssembly"</li>
                <li>"⚡ Reactive signals for state management"</li>
                <li>"🎯 Type-safe components"</li>
                <li>"📱 Responsive design"</li>
                <li>"🔄 Client-side routing"</li>
            </ul>
            
            <h2>"Why Leptos?"</h2>
            <p>"Leptos combines the performance of Rust with the interactivity of modern web frameworks. 
               It compiles to WebAssembly for near-native performance in the browser."</p>
        </div>
    }
}

#[component]
fn ContactPage() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (submitted, set_submitted) = create_signal(false);
    
    let submit_form = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_submitted.set(true);
    };
    
    view! {
        <div>
            <h1>"Contact Us"</h1>
            
            {move || if submitted.get() {
                view! {
                    <div style="background: #d4edda; color: #155724; padding: 1rem; border-radius: 4px; margin-bottom: 1rem;">
                        "Thank you for your message! We'll get back to you soon."
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
            
            <form on:submit=submit_form>
                <div style="margin-bottom: 1rem;">
                    <label for="name" style="display: block; margin-bottom: 0.5rem;">"Name:"</label>
                    <input 
                        type="text" 
                        id="name"
                        style="width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px;"
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        prop:value=name
                    />
                </div>
                
                <div style="margin-bottom: 1rem;">
                    <label for="email" style="display: block; margin-bottom: 0.5rem;">"Email:"</label>
                    <input 
                        type="email" 
                        id="email"
                        style="width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px;"
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        prop:value=email
                    />
                </div>
                
                <div style="margin-bottom: 1rem;">
                    <label for="message" style="display: block; margin-bottom: 0.5rem;">"Message:"</label>
                    <textarea 
                        id="message"
                        rows="5"
                        style="width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px; resize: vertical;"
                        on:input=move |ev| set_message.set(event_target_value(&ev))
                        prop:value=message
                    ></textarea>
                </div>
                
                <button 
                    type="submit"
                    style="background: #0066cc; color: white; border: none; padding: 0.75rem 1.5rem; border-radius: 4px; cursor: pointer;"
                >
                    "Send Message"
                </button>
            </form>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}