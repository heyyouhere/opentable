
const WINDOW_WIDTH = window.innerWidth/2;
const WINDOW_HEIGHT = window.innerHeight;
const TOKEN_SIZE = 400
const CIRCLE_STROKE = 4

function handleFileSelect(e) {
    e.preventDefault();
    var file = e.dataTransfer.files[0];
    var reader = new FileReader();
    reader.onload = function() {
        var img = new Image();
        img.onload = function() {
            ctx.drawImage(img, 500 / 2 - img.width / 2, 500 / 2 - img.height / 2);
        };
    };
    reader.readAsDataURL(file);
}
function handleDragOver(e) {
    e.preventDefault();
}

function save_image(stage){
    document.getElementById('save').onclick = () => {
        const fileName = 'my_image.png'; // Replace 'my_image.png' with your desired file name
        const link = document.createElement('a');
        link.href = stage.toDataURL({
            x: WINDOW_WIDTH/2 - TOKEN_SIZE/2,
            y: WINDOW_HEIGHT/2 - TOKEN_SIZE/2,
            width: TOKEN_SIZE,
            height: TOKEN_SIZE,
        });
        link.download = fileName;
        link.click();
    }
}






window.onload = () => {

    var stage = new Konva.Stage({
        container: 'container',
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
    });
    console.log(stage)
    var preview_stage = new Konva.Stage({
        container: 'preview_container',
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
    });


    var layer = new Konva.Layer();
    var borders_layer = new Konva.Layer();
    stage.add(layer);
    stage.add(borders_layer);

    var circle = new Konva.Circle({
        x: stage.width() / 2,
        y: stage.height() / 2,
        radius: TOKEN_SIZE/2,
        stroke: border_color_picker.value,
        strokeWidth: CIRCLE_STROKE,
        listening : false,
    });


    borders_layer.add(circle);
    var imageObj = new Image();
    var tr = new Konva.Transformer();
    layer.add(tr);
    imageObj.src = 'hero.png';
    imageObj.onload = () => {
        let target_image_size = {
            width : 0,
            height : 0,
        }
        if (imageObj.naturalWidth > imageObj.naturalHeight) {
            target_image_size.height = TOKEN_SIZE
            target_image_size.width = imageObj.naturalWidth * TOKEN_SIZE/imageObj.naturalHeight
        } else {
            target_image_size.width = TOKEN_SIZE
            target_image_size.height = imageObj.naturalHeight * TOKEN_SIZE/imageObj.naturalWidth
        }


        var hero = new Konva.Image({
            x: WINDOW_WIDTH/2 - target_image_size.width/2,
            y: WINDOW_HEIGHT/2 - target_image_size.height/2, 
            image: imageObj,
            width: target_image_size.width,
            height: target_image_size.height,
            draggable: true,
        });

        layer.add(hero);
        tr.nodes([hero]);

        var hero_preview = new Konva.Image({
            x: WINDOW_WIDTH/2 - target_image_size.width/2,
            y: WINDOW_HEIGHT/2 - target_image_size.height/2, 
            image: imageObj,
            width: target_image_size.width,
            height: target_image_size.height,
        });
        var group = new Konva.Group({
            clipFunc: function (ctx) {
                ctx.arc(stage.width()/2, stage.height()/2, TOKEN_SIZE/2, 0, Math.PI * 2, false);
                ctx.arc(stage.width()/2, stage.height()/2, TOKEN_SIZE/2, 0, Math.PI * 2, false);
            },
        });


        var preview_layer = new Konva.Layer();
        preview_stage.add(preview_layer);


        var preview_circle = new Konva.Circle({
            x: stage.width() / 2,
            y: stage.height() / 2,
            radius: 0.99 * TOKEN_SIZE/2,
            stroke:  border_color_picker.value,
            strokeWidth: CIRCLE_STROKE,
        });
        var preview_background_circle = new Konva.Circle({
            x: stage.width() / 2,
            y: stage.height() / 2,
            radius:TOKEN_SIZE/2,
            fill:  background_color_picker.value,
            strokeWidth: CIRCLE_STROKE,
        });


        group.add(preview_background_circle)
        group.add(hero_preview)
        group.add(preview_circle)
        preview_layer.add(group)

        hero.on('dragmove', function() {
            hero_preview.position(hero.position())
            preview_layer.batchDraw()
        })
        hero.on('transform', function() {
            hero_preview.scale(hero.scale())
            preview_layer.batchDraw()
        })
        border_color_picker.onchange = () => {
            circle.stroke(border_color_picker.value)
            preview_circle.stroke(border_color_picker.value)
        }
        background_color_picker.onchange = () => {
            preview_background_circle.fill(background_color_picker.value)
        }






        save_image(preview_stage)
    }
}
