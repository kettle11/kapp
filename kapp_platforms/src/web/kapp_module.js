function get_canvas_size(memory_buffer, output) {
    let canvas = document
        .getElementById("canvas");
    console.log(canvas);

    let context = canvas.getContext('2d');
    context.fillRect(0, 0, canvas.width, canvas.height);

    let width = canvas.clientWidth;
    let height = canvas.clientHeight;
    let data = new Float32Array(memory_buffer, output, 2);
    data[0] = width;
    data[1] = height;
    console.log(data);
}

let request_animation_frame_client_callback = undefined;
let pointer_moved_callback = undefined;
let pointer_down_callback = undefined;
let pointer_up_callback = undefined;
let key_down_callback = undefined;
let key_up_callback = undefined;
let scroll_callback = undefined;
let key_repeat_callback = undefined;
let character_received_callback = undefined;
let pinch_callback = undefined;

let canvas = document
    .getElementById("canvas");

function get_pointer_type(event) {
    switch (event.pointerType) {
        case "mouse": return 1
        case "pen": return 2
        case "touch": return 3
        default:
            return 0
    }
}

canvas.onpointermove = function (event) {
    let pointer_type = get_pointer_type(event);
    call_client_function_f64_4(pointer_moved_callback, event.clientX, event.clientY, pointer_type, event.timeStamp);
}

canvas.onpointerdown = function (event) {
    let pointer_type = get_pointer_type(event);
    call_client_function_f64_5(pointer_down_callback, event.clientX, event.clientY, pointer_type, event.button, event.timeStamp);
}

canvas.onpointerup = function (event) {
    let pointer_type = get_pointer_type(event);
    call_client_function_f64_5(pointer_up_callback, event.clientX, event.clientY, pointer_type, event.button, event.timeStamp);
}

document.onkeydown = function (event) {
    pass_string_to_client(event.code);
    if (event.repeat) {
        call_client_function_f64(key_repeat_callback, event.timeStamp);
    } else {
        call_client_function_f64(key_down_callback, event.timeStamp);
    }

    // Perhaps these character received events should only be sent if text input has been enabled.

    // Ignore keys pressed while composing an IME character.
    // Also ignore keys that are longer than 1 character.
    // This is incorrect for some non-English key combos, but is an OK heuristic for now
    // to reject non-textual character inputs.
    // A more robust solution may watch a text field for changes instead.
    if (!event.is_composing && event.key.length == 1) {
        pass_string_to_client(event.key);
        call_client_function_f64(character_received_callback, event.timeStamp);
    }
}

document.onkeyup = function (event) {
    pass_string_to_client(event.code);
    call_client_function_f64(key_up_callback, event.timeStamp);
}

canvas.on_wheel_callback = function (event) {
    if (event.ctrl_key) {
        // This is a bit weird, but if a pinch gesture is performed
        // the ctrl modifier is set.
        // This is the simplest way to disambiguate it.

        // 0.02 is a completely arbitrary number to make this value more similar
        // to what native MacOS produces.
        // Is this a good idea at all?
        // Should this library even make such adjustments?
        // Is there a way to find an actual scale factor instead of a guess?

        call_client_function_f64_2(pinch_callback, -event.deltaY * 0.02, event.timeStamp);
    } else {
        call_client_function_f64_3(scroll_callback, -event.deltaX, -event.deltaY, event.timeStamp);
    }
}

function request_animation_frame_callback(event) {
    call_client_function(request_animation_frame_client_callback);
}

return {
    send_message_to_host: function (command, memory_buffer, data, data_length) {
        console.log("Receiving a message from the client");
        switch (command) {
            case 0:
                request_animation_frame_client_callback = data;
                window.requestAnimationFrame(request_animation_frame_callback)
                break;
            case 1:
                get_canvas_size(memory_buffer, data);
                break;
            case 2:
                // Set callbacks
                const callbacks = new Uint32Array(wasm_memory.buffer, data, data_length);
                pointer_moved_callback = callbacks[0];
                pointer_down_callback = callbacks[1];
                pointer_up_callback = callbacks[2];
                key_down_callback = callbacks[3];
                key_up_callback = callbacks[4];
                scroll_callback = callbacks[5];
                key_repeat_callback = callbacks[6];
                character_received_callback = callbacks[7];
                pinch_callback = callbacks[8];
                break;
        }
        return 0;
    },
};