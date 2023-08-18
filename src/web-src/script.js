let current_game;
function load_maps(game){
    let maps_container = document.getElementById("maps_container")
    while (maps_container.hasChildNodes()) {
        maps_container.removeChild(maps_container.lastChild);
    }
    game.maps.forEach(map => {
        let map_div = document.createElement('div')
        map_div.innerText = map
        maps_container.appendChild(map_div)
    })
}
function load_messages(game){
    let chat_container = document.getElementById("chat_container")
    while (chat_container.hasChildNodes()) {
        chat_container.removeChild(chat_container.lastChild);
    }
    game.actions.forEach(action => {
        console.log(action);
        let chat_div = document.createElement('div')
        chat_div.innerText = action.content;
        chat_container.appendChild(chat_div)
    })

}
function load_games() {
    let games_containter = document.getElementById("games_container");
    while (games_containter.hasChildNodes()) {
        games_containter.removeChild(games_containter.lastChild);
    }
    var xhttp = new XMLHttpRequest();
    xhttp.open("GET", "http://77.232.23.43:1583/api/games");
    xhttp.setRequestHeader('Content-Type', 'application/x-www-form-urlencode');
    xhttp.send();
    xhttp.onload = () => {
        let games = JSON.parse(xhttp.response)
        games.forEach(game => {
            let game_div = document.createElement('div')
            game_div.className = 'Game'
            game_div.innerText = game.name
            games_containter.appendChild(game_div)
            game_div.onclick = () => {
                current_game = game;
                document.title = current_game.name + ' | Opentable'
                load_maps(game);
                load_messages(game);
            }
        });
    };
}
load_games();
document.getElementById("reload_button").onclick = () => {
    load_games();
}

document.getElementById("new_game_button").onclick = () => {
    let new_game_name = document.getElementById("new_game_name").value;
    var xhttp = new XMLHttpRequest();
    xhttp.open("POST", "http://77.232.23.43:1583/api/games");
    xhttp.setRequestHeader('Content-Type', 'application/x-www-form-urlencode');
    xhttp.send(new_game_name);
    xhttp.onload = () => {
        load_games();
    }
}






// document.getElementById("button").onclick = () => {
//     var xhttp = new XMLHttpRequest();
//     xhttp.open("GET", "http://77.232.23.43:1583/api/pdfs");
//     xhttp.setRequestHeader('Content-Type', 'application/x-www-form-urlencode');
//     xhttp.send();
//     xhttp.onload = () => {
//         console.log(xhttp.responseText)
//         show_games(xhttp.responseText)
//     };
// }

// function show_games(data) {
//     let games = JSON.parse(data);
//     console.log(games);
//     games.forEach(game => {
//         let pdf_button = document.createElement('button')
//         pdf_button.className = "PDFButton";
//         let filename = game.split('/').at(-1);
//         pdf_button.innerText = filename;
//         document.getElementById('pdfs').appendChild(pdf_button)
//         pdf_button.onclick = () => {
//             let pdfUrl = "http://77.232.23.43:1583/assets/" + filename;
//             PDFObject.embed(pdfUrl, '#example1'); // you can add {page : "100"} to open pdf on that page
//         }
//     })
// }
