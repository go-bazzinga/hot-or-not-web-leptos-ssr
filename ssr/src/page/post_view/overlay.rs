use crate::{
    component::{self, canisters_prov::WithAuthCans, modal::Modal, option::SelectOption},
    state::canisters::{auth_canisters_store, Canisters},
    utils::{
        event_streaming::events::{LikeVideo, ShareVideo},
        posts::PostDetails,
        report::ReportOption,
        route::failure_redirect,
        user::UserDetails,
        web::{copy_to_clipboard, share_url},
    },
};
use gloo::timers::callback::Timeout;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_window;

use super::video_iter::post_liked_by_me;


#[derive(Clone, Debug, PartialEq)]
pub enum CoinStates {
    C50,
    C100,
    C200
}

#[component]
pub fn CoinStatesComponent(coin_state: ReadSignal<CoinStates>, 
    // #[prop(default = "w-24 h-24".into())]
    #[prop(default = "".into())]
css_class: String) -> impl IntoView {

    let c50 = move || {coin_state.get() == CoinStates::C50};
    let c100 = move || {coin_state.get() == CoinStates::C100};
    let c200 = move || {coin_state.get() == CoinStates::C200};
    
    let css_class = store_value(css_class);
    view! {
        <Show when=c50>

            <svg
                class=move || css_class.with_value(|css_class| css_class.clone())
                viewBox="0 0 408 408"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <g clip-path="url(#clip0_208_791)">
                    <path
                        d="M204 407.556C316.543 407.556 407.778 316.321 407.778 203.778C407.778 91.2344 316.543 0 204 0C91.4566 0 0.222229 91.2344 0.222229 203.778C0.222229 316.321 91.4566 407.556 204 407.556Z"
                        fill="#FED056"
                    />
                    <path
                        d="M204 369.826C295.705 369.826 370.047 295.484 370.047 203.779C370.047 112.074 295.705 37.7317 204 37.7317C112.295 37.7317 37.9528 112.074 37.9528 203.779C37.9528 295.484 112.295 369.826 204 369.826Z"
                        fill="#FEB635"
                    />
                    <g filter="url(#filter0_d_208_791)">
                        <path
                            d="M112.949 250.093L123.816 233.519C125.953 237.532 129.132 240.737 133.354 243.135C137.576 245.48 142.188 246.653 147.192 246.653C151.778 246.653 155.818 245.689 159.31 243.76C162.802 241.832 165.512 239.148 167.44 235.708C169.421 232.268 170.411 228.307 170.411 223.825C170.411 219.342 169.421 215.381 167.44 211.941C165.46 208.449 162.723 205.739 159.231 203.811C155.739 201.882 151.7 200.918 147.114 200.918C143.83 200.918 140.755 201.465 137.888 202.56C135.022 203.654 132.416 205.244 130.071 207.329L119.907 201.309L125.145 145.176H185.109V163.314H143.986L141.172 190.911C145.029 189.764 149.094 189.191 153.368 189.191C160.769 189.191 167.336 190.755 173.069 193.882C178.802 197.009 183.285 201.335 186.516 206.86C189.799 212.384 191.441 218.717 191.441 225.857C191.441 233.362 189.617 240.06 185.969 245.949C182.32 251.787 177.291 256.399 170.88 259.787C164.469 263.123 157.147 264.79 148.912 264.79C141.928 264.79 135.309 263.514 129.054 260.96C122.8 258.406 117.432 254.784 112.949 250.093ZM245.072 264.79C236.473 264.79 229.019 262.419 222.713 257.676C216.459 252.881 211.638 245.949 208.25 236.88C204.862 227.76 203.168 216.71 203.168 203.732C203.168 190.807 204.862 179.81 208.25 170.741C211.638 161.672 216.459 154.766 222.713 150.023C229.019 145.228 236.473 142.831 245.072 142.831C253.672 142.831 261.099 145.228 267.353 150.023C273.608 154.766 278.429 161.672 281.816 170.741C285.204 179.81 286.898 190.807 286.898 203.732C286.898 216.71 285.204 227.76 281.816 236.88C278.429 245.949 273.608 252.881 267.353 257.676C261.099 262.419 253.672 264.79 245.072 264.79ZM245.072 246.653C251.639 246.653 256.721 242.9 260.317 235.395C263.966 227.89 265.79 217.336 265.79 203.732C265.79 190.181 263.966 179.679 260.317 172.226C256.721 164.721 251.639 160.968 245.072 160.968C238.505 160.968 233.397 164.721 229.749 172.226C226.101 179.679 224.277 190.181 224.277 203.732C224.277 217.336 226.101 227.89 229.749 235.395C233.397 242.9 238.505 246.653 245.072 246.653Z"
                            fill="white"
                        />
                    </g>
                    <path
                        d="M370.047 203.778C370.047 295.319 295.541 369.825 204 369.825C174.15 369.825 146.21 361.944 122.011 348.094C144.299 359.238 169.453 365.526 196.04 365.526C287.581 365.526 362.087 291.02 362.087 199.479C362.087 137.789 328.177 83.8195 278.028 55.2429C332.555 82.4663 370.047 138.744 370.047 203.778ZM215.303 45.5316C248.019 45.5316 278.506 55.0837 304.138 71.4018C276.277 50.3872 241.571 37.8103 204 37.8103C112.459 37.8103 37.9528 112.237 37.9528 203.857C37.9528 262.921 68.997 314.98 115.643 344.273C75.3651 313.945 49.2561 265.787 49.2561 211.579C49.2561 120.038 123.762 45.5316 215.303 45.5316Z"
                        fill="#FC9924"
                    />
                </g>
                <defs>
                    <filter
                        id="filter0_d_208_791"
                        x="112.949"
                        y="142.831"
                        width="188.504"
                        height="136.515"
                        filterUnits="userSpaceOnUse"
                        color-interpolation-filters="sRGB"
                    >
                        <feFlood flood-opacity="0" result="BackgroundImageFix" />
                        <feColorMatrix
                            in="SourceAlpha"
                            type="matrix"
                            values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                            result="hardAlpha"
                        />
                        <feOffset dx="14.5556" dy="14.5556" />
                        <feComposite in2="hardAlpha" operator="out" />
                        <feColorMatrix
                            type="matrix"
                            values="0 0 0 0 0.988235 0 0 0 0 0.6 0 0 0 0 0.141176 0 0 0 1 0"
                        />
                        <feBlend
                            mode="normal"
                            in2="BackgroundImageFix"
                            result="effect1_dropShadow_208_791"
                        />
                        <feBlend
                            mode="normal"
                            in="SourceGraphic"
                            in2="effect1_dropShadow_208_791"
                            result="shape"
                        />
                    </filter>
                    <clipPath id="clip0_208_791">
                        <rect
                            width="407.556"
                            height="407.556"
                            fill="white"
                            transform="translate(0.222229)"
                        />
                    </clipPath>
                </defs>
            </svg>

        </Show>

        <Show when=c100>

            <svg
                class=move || css_class.with_value(|css_class| css_class.clone())
                viewBox="0 0 408 408"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <g clip-path="url(#clip0_208_796)">
                    <path
                        d="M204.222 407.556C316.766 407.556 408 316.321 408 203.778C408 91.2344 316.766 0 204.222 0C91.6789 0 0.444443 91.2344 0.444443 203.778C0.444443 316.321 91.6789 407.556 204.222 407.556Z"
                        fill="#FED056"
                    />
                    <path
                        d="M204.222 369.826C295.927 369.826 370.269 295.484 370.269 203.779C370.269 112.074 295.927 37.7317 204.222 37.7317C112.517 37.7317 38.1747 112.074 38.1747 203.779C38.1747 295.484 112.517 369.826 204.222 369.826Z"
                        fill="#FEB635"
                    />
                    <g filter="url(#filter0_d_208_796)">
                        <path
                            d="M124.887 262.445H104.247V165.503L83.9208 172.852L77.4319 156.434L108.391 145.176H124.887V262.445ZM189.854 264.79C181.254 264.79 173.801 262.419 167.494 257.676C161.24 252.881 156.419 245.949 153.031 236.88C149.643 227.76 147.95 216.71 147.95 203.732C147.95 190.807 149.643 179.81 153.031 170.741C156.419 161.672 161.24 154.766 167.494 150.023C173.801 145.228 181.254 142.831 189.854 142.831C198.453 142.831 205.88 145.228 212.135 150.023C218.389 154.766 223.21 161.672 226.598 170.741C229.986 179.81 231.68 190.807 231.68 203.732C231.68 216.71 229.986 227.76 226.598 236.88C223.21 245.949 218.389 252.881 212.135 257.676C205.88 262.419 198.453 264.79 189.854 264.79ZM189.854 246.653C196.421 246.653 201.502 242.9 205.099 235.395C208.747 227.89 210.571 217.336 210.571 203.732C210.571 190.181 208.747 179.679 205.099 172.226C201.502 164.721 196.421 160.968 189.854 160.968C183.287 160.968 178.179 164.721 174.531 172.226C170.882 179.679 169.058 190.181 169.058 203.732C169.058 217.336 170.882 227.89 174.531 235.395C178.179 242.9 183.287 246.653 189.854 246.653ZM285.389 264.79C276.789 264.79 269.336 262.419 263.029 257.676C256.775 252.881 251.954 245.949 248.566 236.88C245.179 227.76 243.485 216.71 243.485 203.732C243.485 190.807 245.179 179.81 248.566 170.741C251.954 161.672 256.775 154.766 263.029 150.023C269.336 145.228 276.789 142.831 285.389 142.831C293.988 142.831 301.415 145.228 307.67 150.023C313.924 154.766 318.745 161.672 322.133 170.741C325.521 179.81 327.215 190.807 327.215 203.732C327.215 216.71 325.521 227.76 322.133 236.88C318.745 245.949 313.924 252.881 307.67 257.676C301.415 262.419 293.988 264.79 285.389 264.79ZM285.389 246.653C291.956 246.653 297.037 242.9 300.634 235.395C304.282 227.89 306.106 217.336 306.106 203.732C306.106 190.181 304.282 179.679 300.634 172.226C297.037 164.721 291.956 160.968 285.389 160.968C278.822 160.968 273.714 164.721 270.066 172.226C266.417 179.679 264.593 190.181 264.593 203.732C264.593 217.336 266.417 227.89 270.066 235.395C273.714 242.9 278.822 246.653 285.389 246.653Z"
                            fill="white"
                        />
                    </g>
                    <path
                        d="M370.269 203.778C370.269 295.319 295.763 369.825 204.222 369.825C174.372 369.825 146.432 361.944 122.233 348.094C144.521 359.238 169.675 365.526 196.262 365.526C287.803 365.526 362.309 291.02 362.309 199.479C362.309 137.789 328.399 83.8195 278.25 55.2429C332.777 82.4663 370.269 138.744 370.269 203.778ZM215.525 45.5316C248.241 45.5316 278.728 55.0837 304.359 71.4018C276.499 50.3872 241.793 37.8103 204.222 37.8103C112.681 37.8103 38.1747 112.237 38.1747 203.857C38.1747 262.921 69.219 314.98 115.865 344.273C75.587 313.945 49.478 265.787 49.478 211.579C49.478 120.038 123.984 45.5316 215.525 45.5316Z"
                        fill="#FC9924"
                    />
                </g>
                <defs>
                    <filter
                        id="filter0_d_208_796"
                        x="77.4319"
                        y="142.831"
                        width="264.338"
                        height="136.515"
                        filterUnits="userSpaceOnUse"
                        color-interpolation-filters="sRGB"
                    >
                        <feFlood flood-opacity="0" result="BackgroundImageFix" />
                        <feColorMatrix
                            in="SourceAlpha"
                            type="matrix"
                            values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                            result="hardAlpha"
                        />
                        <feOffset dx="14.5556" dy="14.5556" />
                        <feComposite in2="hardAlpha" operator="out" />
                        <feColorMatrix
                            type="matrix"
                            values="0 0 0 0 0.988235 0 0 0 0 0.6 0 0 0 0 0.141176 0 0 0 1 0"
                        />
                        <feBlend
                            mode="normal"
                            in2="BackgroundImageFix"
                            result="effect1_dropShadow_208_796"
                        />
                        <feBlend
                            mode="normal"
                            in="SourceGraphic"
                            in2="effect1_dropShadow_208_796"
                            result="shape"
                        />
                    </filter>
                    <clipPath id="clip0_208_796">
                        <rect
                            width="407.556"
                            height="407.556"
                            fill="white"
                            transform="translate(0.444443)"
                        />
                    </clipPath>
                </defs>
            </svg>

        </Show>
        <Show when=c200>
            <svg
                class=move || css_class.with_value(|css_class| css_class.clone())
                viewBox="0 0 408 408"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <g clip-path="url(#clip0_467_964)">
                    <path
                        d="M203.778 407.556C316.321 407.556 407.556 316.321 407.556 203.778C407.556 91.2344 316.321 0 203.778 0C91.2344 0 0 91.2344 0 203.778C0 316.321 91.2344 407.556 203.778 407.556Z"
                        fill="#FED056"
                    />
                    <path
                        d="M203.777 369.826C295.483 369.826 369.824 295.484 369.824 203.779C369.824 112.074 295.483 37.7317 203.777 37.7317C112.072 37.7317 37.7303 112.074 37.7303 203.779C37.7303 295.484 112.072 369.826 203.777 369.826Z"
                        fill="#FEB635"
                    />
                    <g filter="url(#filter0_d_467_964)">
                        <path
                            d="M145.629 262.445H72.2967V250.171C81.6782 241.884 89.5743 234.509 95.985 228.046C102.448 221.583 107.634 215.746 111.543 210.534C115.504 205.27 118.37 200.397 120.142 195.915C121.914 191.432 122.801 187.08 122.801 182.859C122.801 178.428 121.967 174.572 120.299 171.288C118.683 167.952 116.416 165.372 113.497 163.548C110.578 161.724 107.191 160.812 103.334 160.812C98.8516 160.812 94.5517 162.089 90.4343 164.643C86.369 167.145 82.9812 170.637 80.271 175.119L69.5604 161.985C73.9385 155.939 79.4371 151.248 86.0563 147.912C92.7276 144.525 99.7376 142.831 107.086 142.831C114.018 142.831 120.247 144.342 125.771 147.365C131.348 150.336 135.752 154.506 138.984 159.874C142.267 165.242 143.909 171.575 143.909 178.871C143.909 184.24 142.971 189.66 141.094 195.133C139.218 200.605 136.43 206.104 132.729 211.629C129.081 217.153 124.599 222.652 119.282 228.124C113.966 233.597 107.894 239.017 101.067 244.386H145.629V262.445ZM203.091 264.79C194.491 264.79 187.038 262.419 180.731 257.676C174.477 252.881 169.656 245.949 166.268 236.88C162.88 227.76 161.187 216.71 161.187 203.732C161.187 190.807 162.88 179.81 166.268 170.741C169.656 161.672 174.477 154.766 180.731 150.023C187.038 145.228 194.491 142.831 203.091 142.831C211.69 142.831 219.117 145.228 225.372 150.023C231.626 154.766 236.447 161.672 239.835 170.741C243.223 179.81 244.917 190.807 244.917 203.732C244.917 216.71 243.223 227.76 239.835 236.88C236.447 245.949 231.626 252.881 225.372 257.676C219.117 262.419 211.69 264.79 203.091 264.79ZM203.091 246.653C209.658 246.653 214.739 242.9 218.336 235.395C221.984 227.89 223.808 217.336 223.808 203.732C223.808 190.181 221.984 179.679 218.336 172.226C214.739 164.721 209.658 160.968 203.091 160.968C196.524 160.968 191.416 164.721 187.767 172.226C184.119 179.679 182.295 190.181 182.295 203.732C182.295 217.336 184.119 227.89 187.767 235.395C191.416 242.9 196.524 246.653 203.091 246.653ZM298.626 264.79C290.026 264.79 282.573 262.419 276.266 257.676C270.012 252.881 265.191 245.949 261.803 236.88C258.415 227.76 256.722 216.71 256.722 203.732C256.722 190.807 258.415 179.81 261.803 170.741C265.191 161.672 270.012 154.766 276.266 150.023C282.573 145.228 290.026 142.831 298.626 142.831C307.225 142.831 314.652 145.228 320.907 150.023C327.161 154.766 331.982 161.672 335.37 170.741C338.758 179.81 340.452 190.807 340.452 203.732C340.452 216.71 338.758 227.76 335.37 236.88C331.982 245.949 327.161 252.881 320.907 257.676C314.652 262.419 307.225 264.79 298.626 264.79ZM298.626 246.653C305.193 246.653 310.274 242.9 313.871 235.395C317.519 227.89 319.343 217.336 319.343 203.732C319.343 190.181 317.519 179.679 313.871 172.226C310.274 164.721 305.193 160.968 298.626 160.968C292.059 160.968 286.951 164.721 283.303 172.226C279.654 179.679 277.83 190.181 277.83 203.732C277.83 217.336 279.654 227.89 283.303 235.395C286.951 242.9 292.059 246.653 298.626 246.653Z"
                            fill="white"
                        />
                    </g>
                    <path
                        d="M369.824 203.778C369.824 295.319 295.318 369.825 203.777 369.825C173.927 369.825 145.987 361.944 121.789 348.094C144.077 359.238 169.231 365.526 195.817 365.526C287.358 365.526 361.864 291.02 361.864 199.479C361.864 137.789 327.954 83.8195 277.806 55.2429C332.332 82.4663 369.824 138.744 369.824 203.778ZM215.081 45.5316C247.797 45.5316 278.284 55.0837 303.915 71.4018C276.055 50.3872 241.349 37.8103 203.777 37.8103C112.237 37.8103 37.7303 112.237 37.7303 203.857C37.7303 262.921 68.7746 314.98 115.421 344.273C75.1426 313.945 49.0336 265.787 49.0336 211.579C49.0336 120.038 123.54 45.5316 215.081 45.5316Z"
                        fill="#FC9924"
                    />
                </g>
                <defs>
                    <filter
                        id="filter0_d_467_964"
                        x="69.5604"
                        y="142.831"
                        width="285.447"
                        height="136.515"
                        filterUnits="userSpaceOnUse"
                        color-interpolation-filters="sRGB"
                    >
                        <feFlood flood-opacity="0" result="BackgroundImageFix" />
                        <feColorMatrix
                            in="SourceAlpha"
                            type="matrix"
                            values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                            result="hardAlpha"
                        />
                        <feOffset dx="14.5556" dy="14.5556" />
                        <feComposite in2="hardAlpha" operator="out" />
                        <feColorMatrix
                            type="matrix"
                            values="0 0 0 0 0.988235 0 0 0 0 0.6 0 0 0 0 0.141176 0 0 0 1 0"
                        />
                        <feBlend
                            mode="normal"
                            in2="BackgroundImageFix"
                            result="effect1_dropShadow_467_964"
                        />
                        <feBlend
                            mode="normal"
                            in="SourceGraphic"
                            in2="effect1_dropShadow_467_964"
                            result="shape"
                        />
                    </filter>
                    <clipPath id="clip0_467_964">
                        <rect width="407.556" height="407.556" fill="white" />
                    </clipPath>
                </defs>
            </svg>

        </Show>
    }

}



