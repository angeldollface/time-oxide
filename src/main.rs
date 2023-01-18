/*
TIME OXIDE by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// Declaring our
// modules.
mod modules;

// Importing Yew's APIs.
use yew::prelude::*;

// Importing our time component.
use modules::time_cog::TimeCog;

// Main app component
// that renders our time
// component.
#[function_component]
fn App() -> Html {
    return html!{
        <>
         <TimeCog/>
        </>
    }
}

// Main entry point for the
// Rust compiler and handing over
// our main app component
// to Yew's renderer.
fn main(){
    yew::Renderer::<App>::new().render();
}