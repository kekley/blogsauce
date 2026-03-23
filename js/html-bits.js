export function generate_shout_html(shout) {
  let username_class = "shoutbox-username";
  if (shout.editable === true) {
    username_class += " shoutbox-username-editable";
  }
  return `<li class="shoutbox-message">
    <p><span style="color:${shout.user_color};"class="${username_class}">${shout.display_name}</span>:${shout.content}</p>
    </li>`;
}

export function generate_comment_html(comment, timezone_offset) {
  let edited_or_created = "Created";
  if (comment.edited) {
    edited_or_created = "Edited";
  }

  let edit_buttons = "";
  if (comment.editable) {
    edit_buttons = `
        <div style="display:flex;flex-direction:row;gap:1mm;">
            <div class="comment-edit-button">
                <button id="comment-edit-${comment.id}">Edit</button>
            </div>
            <div class="comment-edit-button">
                <button id="comment-delete-${comment.id}">Delete</button>
            </div>
          </div>
        `;
  }

  let date = new Date(comment.created);
  date.setMinutes(date.getMinutes() - timezone_offset);
  let username_class = "shoutbox-username";
  if (comment.editable === true) {
    username_class += " shoutbox-username-editable";
  }

  return `<li id="comment-${comment.id}" class="comment-container">
        <div class="comment-top-bar">
            <div class="comment-header">
              <div class="comment-time">${edited_or_created} at ${date.toLocaleString()}</div>
              <div class="comment-username">
                  <span style="color:${comment.user_color};"class="${username_class}">${comment.display_name}</span>
              </div>
            </div>
            ${edit_buttons}
        </div>
        <div class="comment-content">${comment.content}</div>
      </li>`;
}