#[component]
pub fn CoinButton() -> impl IntoView {
    let (coin_state, set_coin_state) = create_signal(CoinStates::C50);
    // let toggle_coin_state =  |current_coin_state | {
    //     match current_coin_state {
    //         CoinStates::C50 => CoinStates::C100,
    //         CoinStates::C100 => CoinStates::C200,
    //         CoinStates::C200 => CoinStates::C50
    //     }
    // };
    // create a signal with three states (enum?)

    view! {
        <div class="flex flex-col items-center justify-center">
            <Icon class="text-2xl justify-self-end text-white mb-2" icon=icondata::AiUpOutlined />

            <button on:click=move |ev| {
                ev.stop_propagation();
                set_coin_state
                    .update(|cur_coin_state: &mut CoinStates| {
                        *cur_coin_state = match *cur_coin_state {
                            CoinStates::C50 => CoinStates::C100,
                            CoinStates::C100 => CoinStates::C200,
                            CoinStates::C200 => CoinStates::C50,
                        };
                    });
            }>
                // todo adjust css of coins
                <CoinStatesComponent coin_state />
            </button>

            <Icon
                class="text-2xl justify-self-end text-white mt-2 "
                icon=icondata::AiDownOutlined
            />
        </div>
    }
}

#[component]
pub fn HotButton(post: ReadSignal<PostDetails>, 
    #[prop(default = "w-24 h-24".into())]
    css_class: String) -> impl IntoView {
    view! {
        <p class=css_class>
            <svg viewBox="0 0 492 492" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect width="491.982" height="491.982" rx="245.991" fill="#E2017B" />
                <path
                    d="M205.289 81.7363C205.289 81.7363 199.994 133.945 207.949 142.358C215.903 150.772 264.996 170.236 274.292 186.153C283.589 202.069 276.497 229.516 335.772 279.519C357.791 298.096 346.482 384.295 330.022 400.739C310.998 419.772 218.611 439.188 177.448 412.676C145.175 391.87 134.992 339.23 136.31 323.313C137.627 307.397 184.516 236.156 197.79 216.692C211.063 197.227 177.879 186.608 174.789 172.897C171.698 159.186 179.652 101.632 205.289 81.7363Z"
                    fill="white"
                />
                <path
                    d="M323.361 220.551C314.975 226.328 298.155 206.936 298.155 195.454C298.155 183.948 316.964 168.439 313.202 139.266C313.202 139.242 341.498 208.062 323.361 220.551Z"
                    fill="white"
                />
                <path
                    d="M252.609 139.242C239.599 130.564 230.95 116.23 233.154 103.837C235.358 91.444 251.291 66.6823 250.836 59.5869C250.836 59.5869 246.403 100.289 252.609 111.364C258.79 122.438 263.223 146.337 252.609 139.242Z"
                    fill="white"
                />
                <path
                    d="M160.173 244.545C160.173 244.545 182.719 221.533 172.105 203.842C161.491 186.152 157.514 170.667 157.945 158.706C157.969 158.73 137.196 229.06 160.173 244.545Z"
                    fill="white"
                />
                <path
                    d="M282.487 293.206L252.945 296.634L240.342 314.276L224.05 299.966L194.508 303.394L181.905 321.036L185.403 351.167L249.998 397.79L302.253 337.648L298.779 307.517L282.487 293.206Z"
                    fill="#E2017B"
                />
            </svg>

        </p>
    }
}


