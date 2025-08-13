use std::{thread, time::Duration};

use leptos::{ev::SubmitEvent, prelude::*};
use shared::SimpleEntry;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let title = RwSignal::new(String::new());
    let body = RwSignal::new(String::new());

    let receipt_animation = RwSignal::new(false);
    let receipt_animation_2 = RwSignal::new(false);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if receipt_animation.get() {
            receipt_animation.set(false);
            receipt_animation_2.set(true);
        } else {
            receipt_animation.set(true);
            receipt_animation_2.set(false);
        }
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&SimpleEntry {
                title: title.get_untracked(),
                body: body.get_untracked(),
            })
            .expect("Could not parse entry");
            invoke("push", args).await;

            title.set(String::new());
            body.set(String::new());
        });
    };

    view! {
        <div class="vignette"></div>
        <form class="container" on:submit=on_submit>
            <h1>Silver Wing</h1>
            <div class="overflow-hidden">
            <div class="receipt" class:receipt_anim=receipt_animation class:receipt_anim_2=receipt_animation_2>
                <input
                    type="text"
                    placeholder="Title"
                    class="receipt_title"
                    bind:value=title
                    required
                />
                <textarea
                    id="greet-textarea"
                    placeholder="Add a body (optional)"
                    class="receipt_body"
                    bind:value=body
                ></textarea>
                <svg
                    id="barcode"
                    width="300px"
                    height="142px"
                    x="0px"
                    y="0px"
                    viewBox="0 0 322 142"
                    xmlns="http://www.w3.org/2000/svg"
                    version="1.1"
                    style="transform: translate(0px)"
                >
                    <g transform="translate(10, 10)" style="fill: #000000">
                        <rect x="0" y="0" width="2" height="100" />
                        <rect x="4" y="0" width="4" height="100" />
                        <rect x="12" y="0" width="2" height="100" />
                        <rect x="18" y="0" width="2" height="100" />
                        <rect x="22" y="0" width="2" height="100" />
                        <rect x="26" y="0" width="2" height="100" />
                        <rect x="30" y="0" width="4" height="100" />
                        <rect x="38" y="0" width="2" height="100" />
                        <rect x="42" y="0" width="2" height="100" />
                        <rect x="46" y="0" width="2" height="100" />
                        <rect x="50" y="0" width="2" height="100" />
                        <rect x="56" y="0" width="4" height="100" />
                        <rect x="62" y="0" width="2" height="100" />
                        <rect x="66" y="0" width="2" height="100" />
                        <rect x="70" y="0" width="2" height="100" />
                        <rect x="76" y="0" width="4" height="100" />
                        <rect x="82" y="0" width="2" height="100" />
                        <rect x="86" y="0" width="2" height="100" />
                        <rect x="90" y="0" width="2" height="100" />
                        <rect x="96" y="0" width="4" height="100" />
                        <rect x="102" y="0" width="2" height="100" />
                        <rect x="106" y="0" width="2" height="100" />
                        <rect x="110" y="0" width="2" height="100" />
                        <rect x="116" y="0" width="4" height="100" />
                        <rect x="122" y="0" width="2" height="100" />
                        <rect x="126" y="0" width="2" height="100" />
                        <rect x="130" y="0" width="2" height="100" />
                        <rect x="136" y="0" width="4" height="100" />
                        <rect x="142" y="0" width="2" height="100" />
                        <rect x="146" y="0" width="2" height="100" />
                        <rect x="152" y="0" width="2" height="100" />
                        <rect x="156" y="0" width="4" height="100" />
                        <rect x="162" y="0" width="2" height="100" />
                        <rect x="166" y="0" width="2" height="100" />
                        <rect x="170" y="0" width="4" height="100" />
                        <rect x="178" y="0" width="2" height="100" />
                        <rect x="182" y="0" width="2" height="100" />
                        <rect x="186" y="0" width="2" height="100" />
                        <rect x="190" y="0" width="2" height="100" />
                        <rect x="196" y="0" width="4" height="100" />
                        <rect x="202" y="0" width="4" height="100" />
                        <rect x="208" y="0" width="2" height="100" />
                        <rect x="212" y="0" width="2" height="100" />
                        <rect x="218" y="0" width="2" height="100" />
                        <rect x="222" y="0" width="2" height="100" />
                        <rect x="228" y="0" width="2" height="100" />
                        <rect x="232" y="0" width="2" height="100" />
                        <rect x="236" y="0" width="4" height="100" />
                        <rect x="242" y="0" width="2" height="100" />
                        <rect x="248" y="0" width="2" height="100" />
                        <rect x="252" y="0" width="4" height="100" />
                        <rect x="258" y="0" width="2" height="100" />
                        <rect x="262" y="0" width="2" height="100" />
                        <rect x="266" y="0" width="2" height="100" />
                        <rect x="272" y="0" width="2" height="100" />
                        <rect x="276" y="0" width="4" height="100" />
                        <rect x="282" y="0" width="2" height="100" />
                        <rect x="286" y="0" width="4" height="100" />
                        <rect x="294" y="0" width="2" height="100" />
                        <rect x="300" y="0" width="2" height="100" />
                    </g>
                </svg>

                <p>2025-08-11T16:56:03 CEST</p>
            </div>
            </div>
            <button type="submit">Submit</button>
        </form>
    }
}
