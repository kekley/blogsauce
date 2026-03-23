import { get_splash, verify_token } from "./api.js";
import {
  get_shouts,
  post_shout,
  change_user_color,
  subscribe_shouts,
} from "./api.js";
import { generate_shout_html, generate_comment_html } from "./html-bits.js";
import {
  bind_button_to_open_modal,
  set_up_modal_close_button,
} from "./modal.js";

import { register_display_name } from "./api.js";

import { get_comments, post_comment } from "./api.js";

function do_login() {
  const login_buttons = document.getElementsByClassName("login-button");
  for (const button of login_buttons) {
    button.style.display = "none";
  }
  const shoutbox_ui = document.getElementById("shoutbox-ui");
  shoutbox_ui.style.display = "flex";
  const login_modal = document.getElementById("login-modal");
  login_modal.style.display = "none";
  const token_login_modal = document.getElementById("token-login-modal");
  token_login_modal.style.display = "none";
}

function do_logout() {
  const login_buttons = document.getElementsByClassName("login-button");
  for (const button of login_buttons) {
    button.style.display = "block";
  }
  const shoutbox_ui = document.getElementById("shoutbox-ui");
  shoutbox_ui.style.display = "none";
}

async function try_login(token) {
  localStorage.setItem("logged_in", false);
  const result = await verify_token(token);
  if ("is_valid" in result) {
    const logged_in = result["is_valid"];
    localStorage.setItem("logged_in", logged_in);
    if (logged_in == true) {
      localStorage.setItem("token", token.trim());
      localStorage.setItem("display_name", result.display_name);
      localStorage.setItem("current_color", result.current_color);

      do_login();
      return true;
    } else {
      do_logout();
      return false;
    }
  }
}

async function init_shoutbox() {
  //Populate the shoutbox messages
  const token = localStorage.getItem("token");
  const response = await get_shouts(null, token);
  console.log(response);
  const shout_list = document.getElementById("shoutbox-messages");
  shout_list.innerHTML = "";
  const shouts_array = response["shouts"];
  const scrollArea = document.getElementById("shoutbox-scroll");
  for (const shout of shouts_array) {
    let shout_html = generate_shout_html(shout);
    const shout_template = document.createElement("template");
    shout_template.innerHTML = shout_html;
    shout_list.append(shout_template.content);
  }
  scrollArea.scrollTop = scrollArea.scrollHeight;

  //Set up SSE callback
  const shout_callback = function (shout) {
    const shout_list = document.getElementById("shoutbox-messages");
    const scrollArea = document.getElementById("shoutbox-scroll");
    const should_scroll_bottom =
      scrollArea.scrollTop + scrollArea.clientHeight >=
      scrollArea.scrollHeight - 1;
    const shout_html = generate_shout_html(shout);
    const shout_template = document.createElement("template");
    shout_template.innerHTML = shout_html;
    shout_list.append(shout_template.content);
    if (should_scroll_bottom) {
      scrollArea.scrollTop = scrollArea.scrollHeight;
    }
  };

  subscribe_shouts(shout_callback);

  const shout_send_button = document.getElementById("shoutbox-send");
  shout_send_button.onclick = async function () {
    const shoutbox_textarea = document.getElementById("shoutbox-textarea");
    if (shoutbox_textarea.value.length == 0) {
      //TODO show error here
      return;
    }
    const token = localStorage.getItem("token");
    const shoutbox_text = shoutbox_textarea.value.trim();
    shout_send_button.disabled = true;
    const response = await post_shout(token, shoutbox_text);
    shout_send_button.disabled = false;
    if (!("error" in response)) {
      shoutbox_textarea.value = "";
    }
    console.log(response);
  };
}

async function init_comments() {
  const post_list = JSON.parse(
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
      const token = localStorage.getItem("token");
      const comment_text = comment_input.value.trim();
      if (comment_text.length == 0) {
        //TODO show error maybe
        return;
      }
      submit_button.disabled = true;
      const response = await post_comment(token, post_ident, comment_text);
      submit_button.disabled = false;

      if (!("error" in response)) {
        comment_input.value = "";
        await refresh_comments_for([post_ident], token);
      }
    };
  }
}