#[component]
pub fn NotButton(post: ReadSignal<PostDetails>, #[prop(default = "w-24 h-24".into())]
css_class: String) -> impl IntoView{
    view! {
        <p class=css_class>

            <svg viewBox="0 0 492 492" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect width="491.982" height="491.982" rx="245.991" fill="white" />
                <path
                    d="M279.489 64.7114C279.489 64.7114 284.784 116.92 276.829 125.333C268.875 133.747 219.782 153.211 210.485 169.128C201.189 185.045 208.281 212.491 149.005 262.494C126.987 281.071 138.296 367.246 154.756 383.714C173.78 402.747 266.167 422.163 307.33 395.651C339.603 374.845 349.786 322.205 348.468 306.289C347.15 290.372 300.262 219.131 286.988 199.667C273.714 180.202 306.898 169.583 309.989 155.872C313.104 142.137 305.149 84.6071 279.489 64.7114Z"
                    fill="#E2017B"
                />
                <path
                    d="M161.416 203.526C169.802 209.303 186.622 189.911 186.622 178.429C186.622 166.923 167.813 151.414 171.575 122.241C171.599 122.217 143.303 191.013 161.416 203.526Z"
                    fill="#E2017B"
                />
                <path
                    d="M232.192 122.217C245.202 113.539 253.852 99.2049 251.647 86.812C249.443 74.4191 233.51 49.6574 233.965 42.562C233.965 42.562 238.398 83.2643 232.192 94.3388C225.987 105.413 221.578 129.312 232.192 122.217Z"
                    fill="#E2017B"
                />
                <path
                    d="M324.604 227.52C324.604 227.52 302.059 204.508 312.673 186.818C323.287 169.127 327.264 153.642 326.833 141.681C326.809 141.681 347.606 212.035 324.604 227.52Z"
                    fill="#E2017B"
                />
                <path
                    d="M302.04 326.968L299.563 305.385L292.608 283.382L273.102 264.429L254.072 255.372L221.156 256.862L204.698 264.918L190.098 279.773L178.76 308.412L181.738 335.56L179.308 355.514L206.484 364.594L207.151 371.626L215.106 377.61L225.229 374.001L228.825 383.058L240.638 382.523L243.497 373.512L247.284 379.752L256.692 381.801L262.336 375.747L261.479 366.62L272.173 375.002L279.866 370.811L279.533 361.288L298.468 352.603L306.446 342.312L302.04 326.968ZM228.896 323.499L220.06 332.976L206.937 333.558L197.243 324.92L196.647 312.09L205.484 302.614L218.607 302.032L228.301 310.67L228.896 323.499ZM247.808 350.042L237.947 350.485L234.47 341.474L242.163 335.444L250.404 340.752L247.808 350.042ZM283.201 320.216L274.364 329.693L261.241 330.275L251.547 321.637L250.952 308.807L259.788 299.331L272.911 298.749L282.605 307.387L283.201 320.216Z"
                    fill="white"
                />
            </svg>

        </p>
    }
}



