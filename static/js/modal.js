
function show_modal(){
    this.style.display = "block";
}

function hide_modal(){
    this.style.display="none";
}

export function bind_button_to_open_modal(button,modal){
    var bound_fn = show_modal.bind(modal);

    button.onclick = bound_fn;
}

export function bind_button_to_close_modal(button,modal){
    var bound_fn = hide_modal.bind(modal);
    window.onclick = function(event) {
      if (event.target == modal) {
        modal.style.display = "none";
      }
}
    button.onclick = bound_fn;
}

