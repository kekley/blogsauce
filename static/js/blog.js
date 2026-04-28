(() => {
  // js/api.js
  var API_URL = "http://127.0.0.1:3000";
  async function get_splash() {
    const location = API_URL + `/getSplash`;
    const response = await fetch(location, {
      method: "GET",
      mode: "cors"
    });
    const body = await response.text();
    const response_json = JSON.parse(body);
    if ("splash" in response_json) {
      return response_json["splash"];
    }
    return "";
  }
  async function verify_token(token) {
    const info = {
      token
    };
    const location = API_URL + `/verifyToken`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        return {
          error: `Response status: ${response.status}`
        };
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }
  async function get_comments(post_list, token) {
    const info = {
      post_idents: post_list,
      token
    };
    const location = API_URL + `/getComments`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }
  async function register_display_name(display_name) {
    if (display_name.length == 0) {
      return {
        error: "username cannot be empty"
      };
    }
    const info = {
      display_name
    };
    const location = API_URL + `/registerName`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }
  async function change_user_color(token, color) {
    const info = {
      token,
      color
    };
    const location = API_URL + `/changeColor`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }
  async function edit_comment(comment_id, content, token) {
    const info = {
      comment_id,
      token,
      content
    };
    const location = API_URL + `/editComment`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }
  async function delete_comment(comment_id, token) {
    const info = {
      comment_id,
      token
    };
    const location = API_URL + `/deleteComment`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }
  async function post_comment(token, post, content) {
    const info = {
      token,
      post,
      content
    };
    const location = API_URL + `/postComment`;
    try {
      const response = await fetch(location, {
        method: "POST",
        mode: "cors",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(info)
      });
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }
  function subscribe_shouts(callback, token) {
    let token_param = "";
    if (token != void 0 && token != null) {
      token_param = `?token=${token}`;
    }
    const es = new EventSource(API_URL + `/subscribeShouts${token_param}`);
    es.onopen = () => {
      console.log("SSE connected");
    };
    es.onmessage = (ev) => {
      callback(JSON.parse(ev.data));
    };
    es.onerror = () => {
      console.log(
        "SSE state:",
        es.readyState === EventSource.CONNECTING ? "reconnecting" : "closed"
      );
    };
  }
  async function get_shouts(shouts_before_id, token) {
    let info = {};
    if (shouts_before_id != void 0 && shouts_before_id != null) {
      info["shouts_before_id"] = shouts_before_id;
    }
    if (token != void 0 && token != null) {
      info["token"] = token;
    }
    const location = API_URL + `/getShouts`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
    }
  }
  async function post_shout(token, content) {
    const info = {
      token,
      content
    };
    const location = API_URL + `/postShout`;
    const response = await fetch(location, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(info)
    });
    try {
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const response_json = await response.json();
      return response_json;
    } catch (e) {
      console.log(e);
      return {
        error: `${e}`
      };
    }
  }

  // js/html-bits.js
  function generate_shout_element(shout) {
    const shout_template = document.getElementById("shout-template");
    const new_shout = document.importNode(shout_template.content, true);
    const username = new_shout.querySelector(`#shout-username-template`);
    const content = new_shout.querySelector(`#shout-content-template`);
    new_shout.querySelector("li").id = `shout${shout.id}`;
    username.removeAttribute("id");
    content.removeAttribute("id");
    content.innerHTML = shout.content;
    username.innerHTML = shout.user_display_name;
    username.style = `color:${shout.user_color};`;
    if (shout.editable) {
      username.classList.add("username-span-editable");
    }
    return new_shout;
  }
  function generate_comment_element(comment, timezone_offset) {
    const comment_template = document.getElementById("comment-template");
    const new_comment = document.importNode(comment_template.content, true);
    new_comment.querySelector("li").id = `comment-container${comment.id}`;
    const comment_content = new_comment.querySelector(
      "#comment-content-template"
    );
    const comment_time = new_comment.querySelector("#comment-time-template");
    comment_time.removeAttribute("id");
    const comment_username = new_comment.querySelector("#username-template");
    comment_username.removeAttribute("id");
    comment_username.style = `color:${comment.user_color};`;
    let edited_or_created = "Created";
    if (comment.edited) {
      edited_or_created = "Edited";
    }
    const edit_buttons = new_comment.querySelector(
      "#comment-edit-buttons-template"
    );
    edit_buttons.removeAttribute("id");
    if (!comment.editable) {
      edit_buttons.remove();
    } else {
      const edit_button = new_comment.querySelector(
        "#comment-edit-button-template"
      );
      edit_button.id = `comment-edit-button${comment.id}`;
      const delete_button = new_comment.querySelector(
        "#comment-delete-button-template"
      );
      delete_button.id = `comment-delete-button${comment.id}`;
      comment_username.classList.add("username-span-editable");
    }
    comment_content.innerHTML = comment.content;
    comment_content.id = `comment-content${comment.id}`;
    let date = new Date(comment.created);
    date.setMinutes(date.getMinutes() - timezone_offset);
    comment_time.innerHTML = `${edited_or_created} at ${date.toLocaleString()}`;
    comment_username.innerHTML = comment.display_name;
    return new_comment;
  }

  // js/modal.js
  function bind_button_to_open_modal(button, modal) {
    button.onclick = function() {
      modal.style.display = "block";
    };
  }
  function set_up_modal_close_button(modal) {
    window.addEventListener("click", function(event) {
      if (event.target == modal) {
        modal.style.display = "none";
      }
    });
    const close_button = modal.querySelector(".modal-close");
    close_button.onclick = function() {
      modal.style.display = "none";
    };
  }

  // js/main.js
  function do_login() {
    const login_buttons = document.getElementsByClassName("login-container");
    for (const button of login_buttons) {
      button.hidden = true;
    }
    const elements_to_show = document.getElementsByClassName("show-on-login");
    for (const element of elements_to_show) {
      element.hidden = false;
    }
    const login_modal = document.getElementById("login-modal");
    login_modal.style.display = "none";
    const token_login_modal = document.getElementById("token-login-modal");
    token_login_modal.style.display = "none";
  }
  function do_logout() {
    const login_buttons = document.getElementsByClassName("login-container");
    for (const button of login_buttons) {
      button.hidden = false;
    }
    const elements_to_hide = document.getElementsByClassName("show-on-login");
    for (const element of elements_to_hide) {
      element.hidden = true;
    }
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
    const token = localStorage.getItem("token");
    const response = await get_shouts(null, token);
    const shout_list = document.getElementById("shoutbox-messages");
    shout_list.innerHTML = "";
    const shouts_array = response["shouts"];
    const scrollArea = document.getElementById("shoutbox-scroll");
    for (const shout of shouts_array) {
      let shout_element = generate_shout_element(shout);
      shout_list.append(shout_element);
    }
    scrollArea.scrollTop = scrollArea.scrollHeight;
    const shout_callback = function(shout) {
      const shout_list2 = document.getElementById("shoutbox-messages");
      const scrollArea2 = document.getElementById("shoutbox-scroll");
      const should_scroll_bottom = scrollArea2.scrollTop + scrollArea2.clientHeight >= scrollArea2.scrollHeight - 1;
      const shout_element = generate_shout_element(shout);
      shout_list2.append(shout_element);
      if (should_scroll_bottom) {
        scrollArea2.scrollTop = scrollArea2.scrollHeight;
      }
    };
    subscribe_shouts(shout_callback, token);
    const shout_send_button = document.getElementById("shoutbox-send");
    shout_send_button.onclick = async function() {
      const shoutbox_textarea = document.getElementById("shoutbox-textarea");
      if (shoutbox_textarea.value.length == 0) {
        return;
      }
      const token2 = localStorage.getItem("token");
      const shoutbox_text = shoutbox_textarea.value.trim();
      shout_send_button.disabled = true;
      const response2 = await post_shout(token2, shoutbox_text);
      shout_send_button.disabled = false;
      if (!("error" in response2)) {
        shoutbox_textarea.value = "";
      }
    };
  }
  async function init_comments() {
    const post_list = JSON.parse(
      document.getElementById("visible-posts-list").dataset.posts
    );
    await refresh_comments_for(post_list.post_idents);
    for (const post_ident of post_list.post_idents) {
      const submit_button = document.getElementById(
        `comment-submit-${post_ident}`
      );
      const comment_input = document.getElementById(
        `comment-input-${post_ident}`
      );
      submit_button.onclick = async function() {
        const token = localStorage.getItem("token");
        const comment_text = comment_input.value.trim();
        if (comment_text.length == 0) {
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
  function on_comment_edit(comment_id) {
    const comment_edit_button = document.getElementById(
      `comment-edit-button${comment_id}`
    );
    const comment_delete_button = document.getElementById(
      `comment-delete-button${comment_id}`
    );
    const comment_content = document.getElementById(
      `comment-content${comment_id}`
    );
    const old_content = comment_content.textContent;
    const text_edit_template = document.getElementById(
      "comment-text-box-template"
    );
    const new_text_edit = document.importNode(text_edit_template.content, true);
    const text_area = new_text_edit.querySelector("#comment-edit-input-template");
    text_area.id = `comment-edit-input${comment_id}`;
    text_area.value = old_content;
    comment_content.replaceChildren(new_text_edit);
    comment_edit_button.innerHTML = "Save";
    comment_delete_button.innerHTML = "Cancel";
    comment_edit_button.onclick = async function() {
      on_comment_save(comment_id);
    };
    comment_delete_button.onclick = async function() {
      on_comment_edit_cancel(comment_id, old_content);
    };
  }
  async function on_comment_delete(comment_id) {
    if (confirm("Are you sure you want to delete this comment?")) {
      const token = localStorage.getItem("token");
      const response = await delete_comment(comment_id, token);
      if (!("error" in response)) {
        const comment_container = document.querySelector(
          `#comment-container${comment_id}`
        );
        comment_container.remove();
      }
    }
  }
  async function on_comment_save(comment_id) {
    const comment_edit_button = document.getElementById(
      `comment-edit-button${comment_id}`
    );
    const comment_delete_button = document.getElementById(
      `comment-delete-button${comment_id}`
    );
    const comment_content = document.getElementById(
      `comment-content${comment_id}`
    );
    const content = document.getElementById(`comment-edit-input${comment_id}`).value.trim();
    if (content === "") {
      return;
    }
    const token = localStorage.getItem("token");
    const response = await edit_comment(comment_id, content, token);
    if (!("error" in response)) {
      comment_edit_button.innerHTML = "Edit";
      comment_edit_button.onclick = function() {
        on_comment_edit(comment_id);
      };
      comment_delete_button.innerHTML = "Delete";
      comment_delete_button.onclick = async function() {
        on_comment_delete(comment_id);
      };
      comment_content.innerHTML = content;
    }
  }
  function on_comment_edit_cancel(comment_id, old_content) {
    const comment_edit_button = document.getElementById(
      `comment-edit-button${comment_id}`
    );
    const comment_delete_button = document.getElementById(
      `comment-delete-button${comment_id}`
    );
    const comment_content = document.getElementById(
      `comment-content${comment_id}`
    );
    comment_content.innerHTML = old_content;
    comment_edit_button.innerHTML = "Edit";
    comment_edit_button.onclick = function() {
      on_comment_edit(comment_id);
    };
    comment_delete_button.innerHTML = "Delete";
    comment_delete_button.onclick = async function() {
      on_comment_delete(comment_id);
    };
  }
  async function refresh_comments_for(post_list) {
    const token = localStorage.getItem("token");
    const response = await get_comments(post_list, token);
    const timezone_offset = (/* @__PURE__ */ new Date()).getTimezoneOffset();
    for (const post of response.posts) {
      const post_comments_list = document.getElementById(
        `comments-list-${post.post_ident}`
      );
      post_comments_list.innerHTML = "";
      if ("comments" in post) {
        for (const comment of post.comments) {
          const comment_id = comment.id;
          const comment_element = generate_comment_element(
            comment,
            timezone_offset
          );
          if (comment.editable) {
            const comment_edit_button = comment_element.querySelector(
              `#comment-edit-button${comment_id}`
            );
            const comment_delete_button = comment_element.querySelector(
              `#comment-delete-button${comment_id}`
            );
            comment_edit_button.onclick = function() {
              on_comment_edit(comment_id);
            };
            comment_delete_button.onclick = function() {
              on_comment_delete(comment_id);
            };
          }
          post_comments_list.append(comment_element);
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
    const token_login_button = document.getElementById("token-login-button");
    token_login_button.onclick = async function() {
      const token_login_input = document.getElementById("token-login-token");
      const token = token_login_input.value.trim();
      const logged_in = await try_login(token);
      if (logged_in) {
        const confirmation_modal2 = document.getElementById("confirmation-modal");
        const confirmation_message = document.getElementById(
          "confirmation-message"
        );
        confirmation_modal2.style.display = "block";
        confirmation_message.innerHTML = `<p>you're logged in! the site will remember your login on this device until you clear your cookies or if you are accessing it in a private browser window.</p>`;
      }
    };
    const login_submit_button = document.getElementById("login-submit");
    login_submit_button.onclick = async function() {
      const display_name_input = document.getElementById("display-name-input");
      const message_area = document.getElementById("login-messages");
      const display_name_trimmed = display_name_input.value.trim();
      const result = await register_display_name(display_name_trimmed);
      if ("error" in result) {
        const error = result["error"];
        if (error === "USER_TAKEN") {
          const modal = document.getElementById("login-modal");
          modal.style.display = "none";
          const token_login_modal2 = document.getElementById("token-login-modal");
          const token_login_name = document.getElementById("token-login-name");
          token_login_name.innerHTML = display_name_trimmed;
          token_login_modal2.style.display = "block";
        } else {
          message_area.innerHTML = error;
        }
      } else if ("token" in result) {
        const token = result["token"].trim();
        const logged_in = await try_login(token);
        if (logged_in) {
          const confirmation_modal2 = document.getElementById("confirmation-modal");
          const confirmation_message = document.getElementById(
            "confirmation-message"
          );
          confirmation_modal2.style.display = "block";
          confirmation_message.innerHTML = `<p>you're logged in! the site will remember your login on this device until you clear your cookies but not if you are accessing it in a private browser window.
        if you would like to sign in with this name on another device or otherwise need to sign back in you will need the following token. think of it like a password and keep it somewhere safe. if you do lose it though just ask me for help
        </p><b>${token}</b>`;
        }
      }
    };
  }
  function set_up_color_change_popup() {
    document.addEventListener("click", function(event) {
      if (event.target.classList.contains("username-span-editable")) {
        popup.style.display = "block";
        popup.style.left = event.pageX + "px";
        popup.style.top = event.pageY + "px";
      }
    });
    const popup = document.getElementById("color-change-popup");
    document.addEventListener("click", function(event) {
      if (!event.target.classList.contains("username-span-editable") && !popup.contains(event.target)) {
        popup.style.display = "none";
      }
    });
    const submit_button = document.getElementById("change-color-button");
    const color_input = document.getElementById("new-name-color");
    const current_color = localStorage.getItem("current_color");
    if (current_color) {
      color_input.value = current_color;
    }
    submit_button.onclick = async function() {
      const token = localStorage.getItem("token");
      const color = color_input.value;
      const response = await change_user_color(token, color);
      if (!("error" in response)) {
        const display_names_to_change = document.getElementsByClassName(
          "username-span-editable"
        );
        localStorage.setItem("current_color", color);
        for (const name of display_names_to_change) {
          name.style.color = color;
        }
      }
    };
  }
  document.addEventListener("DOMContentLoaded", async function() {
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
})();
