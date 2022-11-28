use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::{Callback, events::InputEvent, function_component, html, prelude::*};

#[function_component(Bws)]
pub fn bwserver() -> Html {
    let nblisteners = use_state_eq(|| 0.0);
    let bitrate = use_state_eq(|| 0.0);
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

    let calculate = {
        let state = state.clone();
        let nblisteners = nblisteners;
        let bitrate = bitrate;
        Callback::from(move |_| {
            let nblisteners = *nblisteners;
            let bitrate = *bitrate;
            state.set(125.0 * nblisteners * bitrate / 128.0);
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
        <button onclick={calculate}> {"Calculate"} </button>
        <p> {"Server bandwidth (Mib/s): "} {*state} </p>
        </>
    )
}