#[component]
pub fn HNButtonsOverlay(post: ReadSignal<PostDetails>) -> impl IntoView {
    // let post_clone = post.clone();
    view! {
        <div class="flex justify-center absolute bottom-0 left-0 w-full bg-gradient-to-t from-black/70 to-transparent ">
            <div class="flex flex-nowrap items-center space-x-12 pb-20 px-4 bg-transparent z-[4] max-w-screen-sm	">
                <HotButton post=post />
                <CoinButton />
                <NotButton post />
            </div>

        </div>
    }

}


#[component]
pub fn HNWonLost(post: ReadSignal<PostDetails>) -> impl IntoView {  

    // todo based on placed_bet_detail.value, decide the coin state. C50, C100, C200
    let (coin_state, set_coin_state) = create_signal(CoinStates::C50);

    // todo create_signal for win/lost
    let (won_signal, set_won_signal) = create_signal(true);
    
    // todo create_signal for bet direction
    let bet_direction_is_hot = create_rw_signal(true);

    view! {
        <div class="flex w-auto items-center rounded-xl bg-transparent p-4 shadow-sm backdrop-blur-sm">
            // <!-- Coin Component -->
            <div class="relative flex-shrink-0">

                <div class="h-[5rem] w-[5rem] ">
                    // <!-- Assume we have a Coin component here -->
                    // placed_bet_detail.value
                    // coin value
                    <CoinStatesComponent coin_state />
                </div>

                // <!-- Hot icon -->
                <div class="absolute -bottom-1 -right-2 flex items-center justify-center rounded-full">
                    <span>
                        <Show when=bet_direction_is_hot>
                            <HotButton post=post css_class="h-9 w-9".into() />
                        </Show>

                        <Show when=move || !bet_direction_is_hot.get()>
                            <NotButton post=post css_class="h-9 w-9".into() />
                        </Show>
                    // hot or not icon
                    </span>
                </div>
            </div>

            // <!-- Text and Badge Column -->
            <div class="ml-4 flex flex-grow flex-col">
                // <!-- Result Text -->
                <div class="text-sm leading-snug text-gray-800">
                    <p>You staked placed_bet_detail.value tokens on Hot.</p>
                    <p>You received placed_bet_detail.reward tokens.</p>
                </div>

                <Show when=won_signal>
                    // <!-- Win Badge as a full-width button -->
                    <button class="mt-2 w-full rounded-sm bg-pink-500 px-4 py-2 text-sm font-bold text-white">
                        <div class="flex justify-center items-center">
                            <span class="">
                                <Icon
                                    class="fill-white"
                                    style=""
                                    icon=icondata::RiTrophyFinanceFill
                                />
                            </span>
                            <span class="ml-2">"You Won"</span>
                        </div>
                    </button>
                </Show>

                <Show when=move || !won_signal.get()>
                    // <!-- Lost Badge as a full-width button -->
                    <button class="mt-2 w-full rounded-sm bg-white px-4 py-2 text-sm font-bold text-black">
                        <Icon class="fill-white" style="" icon=icondata::RiTrophyFinanceFill />
                        "You Lost"
                    </button>
                </Show>
            </div>
        </div>
    }
}


