use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::{Callback, events::InputEvent, function_component, html, prelude::*};

#[function_component(Subw)]
pub fn server_usage_bw() -> html {
    let nblisteners = use_state_eq(|| 0.0);
    let bitrate = use_state_eq(|| 0.0);
    let nbdays = use_state_eq(|| 0.0);
    let nbhours = use_state_eq(|| 0.0);
    let state = use_state(|| 0.0);

    let bitrate_callback = {
        let bitrate = bitrate.clone();
        Callback::from(move |e: InputEvent| {
            let event: Event = e.dyn_into().unwrap();
            let target: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let value = target.value();
            bitrate.set(value.parse::<f64>().unwrap());
        })
    };

    let nblisteners_callback = {
        let nblisteners = nblisteners.clone();
        Callback::from(move |e: InputEvent| {
            let event: Event = e.dyn_into().unwrap();
            let target: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let value = target.value();
            nblisteners.set(value.parse::<f64>().unwrap());
        })
    };

    let nbdays_callback = {
        let nbdays = nbdays.clone();
        Callback::from(move |e: InputEvent| {
            let event: Event = e.dyn_into().unwrap();
            let target: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let value = target.value();
            nbdays.set(value.parse::<f64>().unwrap());
        })
    };

    let nbhours_callback = {
        let nbhours = nbhours.clone();
        Callback::from(move |e: InputEvent| {
            let event: Event = e.dyn_into().unwrap();
            let target: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let value = target.value();
            nbhours.set(value.parse::<f64>().unwrap());
        })
    };

    let calculate = {
        let state = state.clone();
        let nblisteners = nblisteners.clone();
        let nbdays = nbdays.clone();
        let nbhours = nbhours.clone();
        let bitrate = bitrate.clone();
        Callback::from(move |_| {
            let nblisteners = *nblisteners;
            let bitrate = *bitrate;
            let nbdays = *nbdays;
            let nbhours = *nbhours;
            state.set(28125.0 * nbdays * nbhours * nblisteners * bitrate / 65536.0);
        })
    };

    html!(
        <>
        <p>
        <label for="nblisteners">{"Number of listeners: "}</label>
        <input
            id="nblisteners"
            type="number"
            min="0"
            oninput={nblisteners_callback}
        />
        </p>
        <p>
        <label for="bitrate">{"Bitrate (kb/s): "}</label>
        <input
            id="bitrate"
            type="number"
            min="0"
            oninput={bitrate_callback}
        />
        </p>
        <p>
        <label for="nbdays">{"Number of days: "}</label>
        <input
            id="nbdays"
            type="number"
            min="0"
            max="366"
            oninput={nbdays_callback}
        />
        </p>
        <p>
        <label for="nbhours">{"Number of hours: "}</label>
        <input
            id="nbhours"
            type="number"
            min="0"
            max="24"
            oninput={nbhours_callback}
        />
        </p>
        <button onclick={calculate}> {"Calculate"} </button>
        <p> {"Server bandwidth (Mib/s): "} {*state} </p>
        </>
    )
}