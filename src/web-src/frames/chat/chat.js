const COUNT = 2;
let message = document.getElementById("message")
let parent = document.getElementById("messages_container")
for (let i = 0; i < COUNT; i++){
    parent.appendChild(message.cloneNode(true));
}
function post_to_server(update){
    console.log("update");
}

function send_message(){
    if (send_input.value != ""){
        let new_message = message.cloneNode(true);
        new_message.childNodes[1].innerText = "heyyouhere";
        new_message.childNodes[3].innerText = send_input.value;
        send_input.value = "";
        console.log(new_message.childNodes);

        parent.appendChild(new_message);
        parent.scrollIntoView();
        send_input.focus();
    }
}

parent.scrollIntoView();
let send_button = document.getElementById('send_button');
let send_input = document.getElementById('input');
send_button.onclick = send_message 
