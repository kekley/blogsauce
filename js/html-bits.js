export function generate_shout_html(shout) {
  let username_class = "shoutbox-username";
  if (shout.editable === true) {
    username_class += " shoutbox-username-editable";
  }
  return `<div class="shoutbox-message">
    <p><span style="color:${shout.user_color};"class="${username_class}">${shout.display_name}</span>:${shout.content}</p>
    </div>`;
}

export function generate_comment_html(comment, timezone_offset) {
  let edited_or_created = "Created";
  if (comment.edited) {
    edited_or_created = "Edited";
  }
  let date = new Date(comment.created);
  date.setMinutes(date.getMinutes() - timezone_offset);
  let username_class = "shoutbox-username";
  if (comment.editable === true) {
    username_class += " shoutbox-username-editable";
  }

  return `<li id="comment-${comment.id}" class="comment-container">
        <div class="comment-header">
          <div class="comment-time">${edited_or_created} at ${date.toLocaleString()}</div>
          <div class="comment-username">
          <span style="color:${comment.user_color};"class="${username_class}">${comment.display_name}</span>
          </div>
        </div>
        <div class="comment-content">${comment.content}</div>
      </li>`;
}
