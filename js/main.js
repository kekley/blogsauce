import { get_splash, verify_token } from "./api.js";
import { get_shouts, post_shout, subscribe_shouts } from "./api.js";
import { generate_shout_html, generate_comment_html } from "./html-bits.js";
import {
  bind_button_to_close_modal,
  bind_button_to_open_modal,
} from "./modal.js";

import { register_display_name } from "./api.js";

import { get_comments, post_comment } from "./api.js";

async function refresh_comments_for(post_list) {
  const token = localStorage.getItem("token");
  let post_list_comments = await get_comments(post_list, token);
  let timezone_offset = new Date().getTimezoneOffset();
  if ("posts" in post_list_comments) {
    for (const post of post_list_comments.posts) {
      let comments_list = document.getElementById(
        `comments-list-${post.post_ident}`,
      );
      comments_list.innerHTML = "";
      if ("comments" in post) {
        for (const comment of post.comments) {
          let comment_html = generate_comment_html(comment, timezone_offset);
          const comment_template = document.createElement("template");
          comment_template.innerHTML = comment_html;
          comments_list.append(comment_template.content);
        }
      }
    }
  }
}

document.addEventListener("DOMContentLoaded", async function () {
  document.title = await get_splash();
  const maybe_token = localStorage.getItem("token");
  if (maybe_token && maybe_token.length == 32) {
    try_login(maybe_token);
  } else {
    do_logout();
  }
  //Populate the shoutbox messages
  const token = localStorage.getItem("token");
  let shouts = await get_shouts(null, token);
  console.log(shouts);
  let messages = document.getElementById("shoutbox-messages");
  if ("shouts" in shouts) {
    let shouts_array = shouts["shouts"];
    let scrollArea = document.getElementById("shoutbox-scroll");
    for (let i = 0; i < shouts_array.length; ++i) {
      let item = shouts_array[i];
      let html = generate_shout_html(item);
      messages.innerHTML += html;
    }
    scrollArea.scrollTop = scrollArea.scrollHeight;
  }

  let shout_callback = function (shout) {
    let messages = document.getElementById("shoutbox-messages");
    let scrollArea = document.getElementById("shoutbox-scroll");
    const should_scroll_bottom =
      scrollArea.scrollTop + scrollArea.clientHeight >=
      scrollArea.scrollHeight - 1;
    let html = generate_shout_html(shout);
    messages.innerHTML += html;
    if (should_scroll_bottom) {
      scrollArea.scrollTop = scrollArea.scrollHeight;
    }
  };

  subscribe_shouts(shout_callback);

  let post_list = JSON.parse(
    document.getElementById("visible-posts-list").dataset.posts,
  );
  await refresh_comments_for(post_list.post_idents);

  //Hook up all the submit buttons
  for (const post_ident of post_list.post_idents) {
    const submit_button = document.getElementById(
      `comment-submit-${post_ident}`,
    );
    const comment_input = document.getElementById(
      `comment-input-${post_ident}`,
    );

    submit_button.onclick = async function () {
      let comment_text = comment_input.value.trim();
      if (comment_text.length == 0) {
        //TODO show error maybe
        return;
      }
      let result = await post_comment(token, post_ident, comment_text);
      if (!("error" in result)) {
        comment_input.value = "";
        await refresh_comments_for([post_ident], token);
      }
    };
  }
});

function do_login() {
  let login_buttons = document.getElementsByClassName("login-button");
  for (let i = 0; i < login_buttons.length; ++i) {
    let item = login_buttons[i];
    item.style.display = "none";
  }
  let shoutbox_ui = document.getElementById("shoutbox-ui");
  shoutbox_ui.style.display = "flex";
  let login_modal = document.getElementById("login-modal");
  login_modal.style.display = "none";
  let token_login_modal = document.getElementById("token-login-modal");
  token_login_modal.style.display = "none";
}

function do_logout() {
  let login_buttons = document.getElementsByClassName("login-button");
  for (let i = 0; i < login_buttons.length; ++i) {
    let item = login_buttons[i];
    item.style.display = "block";
  }
  let shoutbox_ui = document.getElementById("shoutbox-ui");
  shoutbox_ui.style.display = "none";
}

