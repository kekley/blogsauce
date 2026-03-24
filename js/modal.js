export function bind_button_to_open_modal(button, modal) {
  button.onclick = function () {
    modal.style.display = "block";
  };
}

export function set_up_modal_close_button(modal) {
  window.addEventListener("click", function (event) {
    if (event.target == modal) {
      modal.style.display = "none";
    }
  });

  const close_button = modal.querySelector(".modal-close");
  close_button.onclick = function () {
    modal.style.display = "none";
  };
}