#[component]
fn LikeAndAuthCanLoader(post: PostDetails) -> impl IntoView {
    let likes = create_rw_signal(post.likes);

    let liked = create_rw_signal(None::<bool>);
    let icon_class = Signal::derive(move || {
        if liked().unwrap_or_default() {
            Some(TextProp::from("fill-primary-600"))
        } else {
            None
        }
    });
    let icon_style = Signal::derive(move || {
        if liked().unwrap_or_default() {
            Some(TextProp::from("filter: drop-shadow(2px 0 0 white) drop-shadow(-2px 0 0 white) drop-shadow(0 2px 0 white) drop-shadow(0 -2px 0 white);"))
        } else {
            None
        }
    });

    let post_canister = post.canister_id;
    let post_id = post.post_id;
    let initial_liked = post.liked_by_user;
    let canisters = auth_canisters_store();

    let like_toggle = create_action(move |&()| {
        let post_details = post.clone();
        let canister_store = canisters;

        async move {
            let Some(canisters) = canisters.get_untracked() else {
                log::warn!("Trying to toggle like without auth");
                return;
            };
            batch(move || {
                if liked.get_untracked().unwrap_or_default() {
                    likes.update(|l| *l -= 1);
                    liked.set(Some(false));
                } else {
                    likes.update(|l| *l += 1);
                    liked.set(Some(true));

                    LikeVideo.send_event(post_details, likes, canister_store);
                }
            });
            let Ok(individual) = canisters.individual_user(post_canister).await else {
                return;
            };
            match individual
                .update_post_toggle_like_status_by_caller(post_id)
                .await
            {
                Ok(_) => (),
                Err(e) => {
                    log::warn!("Error toggling like status: {:?}", e);
                    liked.update(|l| _ = l.as_mut().map(|l| *l = !*l));
                }
            }
        }
    });

    let liked_fetch = move |cans: Canisters<true>| async move {
        if let Some(liked) = initial_liked {
            return liked;
        }

        match post_liked_by_me(&cans, post_canister, post_id).await {
            Ok(liked) => liked,
            Err(e) => {
                failure_redirect(e);
                false
            }
        }
    };

    let liking = like_toggle.pending();

    view! {
        <div class="relative flex flex-col gap-1 items-center">
            <button
                on:click=move |_| like_toggle.dispatch(())
                class="drop-shadow-lg"
                disabled=move || liking() || liked.with(|l| l.is_none())
            >
                <Icon class=icon_class style=icon_style icon=icondata::AiHeartFilled />
            </button>
            <span class="absolute -bottom-5 text-sm md:text-md">{likes}</span>
        </div>
        <WithAuthCans with=liked_fetch let:d>
            {move || liked.set(Some(d.1))}
        </WithAuthCans>
    }
}

