/*
TIME OXIDE by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// Importing Yew's APIs.
use yew::prelude::*;

// Importing our time handler.
use super::time::Now;

// Importing APIs from
// Yew hooks.
use yew_hooks::prelude::*;

// The functional component
// that renders and refreshes our
// reading from our time handler.
#[function_component]
pub fn TimeCog() -> Html {

    // Seconds as a stateful string variable.
    let time_seconds = use_state(|| Now::seconds());

    // Minutes as a stateful string variable.
    let time_minutes = use_state(|| Now::minutes());

    // Hours as a stateful string variable.
    let time_hours = use_state(|| Now::hours());

    // Closure to run the refresh every 1000 milliseconds.
    {
        // Cloning our seconds to mutate them in the DOM.
        let time_seconds_clone = time_seconds.clone();

        // Cloning our minutes to mutate them in the DOM.
        let time_minutes_clone = time_minutes.clone();

        // Cloning our hours to mutate them in the DOM.
        let time_hours_clone = time_hours.clone();

        // Using the "use_interval" hook from Yew Hooks to
        // refresh stateful data.
        use_interval(
            move || {

                // Updating our seconds.
                time_seconds_clone.set(Now::seconds());

                // Updating our minutes.
                time_minutes_clone.set(Now::minutes());

                // Updating our hours.
                time_hours_clone.set(Now::hours());
            },

            // Closure runs every 1000 milliseconds.
            1000
        );
    };

    // Returning the container and paragraphs with the stateful
    // data.
    return html!{
        <div class="timeContainer">
         <p>
         { format!("{}", &time_hours.clone().to_string()) }
         </p>
         <p>
         { format!("{}", &time_minutes.clone().to_string()) }
         </p>
         <p>
         { format!("{}", &time_seconds.clone().to_string()) }
         </p>
        </div>
    }
}