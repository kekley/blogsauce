export function generate_shout_html(shout){
return `<div class="shoutbox-message">
    <span style="color:${shout.user_color};"class="shoutbox-username">${shout.display_name}</span>:${shout.content}
    </div>`;

}
