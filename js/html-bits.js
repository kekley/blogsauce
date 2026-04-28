export function generate_shout_element(shout) {
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

export function generate_comment_element(comment, timezone_offset) {
    const comment_template = document.getElementById("comment-template");
    const new_comment = document.importNode(comment_template.content, true);
    new_comment.querySelector("li").id = `comment-container${comment.id}`;
    const comment_content = new_comment.querySelector(
        "#comment-content-template",
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
        "#comment-edit-buttons-template",
    );
    edit_buttons.removeAttribute("id");

    if (!comment.editable) {
        edit_buttons.remove();
    } else {
        const edit_button = new_comment.querySelector(
            "#comment-edit-button-template",
        );
        edit_button.id = `comment-edit-button${comment.id}`;
        const delete_button = new_comment.querySelector(
            "#comment-delete-button-template",
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