async function try_login(token) {
  localStorage.setItem("logged_in", false);
  const result = await verify_token(token);
  if ("is_valid" in result) {
    let logged_in = result["is_valid"];
    localStorage.setItem("logged_in", logged_in);
    if (logged_in == true) {
      localStorage.setItem("token", token.trim());
      do_login();
      return true;
    } else {
      do_logout();
      return false;
    }
  }
}

let login_modal = document.getElementById("login-modal");
let login_close_button = document.getElementById("modal-close-button");

let login_buttons = document.getElementsByClassName("login-button");
for (let i = 0; i < login_buttons.length; ++i) {
  let item = login_buttons[i];
  bind_button_to_open_modal(item, login_modal);
}

bind_button_to_close_modal(login_close_button, login_modal);

let token_login_modal = document.getElementById("token-login-modal");
let token_login_close_button = document.getElementById(
  "token-login-modal-close-button",
);

bind_button_to_close_modal(token_login_close_button, token_login_modal);

let confirmation_modal = document.getElementById("confirmation-modal");
let confirmation_modal_close_button = document.getElementById(
  "confirmation-modal-close-button",
);
bind_button_to_close_modal(confirmation_modal_close_button, confirmation_modal);

// Handle the login button on the modal where the user is entering their token
let token_login_button = document.getElementById("token-login-button");
token_login_button.onclick = async function () {
  let token_login_input = document.getElementById("token-login-token");
  let token = token_login_input.value.trim();
  let logged_in = await try_login(token);
  if (logged_in) {
    let confirmation_modal = document.getElementById("confirmation-modal");
    let confirmation_message = document.getElementById("confirmation-message");
    confirmation_modal.style.display = "block";
    confirmation_message.innerHTML = `<p>you're logged in! the site will remember your login on this device until you clear your cookies or if you are accessing it in a private browser window.</p>`;
  }
};

// Handle the login button on the main register modal
let login_submit_button = document.getElementById("login-submit");
login_submit_button.onclick = async function () {
  let display_name_input = document.getElementById("display-name-input");
  let message_area = document.getElementById("login-messages");
  let display_name_trimmed = display_name_input.value.trim();
  const result = await register_display_name(display_name_trimmed);
  if ("error" in result) {
    let error = result["error"];
    if (error === "NAME_TAKEN") {
      let modal = document.getElementById("login-modal");
      modal.style.display = "none";
      let token_login_modal = document.getElementById("token-login-modal");

      let token_login_name = document.getElementById("token-login-name");
      token_login_name.innerHTML = display_name_trimmed;
      token_login_modal.style.display = "block";
    } else {
      message_area.innerHTML = error;
      console.log(JSON.stringify(result));
    }
  } else if ("token" in result) {
    let token = result["token"].trim();
    let logged_in = await try_login(token);
    if (logged_in) {
      let confirmation_modal = document.getElementById("confirmation-modal");
      let confirmation_message = document.getElementById(
        "confirmation-message",
      );
      confirmation_modal.style.display = "block";
      confirmation_message.innerHTML = `<p>you're logged in! the site will remember your login on this device until you clear your cookies but not if you are accessing it in a private browser window.
        if you would like to sign in with this name on another device or otherwise need to sign back in you will need the following token. think of it like a password and keep it somewhere safe. if you do lose it though just ask me for help
        </p><b>${token}</b>`;
    }
  }
};

let shout_send_button = document.getElementById("shoutbox-send");
shout_send_button.onclick = async function () {
  let shoutbox_textarea = document.getElementById("shoutbox-textarea");
  if (shoutbox_textarea.value.length == 0) {
    //TODO show error here
    return;
  }
  const token = localStorage.getItem("token");
  let shoutbox_text = shoutbox_textarea.value.trim();
  const response = await post_shout(token, shoutbox_text);
  if (!("error" in response)) {
    shoutbox_textarea.value = "";
  }
  console.log(response);
};

// Clicking your own username opens the color change popup
document.addEventListener("click", function (event) {
  if (event.target.classList.contains("shoutbox-username-editable")) {
    popup.style.display = "block";
    popup.style.left = event.pageX + "px";
    popup.style.top = event.pageY + "px";
  }
});

//Closing the color change popup by clicking elsewhere
let popup = document.getElementById("color-change-popup");
document.addEventListener("click", function (event) {
  if (
    !event.target.classList.contains("shoutbox-username-editable") &&
    !popup.contains(event.target)
  ) {
    popup.style.display = "none";
  }
});