#[component]
pub fn VideoDetailsOverlay(post: PostDetails) -> impl IntoView {
    let show_share = create_rw_signal(false);
    let show_report = create_rw_signal(false);
    let (report_option, set_report_option) =
        create_signal(ReportOption::Nudity.as_str().to_string());
    let show_copied_popup = create_rw_signal(false);
    let base_url = || {
        use_window()
            .as_ref()
            .and_then(|w| w.location().origin().ok())
    };
    let video_url = move || {
        base_url()
            .map(|b| format!("{b}/hot-or-not/{}/{}", post.canister_id, post.post_id))
            .unwrap_or_default()
    };

    let post_details_share = post.clone();
    let canisters = auth_canisters_store();
    let canisters_copy = canisters;

    let share = move || {
        let post_details = post_details_share.clone();
        let url = video_url();
        if share_url(&url).is_some() {
            return;
        }
        show_share.set(true);
        ShareVideo.send_event(post_details, canisters);
    };

    let profile_url = format!("/profile/{}", post.poster_principal.to_text());
    let post_c = post.clone();

    let click_copy = move |text: String| {
        _ = copy_to_clipboard(&text);
        show_copied_popup.set(true);
        Timeout::new(1200, move || show_copied_popup.set(false)).forget();
    };
    let post_details_report = post.clone();

    let (post_read_signal, set_post_read_signal) = create_signal(post.clone());

    let click_report = create_action(move |()| {
        #[cfg(feature = "ga4")]
        {
            use crate::utils::report::send_report_offchain;

            let post_details = post_details_report.clone();
            let user_details = UserDetails::try_get_from_canister_store(canisters_copy).unwrap();

            spawn_local(async move {
                send_report_offchain(
                    user_details.details.principal.to_string(),
                    post_details.poster_principal.to_string(),
                    post_details.canister_id.to_string(),
                    post_details.post_id.to_string(),
                    post_details.uid,
                    report_option.get_untracked(),
                    video_url(),
                )
                .await
                .unwrap();
            });
        }

        async move {
            show_report.set(false);
        }
    });

    view! {
        <div class="flex flex-row flex-nowrap justify-between items-end pt-20 px-2 md:px-6 w-full text-white absolute top-0 left-0 bg-transparent z-[4]">
            <div class="flex flex-col gap-2 w-9/12">
                <div class="flex flex-row items-center gap-2 min-w-0">
                    <a
                        href=profile_url
                        class="w-10 md:w-12 h-10 md:h-12 overflow-clip rounded-full border-white border-2"
                    >
                        <img class="h-full w-full object-cover" src=post.propic_url />
                    </a>
                    <div class="flex flex-col w-7/12">
                        <span class="w-7/12 text-md md:text-lg font-bold truncate ...">
                            {post.display_name}
                        </span>
                        <span class="flex gap-1 items-center text-sm md:text-md">
                            <Icon icon=icondata::AiEyeOutlined />
                            {post.views}
                        </span>
                    </div>

                </div>
                <ExpandableText description=post.description />
            </div>

        </div>

        <div class="mb-20 px-2 md:px-6 w-full text-white relative right-0 bg-transparent z-[4]">

            <div class="grow-0 justify-self-end flex flex-col gap-8 items-end w-3/12 text-4xl">
                <button on:click=move |_| show_report.set(true)>
                    <Icon class="drop-shadow-lg" icon=icondata::TbMessageReport />
                </button>
                <a href="/refer-earn">
                    <Icon class="drop-shadow-lg" icon=icondata::AiGiftFilled />
                </a>
                <LikeAndAuthCanLoader post=post_c />
                <button on:click=move |_| share()>
                    <Icon class="drop-shadow-lg" icon=icondata::RiSendPlaneBusinessFill />
                </button>
            </div>

        </div>

        <div class="flex flex-row flex-nowrap justify-end items-end mb-10 px-2 md:px-6 w-full text-white absolute bottom-0 right-0 bg-transparent z-[4]">
            <div class="flex flex-col grow gap-8 items-center w-9/12 sm:text-xl md:text-2xl py-10">
                // todo do not show hon button if it is not enabled on a post
                // <HNButtonsOverlay post=post_read_signal />
                <HNWonLost post=post_read_signal />
            </div>
        </div>

        <Modal show=show_share>
            <div class="flex flex-col justify-center items-center gap-4 text-white">
                <span class="text-lg">Share</span>
                <div class="flex flex-row w-full gap-2">
                    <p class="text-md max-w-full bg-white/10 rounded-full p-2 overflow-x-scroll whitespace-nowrap">
                        {video_url}
                    </p>
                    <button on:click=move |_| click_copy(video_url())>
                        <Icon class="text-xl" icon=icondata::FaCopyRegular />
                    </button>
                </div>
            </div>

            <Show when=show_copied_popup>
                <div class="flex flex-col justify-center items-center">
                    <span class="absolute mt-80 flex flex-row justify-center items-center bg-white/90 rounded-md h-10 w-28 text-center shadow-lg">
                        <p>Link Copied!</p>
                    </span>
                </div>
            </Show>
        </Modal>

        <Modal show=show_report>
            <div class="flex flex-col justify-center items-center gap-4 text-white">
                <span class="text-lg">Report Post</span>
                <span class="text-lg">Please select a reason:</span>
                <div class="max-w-full text-md text-black">
                    <select
                        class="p-2 w-full block rounded-lg text-sm"
                        on:change=move |ev| {
                            let new_value = event_target_value(&ev);
                            set_report_option(new_value);
                        }
                    >
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Nudity.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Violence.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Offensive.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Spam.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Other.as_str())
                        />
                    </select>
                </div>
                <button on:click=move |_| click_report.dispatch(())>
                    <div class="rounded-lg bg-pink-500 p-1">Submit</div>
                </button>
            </div>
        </Modal>
    }
}

#[component]
fn ExpandableText(description: String) -> impl IntoView {
    let truncated = create_rw_signal(true);

    view! {
        <span
            class="text-sm md:text-md ms-2 md:ms-4 w-full"
            class:truncate=truncated

            on:click=move |_| truncated.update(|e| *e = !*e)
        >
            {description}
        </span>
    }
}

#[component]
pub fn HomeButtonOverlay() -> impl IntoView {
    view! {
        // <div class="flex justify-center items-center">
        <div class="flex w-full items-center justify-center pt-4 absolute top-0 left-0 bg-transparent z-[4]">// <img src="/img/yral-logo.svg" alt="Logo"/>
        // </div>
        // <div class="rounded-full p-2 text-white bg-black/20">
        // <div class="flex flex-row items-center gap-1 py-2 px-6 rounded-full">
        // <Icon class="w-3 h-3" icon=HomeSymbolFilled/>
        // <span class="font-sans font-semibold">Home Feed</span>
        // </div>
        // </div>
        </div>
    }
}
