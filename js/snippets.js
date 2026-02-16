export function generate_shout_html(shout,element){
    let username_class = "shoutbox-username";
    if(shout.editable===true){
        username_class +=" shoutbox-username-editable";
    }
return `<div class="shoutbox-message">
    <p><span style="color:${shout.user_color};"class="${username_class}">${shout.display_name}</span>:${shout.content}</p>
    </div>`;

}