async function refresh_comments_for(post_list) {
  const token = localStorage.getItem("token");
  const response = await get_comments(post_list, token);
  const timezone_offset = new Date().getTimezoneOffset();
  for (const post of response.posts) {
    const post_comments_list = document.getElementById(
      `comments-list-${post.post_ident}`,
    );
    post_comments_list.innerHTML = "";
    if ("comments" in post) {
      for (const comment of post.comments) {
        const comment_html = generate_comment_html(comment, timezone_offset);
        const comment_template = document.createElement("template");
        comment_template.innerHTML = comment_html;
        post_comments_list.append(comment_template.content);
      }
    }
  }
}
function set_up_login_modals() {
  const login_modal = document.getElementById("login-modal");
  set_up_modal_close_button(login_modal);

  const login_buttons = document.getElementsByClassName("login-button");
  for (const button of login_buttons) {
    bind_button_to_open_modal(button, login_modal);
  }

  const token_login_modal = document.getElementById("token-login-modal");
  set_up_modal_close_button(token_login_modal);

  const confirmation_modal = document.getElementById("confirmation-modal");
  set_up_modal_close_button(confirmation_modal);

  // set up the login button on the modal where the user is entering their token
  const token_login_button = document.getElementById("token-login-button");
  token_login_button.onclick = async function () {
    const token_login_input = document.getElementById("token-login-token");
    const token = token_login_input.value.trim();
    const logged_in = await try_login(token);
    if (logged_in) {
      const confirmation_modal = document.getElementById("confirmation-modal");
      const confirmation_message = document.getElementById(
        "confirmation-message",
      );
      confirmation_modal.style.display = "block";
      confirmation_message.innerHTML = `<p>you're logged in! the site will remember your login on this device until you clear your cookies or if you are accessing it in a private browser window.</p>`;
    }
  };

  // Handle the login button on the main register modal
  const login_submit_button = document.getElementById("login-submit");
  login_submit_button.onclick = async function () {
    const display_name_input = document.getElementById("display-name-input");
    const message_area = document.getElementById("login-messages");
    const display_name_trimmed = display_name_input.value.trim();
    const result = await register_display_name(display_name_trimmed);
    if ("error" in result) {
      const error = result["error"];
      if (error === "NAME_TAKEN") {
        const modal = document.getElementById("login-modal");
        modal.style.display = "none";
        const token_login_modal = document.getElementById("token-login-modal");

        const token_login_name = document.getElementById("token-login-name");
        token_login_name.innerHTML = display_name_trimmed;
        token_login_modal.style.display = "block";
      } else {
        message_area.innerHTML = error;
        console.log(JSON.stringify(result));
      }
    } else if ("token" in result) {
      const token = result["token"].trim();
      const logged_in = await try_login(token);
      if (logged_in) {
        const confirmation_modal =
          document.getElementById("confirmation-modal");
        const confirmation_message = document.getElementById(
          "confirmation-message",
        );
        confirmation_modal.style.display = "block";
        confirmation_message.innerHTML = `<p>you're logged in! the site will remember your login on this device until you clear your cookies but not if you are accessing it in a private browser window.
        if you would like to sign in with this name on another device or otherwise need to sign back in you will need the following token. think of it like a password and keep it somewhere safe. if you do lose it though just ask me for help
        </p><b>${token}</b>`;
      }
    }
  };
}

function set_up_color_change_popup() {
  // Clicking your own username opens the color change popup
  document.addEventListener("click", function (event) {
    if (event.target.classList.contains("shoutbox-username-editable")) {
      popup.style.display = "block";
      popup.style.left = event.pageX + "px";
      popup.style.top = event.pageY + "px";
    }
  });

  //Closing the color change popup by clicking elsewhere
  const popup = document.getElementById("color-change-popup");
  document.addEventListener("click", function (event) {
    if (
      !event.target.classList.contains("shoutbox-username-editable") &&
      !popup.contains(event.target)
    ) {
      popup.style.display = "none";
    }
  });
  const submit_button = document.getElementById("change-color-button");
  const color_input = document.getElementById("new-name-color");
  const current_color = localStorage.getItem("current_color");
  if (current_color) {
    color_input.value = current_color;
  }

  submit_button.onclick = async function () {
    const token = localStorage.getItem("token");
    const color = color_input.value;
    const response = await change_user_color(token, color);
    if (!("error" in response)) {
      const display_names_to_change = document.getElementsByClassName(
        "shoutbox-username-editable",
      );
      localStorage.setItem("current_color", color);
      for (const name of display_names_to_change) {
        name.style.color = color;
      }
    }
  };
}

document.addEventListener("DOMContentLoaded", async function () {
  document.title = await get_splash();
  const maybe_token = localStorage.getItem("token");
  if (maybe_token && maybe_token.length == 32) {
    await try_login(maybe_token);
  } else {
    do_logout();
  }
  await init_shoutbox();
  await init_comments();
  set_up_login_modals();
  set_up_color_change_popup();
});
